local size       = require("engine.vector2.size")
local rgba8      = require("engine.color.rgba8")
local point      = require("engine.vector2.point")
local lua_thread = require('lua_thread')
local shape      = {
    style = {
        translation = { 1.0, 0.0, 0.0, 1.0, 0.0, 0.0 },
        fill = { Color = rgba8.new(255, 0, 255, 100) },
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
            brush = { Color = rgba8.new(255, 0, 0, 255) },
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
    },
    player_sprite = nil,
    up_run_animation = nil,
    down_run_animation = nil,
    left_run_animation = nil,
    right_run_animation = nil,
    orc_last_pos = { x = 0, y = 0 },
    orc_last_direction = ""
}

---@param engine Engine
---@diagnostic disable-next-line: lowercase-global
function shape:init(engine)
    shape.player_sprite = engine.graphics:create_sprite("image/player.png", size.new(64, 64), 32)
    shape.up_run_animation = shape.player_sprite:create_animation("run_up", { 0, 1, 2, 3, 4, 5, 6, 7 }, 5)
    shape.down_run_animation = shape.player_sprite:create_animation("run_down", { 8, 9, 10, 11, 12, 13, 14, 15 }, 5)
    shape.left_run_animation = shape.player_sprite:create_animation("run_left", { 16, 17, 18, 19, 20, 21, 22, 23 }, 5)
    shape.right_run_animation = shape.player_sprite:create_animation("run_right", { 24, 25, 26, 27, 28, 29, 30, 31 }, 5)
    engine.audio:add_group("default", 0.0, true, nil)
    engine.audio:play("default", "audio/bgm.mp3", -10.0)
end

---@param engine Engine
---@param event Event
function shape:view(engine, event)
    engine.graphics:set_scale(0.9)
    local state = engine.audio:state("default", "audio/bgm.mp3")
    if state ~= nil and state ~= "Playing" then
        engine.audio:play("default", "audio/bgm.mp3", -10.0)
    end
    engine.graphics:draw_shape({
        style = self.style,
        drawable = {
            Ellipse = {
                center = point.new(lua_thread.shared_state.position.x, lua_thread.shared_state.position.y),
                radii = { x = 20, y = 20 },
                rotation = 0
            }
        },
        children = {
            {
                drawable = {
                    RoundedRect = {
                        p0 = point.new(0, 400),
                        size = size.new(800, 10),
                        radii = { top_left = 5, bottom_left = 5, bottom_right = 5, top_right = 5 },
                    }
                }
            },
            {
                drawable = {
                    RoundedRect = {
                        p0 = point.new(0, -400),
                        size = size.new(800, 10),
                        radii = { top_left = 5, bottom_left = 5, bottom_right = 5, top_right = 5 },
                    }
                }
            },
            {
                drawable = {
                    RoundedRect = {
                        p0 = point.new(400, 0),
                        size = size.new(10, 800),
                        radii = { top_left = 5, bottom_left = 5, bottom_right = 5, top_right = 5 },
                    }
                }
            },
            {
                drawable = {
                    RoundedRect = {
                        p0 = point.new(-400, 0),
                        size = size.new(10, 800),
                        radii = { top_left = 5, bottom_left = 5, bottom_right = 5, top_right = 5 },
                    }
                }
            },
            {
                style = self.style,
                drawable = {
                    Text = {
                        position = point.new(0, 0),
                        text = "Hello\nLua!"
                    }
                }
            },
            {
                drawable = {
                    Image = {
                        position = point.new(100, 100),
                        image = "image/linux.png"
                    }
                }
            }
        },
        apply_parent_style = true
    })

    lua_thread.shared_state.orc_force = { x = 0, y = 0 }
    if shape.orc_last_direction == "right" then
        shape.right_run_animation:draw(lua_thread.shared_state.orc_position)
        shape.right_run_animation:next()
    elseif shape.orc_last_direction == "left" then
        shape.left_run_animation:draw(lua_thread.shared_state.orc_position)
        shape.left_run_animation:next()
    else
        shape.up_run_animation:draw(lua_thread.shared_state.orc_position)
        shape.up_run_animation:next()
    end
    shape.orc_last_direction = ""
    if event:key_held("ArrowLeft") then
        shape.orc_last_direction = "left"
        lua_thread.shared_state.orc_force.x = -100
    end
    if event:key_held("ArrowRight") then
        shape.orc_last_direction = "right"
        lua_thread.shared_state.orc_force.x = 100
    end
    if event:key_held("ArrowUp") then
        local state = engine.audio:state("default", "audio/jump.mp3")
        if state == "Playing" then
            engine.audio:stop("default", "audio/jump.mp3", 1)
        end
        engine.audio:play("default", "audio/jump.mp3", -8.0)
        lua_thread.shared_state.orc_force.y = -400
    end
end

return shape
