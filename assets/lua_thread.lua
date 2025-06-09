-- lua_thread.lua
return {
    name = "lua_thread",
    kind = "Core",
    state = {
        counter = 0.0,
        map = { x = 1, y = 2 }
    },
    deps = {},
    init = function(state)
        state.counter = 0.0
        state.map = { x = 0, y = 0 }
    end,
    update = function(ctx)
        local self = ctx.self
        self.counter = self.counter + 1
        if self.counter > 100.0 then
            self.counter = 0.0
        end
        self.map.x = self.map.x + 1
        self.map.y = self.map.y + 1
    end
}
