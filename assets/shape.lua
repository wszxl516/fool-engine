local size  = require("engine.vector2.size")
local rgba8 = require("engine.color.rgba8")
local point = require("engine.vector2.point")

local shape = {
    style = {
        translation = { 1.0, 0.0, 0.0, 1.0, 0.0, 0.0 },
        fill = rgba8.new(255, 100, 0, 100):to_brush(),
        fill_rule = "NonZero",
        stoke = {
            stoke =
            {
                width = 1.0,
                join = "Round",
                miter_limit = 1.0,
                start_cap = "Round",
                end_cap = "Round",
                dash_pattern = { 0, 0, 0, 0 },
                dash_offset = 1.0
            },
            brush = rgba8.new(255, 100, 0, 100):to_brush(),
        },
        opacity = 0.8,
        visible = true,
        z_index = 0,
        tag = "test",
        font = "fonts/SarasaTermSCNerd-Regular.ttf",
        font_size = 22,
        hint = false,
        align = "Center",
        line_spacing = 5,
        vertical = false
    }
}

---@param window Window
---@diagnostic disable-next-line: lowercase-global
function shape:init(window)

end

---@param engine Engine
function shape:view(engine)
    engine:draw_shape({
        style = self.style,
        drawable = {
            Ellipse = {
                center = point.new(100, 100),
                radii = { x = 50, y = 20 },
                rotation = 0
            }
        },
        children = {
            {
                -- style = self.style,
                drawable = {
                    RoundedRect = {
                        p0 = point.new(0, 0),
                        size = size.new(100, 100),
                        radii = { top_left = 5, bottom_left = 5, bottom_right = 5, top_right = 5 },
                    }
                }
            },
            {
                style = self.style,
                drawable = {
                    Text = {
                        position = point.new(0, 0),
                        text = "Lua!\n你好"
                    }
                }
            }
        },
        apply_parent_style = true
    })
end

return shape
