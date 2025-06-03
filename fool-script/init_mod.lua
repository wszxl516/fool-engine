-- init_mod.lua
return {
    name = "init_mod",
    kind = "Core",
    state = {
        value = 0,
    },
    deps = { "core_mod" },
    init = function(state)
        print("init_mod init: value = " .. state.value)
    end,
    update = function(ctx)
        local self = ctx.self
        self.value = self.value + ctx.core_mod.counter
        -- ctx.core_mod.map.x =11
        print("init_mod access core_mod.counter = " .. ctx.core_mod.counter)
    end
}
