-- init_mod.lua
return {
    name = "init_mod",
    frames_interval = 10,
    shared_state = {
        value = 0,
    },
    deps = { "core_mod" },
    local_state = { value = 0},
    init = function(local_state)
        print("init_mod init: value = " .. local_state.value)
    end,
    update = function(ctx)
        local shared_state = ctx.shared_state
        shared_state.value = shared_state.value + ctx.core_mod.counter
        print("init_mod access core_mod.counter = " .. ctx.core_mod.counter)
    end
}
