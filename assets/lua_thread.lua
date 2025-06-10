-- lua_thread.lua
return {
    name = "lua_thread",
    frames_interval = 2,
    shared_state = {
        counter = 0.0,
        position = { x = 200, y = -400 }
    },
    local_state = {
        counter = 0,
        position = { x = 0, y = 0 }
    },
    deps = {},
    init = function(local_state)
        local Physics = require("Physics")
        local_state.phy =  Physics.new(1.0, 98)
        local_state.ball_hadle =  local_state.phy:add_body({
            user_data = 0,
            position = {x = 200, y = -400},
            shape = {
                Ball = {radius = 20}
            },
            restitution = 0.8
        })
        local_state.phy:add_body({
            user_data = 0,
            position = {x = 0, y = 400},
            shape = {
                Cuboid = {width = 800, height = 10}
            },
            body_type = "Fixed"
        })
    end,
    update = function(ctx)
        local shared_state = ctx.shared_state
        local local_state = ctx.local_state
        local_state.phy:update()
        local body = local_state.phy:find_body(local_state.ball_hadle)
        shared_state.position = {x=body.pos.x, y= body.pos.y}
        shared_state.counter = shared_state.counter + 1
        if shared_state.counter > 100.0 then
            shared_state.counter = 0.0
        end
    end
}
