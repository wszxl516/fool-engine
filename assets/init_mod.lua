-- init_mod.lua
return {
    name = "init_mod",
    kind = "Core",
    state = {
        counter = 100,
    },
    deps = { "core_mod" },
    init = function(state)
        state.counter = 0
    end,
    update = function(ctx)
        local self = ctx.self
        self.counter = self.counter + ctx.core_mod.counter
    end
}
