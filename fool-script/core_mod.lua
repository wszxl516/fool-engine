-- core_mod.lua
return {
    name = "core_mod",
    frames_interval = 10,
    shared_state = {
        counter = 0,
        map = { x =0, y =0}
    },
    deps = {"init_mod"},
    local_state = {counter = 0},
    init = function(local_state)
        local ab = require("a.b");
        local_state.ab = ab
        print("core_mod init: counter = " .. local_state.counter)
    end,
    update = function(ctx)
        local local_state = ctx.local_state
        local_state.ab.c.test()
        local shared_state = ctx.shared_state
        shared_state.counter = shared_state.counter + 1
        print("core_mod update: counter = " , shared_state.counter, ctx.init_mod.value)
    end
}
