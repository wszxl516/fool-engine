-- core_mod.lua
return {
    name = "core_mod",
    kind = "Core",
    state = {
        counter = 100,
        map = { x =1, y =2}
    },
    deps = {"init_mod"},
    init = function(state)
        state.counter = 0
        state.map = { x =0, y =0}
    end,
    update = function(ctx)
        local self = ctx.self
        self.counter = self.counter + 1
        self.map.x = self.map.x + 1
        self.map.y = self.map.y + 1
    end
}
