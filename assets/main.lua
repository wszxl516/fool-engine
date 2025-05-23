local graphics = require("graphics")
local UI = require("ui")
local ui = UI:new()
---@param canvas Canvas
---@param ui_context EguiContext
---@param window Window
---@diagnostic disable-next-line: lowercase-global
function view(canvas, ui_context, window)
    ui:view(ui_context, window)
    graphics:draw(canvas)
    window:set_ime_allowed(true)
    window:set_ime_position({x = 100, y = 100})

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
---@param input Input
---@diagnostic disable-next-line: lowercase-global
function event(input, dt)
    graphics:event(input, dt)
end
