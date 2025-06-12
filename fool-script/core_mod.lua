-- core_mod.lua
local local_state = { counter = 0 }
---@diagnostic disable-next-line: lowercase-global
function init()
    local ab = require("a.b");
    local_state.ab = ab
    print("core_mod init: counter = " .. local_state.counter)
end


---@diagnostic disable-next-line: lowercase-global
function update(ctx)
    local shared_state = ctx.shared_state
    local_state.ab.c.test()
    shared_state.counter = shared_state.counter + 1
    print("core_mod update: counter = ", shared_state.counter, ctx.init_mod.value)
end

return {
    name = "core_mod",
    frames_interval = 10,
    shared_state = {
        counter = 0,
        map = { x = 0, y = 0 }
    },
    deps = { "init_mod" },
    init = init,
    update = update
}
