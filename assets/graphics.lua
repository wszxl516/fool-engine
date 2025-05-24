local utils = require("engine.utils")
local LOG = require("engine.log")
local logger = LOG:new("graphics", "debug", true, true)
local Sprite = require("engine.sprite")
local graphics = {}
local global_physics = physics_init(0, 900)
local key_event = require("engine.input.key")
local side_attr = {
    body_attr = {
        body_type = "Fixed",
        linear_damping = 0.0,
        angular_damping = 0.0,
        gravity_scale = 0.0,
        additional_mass = 0.0,
        mass = 0.0,
        can_sleep = false,
        sleeping = false,
        restitution = 0.0,
        friction = 0.0,
        density = 0.0,
        is_sensor = false,
        active_events = "",
        active_hooks = ""
    },
    physics = {
        {
            user_data = 1,
            position = { x = 0.0, y = -400.0 },
            shape = {
                Cuboid = {
                    width = 800.0,
                    height = 10.0,
                }
            }
        },
        {
            user_data = 2,
            position = { x = 400.0, y = 0.0 },
            shape = {
                Cuboid = {
                    width = 10.0,
                    height = 800.0,
                }
            }
        },
        {
            user_data = 3,
            position = { x = 0.0, y = 400.0 },
            shape = {
                Cuboid = {
                    width = 800.0,
                    height = 10.0,
                }
            }
        },
        {
            user_data = 4,
            position = { x = -400.0, y = 0.0 },
            shape = {
                Cuboid = {
                    width = 10.0,
                    height = 800.0,
                }
            }
        }
    },
    shape_attr = {
        no_fill = false,
        stroke_color = { r = 255, g = 255, b = 255, a = 255 },
        color = { r = 255, g = 255, b = 255, a = 255 },
        stroke = { start_cap = "Square", end_cap = "Square", line_join = "Round", line_width = 1.0, miter_limit = 1.0, tolerance = 0.1 }
    },
    physics_handle = {}
}
local player_attr = {
    sprite = Sprite,
    physics = {
        x = 100.0,
        y = -345.0,
        handle = nil,
        config = {
            user_data = 0,
            position = { x = 100.0, y = -345.0 },
            shape = {
                Cuboid = {
                    width = 80.0,
                    height = 110.0,
                }
            },
            body_type = "Dynamic",
            rotation = 0.0,
            linear_damping = 0.0,
            angular_damping = 0.0,
            gravity_scale = 1.0,
            additional_mass = 1.0,
            mass = 1.0,
            can_sleep = false,
            sleeping = false,
            restitution = 0.2,
            friction = 0.8,
            density = 1.0,
            is_sensor = false,
            active_events = "all",
            active_hooks = "all"
        },
        key_event = { { { x = 0, y = -200 }, key_event.new("Up", 0.1) },
            { { x = 0, y = 200 },  key_event.new("Down", 0.1) },
            { { x = -50, y = 0 },  key_event.new("Left", 0.1) },
            { { x = 50, y = 0 },   key_event.new("Right", 0.1) } },
        key_delay = 0
    },
    texture = nil
}
---@diagnostic disable-next-line: lowercase-global
function graphics:init()
    player_attr.texture = ResourceManager:load_texture("player.png")
    player_attr.sprite = Sprite.new(player_attr.texture, 80.0, 110.0, 720.0, 330.0, 1.0, 9.0, 10.0, 0.0)
    player_attr.physics.handle = global_physics:add_body(player_attr.physics.config)
    for index, value in ipairs(side_attr.physics) do
        local args = utils:deepcopy(value)
        utils:merge_table(args, side_attr.body_attr)
        side_attr.physics_handle[index] = global_physics:add_body(args)
    end
    player_attr.sprite.debug = true
end

function graphics:update(value)
    global_physics:set_gravity(0, value * 8)
    global_physics:update()
    local body = global_physics:find_body(player_attr.physics.handle)
    if body then
        player_attr.physics.x = body.pos.x
        player_attr.physics.y = body.pos.y
        if math.floor(math.abs(body.linvel.x)) == 0 then
            player_attr.sprite:pause()
        else
            player_attr.sprite:play()
        end
    end
    global_physics:event_update()
    player_attr.sprite:update()
end

---@param canvas Canvas
function graphics:draw(canvas)
    player_attr.sprite:draw(canvas, player_attr.physics.x, player_attr.physics.y)
    for _key, value in pairs(side_attr.physics) do
        local shape = utils:deepcopy(side_attr.shape_attr)
        utils:merge_table(shape, side_attr.shape_attr)
        shape["position"] = { x = value.position.x, y = value.position.y }
        shape["width"] = value.shape.Cuboid.width
        shape["height"] = value.shape.Cuboid.height
        canvas:draw_rect(shape)
    end
    player_attr.sprite:draw_frame(canvas, -350, 340, 1, 1)
end

---@param dt number -- delay time
---@param event Event
function graphics:event(event, dt)
    -- Left, Up, Right, Down
    for key, value in pairs(player_attr.physics.key_event) do
        local result = value[2]:update(event, dt)
        if result == "hold" then
            logger:error("%s", {aa = 11, bb = "bb", cc = init, dd = false, ee = nil})
            global_physics:apply_impulse(player_attr.physics.handle, value[1])
        end
    end
end

return graphics
