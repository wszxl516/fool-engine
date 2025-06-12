-- lua_thread.lua
return {
    name = "lua_thread",
    frames_interval = 1,
    shared_state = {
        counter = 0.0,
        position = { x = 200, y = -400 },
        orc_position = { x = 0, y = 0 },
        orc_force = {x = 0, y = 0},
        gravity = {x= 0, y = 40}
    },
    local_state = {
        counter = 0,
        position = { x = 0, y = 0 },
        orc_position = { x = 0, y = 0 }
    },
    deps = {},
    init = function(local_state)
        local Physics = require("Physics")
        local_state.phy =  Physics.new(1.0, 98)
        local_state.ball_hadle =  local_state.phy:add_body({
            user_data = 0,
            position = {x = 200, y = 100},
            shape = {
                Ball = {radius = 23}
            },
            restitution = 2.0
        })
        local_state.orc_hadle =  local_state.phy:add_body({
            user_data = 1,
            position = local_state.orc_position,
            shape = {
                Cuboid = {width = 32, height = 32}
            },
            restitution = 0.0,
        })
        local_state.phy:add_body({
            user_data = 0,
            position = {x = 0, y = 400},
            shape = {
                Cuboid = {width = 800, height = 10}
            },
            body_type = "Fixed",
            restitution = 0.0,
        })
        local_state.phy:add_body({
            user_data = 0,
            position = {x = 0, y = -400},
            shape = {
                Cuboid = {width = 800, height = 10}
            },
            body_type = "Fixed",
            restitution = 0.0,
        })
        local_state.phy:add_body({
            user_data = 0,
            position = {x = 400, y = 0},
            shape = {
                Cuboid = {width = 10, height = 800}
            },
            body_type = "Fixed",
            restitution = 0.0,
        })
        local_state.phy:add_body({
            user_data = 0,
            position = {x = -400, y = 0},
            shape = {
                Cuboid = {width = 10, height = 800}
            },
            body_type = "Fixed",
            restitution = 0.0,
        })
    end,
    update = function(ctx)
        local shared_state = ctx.shared_state
        local local_state = ctx.local_state
        if shared_state.orc_force.x ~= 0 or shared_state.orc_force.y ~= 0 then
            local_state.phy:apply_impulse(local_state.orc_hadle,shared_state.orc_force)
        end
        local_state.phy:set_gravity(shared_state.gravity.x,shared_state.gravity.y )
        local_state.phy:update()
        local body = local_state.phy:find_body(local_state.ball_hadle)
        shared_state.position = {x=body.pos.x, y= body.pos.y}
        shared_state.counter = shared_state.counter + 1

        local body = local_state.phy:find_body(local_state.orc_hadle)
        shared_state.orc_position = {x=body.pos.x, y= body.pos.y}
        if shared_state.counter > 100.0 then
            shared_state.counter = 0.0
        end
        
    end
}
