local utils      = require("engine.utils")
local LOG        = require("engine.log")
local rgba8      = require("engine.color.rgba8")
local logger     = LOG.new("ui", true, true)
local lua_thread = require('lua_thread')

local ui_data    = {
    gravity_slider = {
        current = 40,
        min = 0,
        max = 100,
        label = "gravity"
    },
    volume_slider = {
        current = 50,
        min = 0,
        max = 100,
        label = "volume"
    },
    progress_bar = {
        progress = 0.0,
        name = "lua_thread",
        show_percentage = true,
        color = { r = 200, g = 0, b = 200, a = 200 },
        animate = true
    },
    font = nil
}
local UI         = {}
UI.__index       = UI
function UI:new()
    local self = setmetatable({}, UI)
    self.data = utils:deepcopy(ui_data)
    return self
end

---@param engine Engine
---@diagnostic disable-next-line: lowercase-global
function UI:view(engine)
    engine.ui_ctx:draw_window({
        title = "Gravity",
        collapsible = true,
        constrain = true,
        default_open = true,
        drag_to_scroll = true,
        resizable = true,
        title_bar = true,
        movable = true,
        x = -100.0,
        y = 00.0,
        w = 200.0,
        h = 400.0,
        font_name = "fonts/SarasaTermSCNerd-Regular.ttf",
        frame = {
            inner_margin = { left = 5, right = 5, top = 5, bottom = 5 },
            outer_margin = { left = 1, right = 1, top = 1, bottom = 1 },
            rounding = { nw = 5, ne = 5, sw = 5, se = 5 },
            shadow = { offset = { 1, 2 }, blur = 1, spread = 1, color = rgba8.new(0, 0, 0, 0) },
            fill = rgba8.new(0, 0, 0, 0),
            stroke_width = 1,
            stroke_color = rgba8.new(50, 50, 50, 50)
        },
        -- bg_img = "image/linux.png",
        bg_img_color = rgba8.new(100, 100, 100, 50)
    }, function(ui)
        gui_run(engine.audio, ui)
    end)
end

---@param engine Engine
---@diagnostic disable-next-line: lowercase-global
function UI:init(engine)
    local save = engine.save:load("aaa")
    if save then
        ui_data.gravity_slider.current = save.data.gravity
        ui_data.volume_slider.current = save.data.volume
        engine.audio:set_volume_all(ui_data.volume_slider.current - 50.0, 100)
    end
end

---@param audio Audio
---@diagnostic disable-next-line: lowercase-global
function gui_run(audio,ui)
    ui:set_row_height(20)
    local sl = ui:slider(ui_data.gravity_slider)
    if sl:changed() then
        lua_thread.shared_state.gravity.y = ui_data.gravity_slider.current
    end
    local sl = ui:slider(ui_data.volume_slider)
    if sl:changed() then
        audio:set_volume_all(ui_data.volume_slider.current - 50.0, 100)
    end
    ui:progress_bar(ui_data.progress_bar)
    ui_data.progress_bar.progress = lua_thread.shared_state.counter / 100.0
end

---@param engine Engine
---@param event Event
---@param dt number -- delay time
---@diagnostic disable-next-line: lowercase-global
function UI:exit(engine, event, dt)
    engine.audio:pause_all(0)
    engine.ui_ctx:draw_window({
        title = "Are you sure to exit?",
        collapsible = false,
        constrain = false,
        default_open = true,
        drag_to_scroll = false,
        resizable = false,
        title_bar = true,
        movable = false,
        x = 0.0,
        y = -100.0,
        w = 200.0,
        h = 50.0,
        font_name = "fonts/SarasaTermSCNerd-Regular.ttf",
        frame = {
            inner_margin = { left = 5, right = 5, top = 5, bottom = 5 },
            outer_margin = { left = 1, right = 1, top = 1, bottom = 1 },
            rounding = { nw = 5, ne = 5, sw = 5, se = 5 },
            shadow = { offset = { 1, 2 }, blur = 1, spread = 1, color = rgba8.new(0, 0, 0, 0) },
            fill = rgba8.new(0, 0, 0, 0),
            stroke_width = 1,
            stroke_color = rgba8.new(50, 50, 50, 50)
        },
        bg_img_color = rgba8.new(100, 100, 100, 50)
    }, function(ui)
        ui:empty_space(0, 30)
        ui:horizontal(function(ctx)
            if ctx:button("Yes"):clicked() then
                engine.save:save("aaa", {gravity=ui_data.gravity_slider.current, volume= ui_data.volume_slider.current})
                engine.window:exit()
            end
            ctx:empty_space(150, 0)
            if ctx:button("No"):clicked() then
                engine:set_running()
                engine.audio:resume_all(0)
            end
        end)
    end)
end

return UI
