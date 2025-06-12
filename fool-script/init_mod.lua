-- init_mod.lua
local local_state = { value = 0 }
---@diagnostic disable-next-line: lowercase-global
function update(ctx)
    local shared_state = ctx.shared_state
    shared_state.value = shared_state.value + ctx.core_mod.counter
    print("init_mod access core_mod.counter = " .. ctx.core_mod.counter)
end

---@diagnostic disable-next-line: lowercase-global
function init()
    print("init_mod init: value = " .. local_state.value)
end

return {
    name = "init_mod",
    frames_interval = 10,
    shared_state = {
        value = 0,
    },
    deps = { "core_mod" },
    init = init,
    update = update
}
