-- core_mod.lua
return {
    name = "core_mod",
    kind = "Core",
    state = {
        counter = 0,
        map = { x =1, y =2}
    },
    deps = {"init_mod"},
    init = function(state)
        print("core_mod init: counter = " .. state.counter)
    end,
    update = function(ctx)
        local self = ctx.self
        self.counter = self.counter + 1
        print("core_mod update: counter = " , self.counter, ctx.init_mod.value)
    end
}
