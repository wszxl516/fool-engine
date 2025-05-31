local graphics = require("graphics")
local UI = require("ui")
local point = require("engine.vector2.point")
local ui = UI:new()
local LOG = require("engine.log")
local rgba8 = require("engine.color.rgba8")
local logger = LOG.new("main", true, true)
-- ---@param canvas Canvas
-- ---@param ui_context EguiContext
-- ---@param window Window
-- ---@diagnostic disable-next-line: lowercase-global
-- function view(canvas, ui_context, window)
--     ui:view(ui_context, window)
--     graphics:draw(canvas)
--     window:set_ime_allowed(true)
--     window:set_ime_position(point.new(100,100))
--     canvas:draw_text("OK!", point.new(-100, 100), nil, nil, rgba8.new(100,100,0,100), {})
-- end

---@param window Window
---@param ui_context EguiContext
---@diagnostic disable-next-line: lowercase-global
function view(window, ui_context)
    ui:view(ui_context)
    -- window:set_cursor_grab("None")
    -- window:set_cursor("move")
    -- window:set_cursor_visible(false)
    window:set_cursor_icon("linux.png")
end

---@param dt number -- delay time
---@diagnostic disable-next-line: lowercase-global
function update(dt)
    graphics:update(ui.data.slider.current)
end

---@param window Window
---@param ui_context EguiContext
---@diagnostic disable-next-line: lowercase-global
function init(window, ui_context)
    ui_context:set_font("SarasaTermSCNerd-Regular.ttf")
    window:set_title("aaaaaaaa")
    window:set_fullscreen(false)
    window:load_cursor_icon("linux.png")
    window:set_window_icon("linux.png")
    ui_context:set_style({
        text = {
            Heading = 22.0,
            Body = 18.0,
            Button = 18.0,
            Small = 16.0,
            Monospace = 18.0
        },
        dark = true,
        animation_time = 0.2,
        wrap = "Extend",
        noninteractive_fg_color = rgba8.new(255, 0, 0, 0),
        hovered_fg_color = rgba8.new(255, 255, 255, 0),
        active_fg_color = rgba8.new(0, 0, 0, 0),
        inactive_fg_color = rgba8.new(200, 200, 200, 200),
        open_fg_color = rgba8.new(200, 0, 0, 0)
    })
    print(window:monitor())
    -- ui:init()
    -- graphics:init()
end

---@param dt number -- delay time
---@param event Event
---@param window Window
---@diagnostic disable-next-line: lowercase-global
function event(event, window, dt)
    graphics:event(event, dt)
    if event:key_pressed("Escape") then
        print("Escape")
        window:exit()
    end
    event:on_exit(function()
        logger:debug("exit from lua")
    end)
end
