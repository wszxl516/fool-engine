local utils      = require("engine.utils")
local LOG        = require("engine.log")
local rgba8      = require("engine.color.rgba8")
local logger     = LOG.new("ui", true, true)
local lua_thread = require('lua_thread')

local ui_data    = {
    combo_box = { selected = "aa", id = "combo_box", items = { "bb", "aa", "cc" } },
    radio = { { selected = false, text = "aaa" }, { selected = true, text = "bbb" } },
    text_edit = {
        id = "id_text",
        content = "default",
        single_line = false,
        code_editor = false,
        char_limit = 256,
        clip_text = false,
        rows = 1,
        password = false,
    },
    checkbox = {
        checked = false,
        label = "checkbox"
    },
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
    color_picker = { r = 100, g = 0, b = 100, a = 100 },
    label_text = "empty",
    linux_texture = nil,
    gear_texture = nil,
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
        title = "test windows",
        collapsible = false,
        constrain = false,
        default_open = true,
        drag_to_scroll = false,
        resizable = false,
        title_bar = false,
        movable = true,
        x = 0.0,
        y = 0.0,
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
        bg_img = "image/linux.png",
        bg_img_color = rgba8.new(100, 100, 100, 50)
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
    ui:with_layout(true, function(image_ui)
        local btn = image_ui:image_button(
            {
                img = "image/linux.png",
                tint = rgba8.new(50, 50, 50, 30),
                img_bg_fill = rgba8.new(0, 0, 0, 50),
                scale = 0.5,
                -- uv = {min = {x= 0.1, y = 0.1}, max = {x= 0.9, y = 0.9}},
                img_rotate = { angle = 3.14, origin = { x = 0.5, y = 0.5 } },
                sense = "CLICK",
                corner_radius = { sw = 32, se = 32, nw = 32, ne = 32 },
                frame = false,
            })
        if btn:clicked() then
            logger:info("image_button clicked")
        end
        image_ui:image(
            {
                img = "image/linux.png",
                tint = rgba8.new(50, 50, 50, 50),
                img_bg_fill = rgba8.new(10, 10, 10, 50),
                scale = 0.5,
                uv = { min = { x = 0.1, y = 0.1 }, max = { x = 0.9, y = 0.9 } },
                img_rotate = { angle = 3.14, origin = { x = 0.5, y = 0.5 } },
                sense = "CLICK",
                corner_radius = { sw = 32, se = 32, nw = 32, ne = 32 },
                frame = false,
            })
    end)

    ui:grid("111", { w = 10, h = 10 }, 0, function(grid_ui)
        grid_ui:with_layout(true, function(center_ui)
            center_ui:collapsing(data.label_text, function(collapsing_ui)
                collapsing_ui:label("啊!")
            end)
            center_ui:separator()
            local res = center_ui:hyperlink("https://example.com")
            if res:clicked() then
                logger:info("clicked hyperlink")
            end
            local res = center_ui:button("Click Me")
            if res:clicked() then
                logger:info("clicked button")
            end
            local resp = center_ui:checkbox(data.checkbox)
            if resp:changed() then
                logger:info("checkbox changed %s", data.checkbox.checked)
            end
            center_ui:radio(data.radio, false)
            center_ui:end_row()
            center_ui:separator()
            center_ui:combo_box(data.combo_box)
            center_ui:heading("heading")
            center_ui:small("small")
            local selectable_label = center_ui:selectable_label(true, "Color Picker")
            if selectable_label:double_clicked() then
                logger:info("clicked selectable_label")
            end
            local r = center_ui:color_picker(data.color_picker)
            center_ui:end_row()
            center_ui:vertical(function(vertical_ui)
                local sl = vertical_ui:slider(data.slider)
                if sl:changed() then
                    data.progress_bar.progress = data.slider.current / 100.0
                end
            end)
            center_ui:horizontal(function(vertical_ui)
                vertical_ui:progress_bar(data.slider.current)
            end)
            center_ui:end_row()
            data.progress_bar.progress = lua_thread.shared_state.counter / 100.0
            center_ui:with_layout(true, function(vertical_ui)
                vertical_ui:progress_bar(data.progress_bar)
            end)
            center_ui:end_row()
            local resp = center_ui:text_edit(data.text_edit)
            if resp:changed() then
                data.label_text = data.text_edit.content
            end
        end)
    end)
end

return UI
