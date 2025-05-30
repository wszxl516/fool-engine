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

---@param ui_context EguiContext
---@diagnostic disable-next-line: lowercase-global
function view(ui_context)
    ui:view(ui_context, nil)
end

---@param dt number -- delay time
---@diagnostic disable-next-line: lowercase-global
function update(dt)
    graphics:update(ui.data.slider.current)
end

---@diagnostic disable-next-line: lowercase-global
function init()
    ui:init()
    graphics:init()
end

---@param dt number -- delay time
---@param event Event
---@diagnostic disable-next-line: lowercase-global
function event(event, dt)
    graphics:event(event, dt)
    event:on_exit(function()
        logger:debug("exit at dt %s", dt)
        return false
    end)
    -- event:exit()
end
