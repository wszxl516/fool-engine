local utils      = require("engine.utils")
local LOG        = require("engine.log")
local rgba8      = require("engine.color.rgba8")
local logger     = LOG.new("ui", true, true)
local lua_thread = require('lua_thread')

local ui_data    = {
    slider = {
        current = 40,
        min = 0,
        max = 100,
        label = "gravity"
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
    engine:draw_window({
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
        -- frame = {
        --     inner_margin = { left = 5, right = 5, top = 5, bottom = 5 },
        --     outer_margin = { left = 1, right = 1, top = 1, bottom = 1 },
        --     rounding = { nw = 5, ne = 5, sw = 5, se = 5 },
        --     shadow = { offset = { 1, 2 }, blur = 1, spread = 1, color = rgba8.new(0, 0, 0, 0) },
        --     fill = rgba8.new(0, 0, 0, 0),
        --     stroke_width = 1,
        --     stroke_color = rgba8.new(50, 50, 50, 50)
        -- },
        -- bg_img = "image/linux.png",
        -- bg_img_color = rgba8.new(100, 100, 100, 50)
    }, function(ui)
        gui_run(self.data, ui)
    end)
end

---@diagnostic disable-next-line: lowercase-global
function UI:init()
end

---@diagnostic disable-next-line: lowercase-global
function gui_run(data, ui)
    ui:set_row_height(20)
    local sl = ui:slider(data.slider)
    if sl:changed() then
        lua_thread.shared_state.gravity.y = data.slider.current
    end
    ui:progress_bar(data.progress_bar)
    data.progress_bar.progress = lua_thread.shared_state.counter / 100.0
end

return UI
