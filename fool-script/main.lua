
local core_mod = require('core_mod')
local init_mod = require('init_mod')
local abc = require("a.b");
register_threaded_module(core_mod)
register_threaded_module(init_mod)

print('core_mod', core_mod.shared_state)
print('init_mod', init_mod.shared_state)
local mem_module = require("mem_module")
---@return number
---@diagnostic disable-next-line: lowercase-global
function main()
    -- print(abc)
    -- abc.c.test()
    print(core_mod.shared_state)
    print(init_mod.shared_state)
    return core_mod.shared_state.counter
end
