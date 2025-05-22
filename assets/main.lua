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
end

---@diagnostic disable-next-line: lowercase-global
function update()
    graphics:update(ui.data.slider.current)
end

---@diagnostic disable-next-line: lowercase-global
function init()
    ui:init()
    graphics:init()
end

---@diagnostic disable-next-line: lowercase-global
function event(input)
    graphics:event(input)
end
