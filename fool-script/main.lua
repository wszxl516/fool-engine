
local core_mod = require('core_mod')
local init_mod = require('init_mod')
local abc = require("a.b");
register_module(core_mod)
register_module(init_mod)

print('core_mod', core_mod.state)
print('init_mod', init_mod.state)
local mem_module = require("mem_module")
---@return number
---@diagnostic disable-next-line: lowercase-global
function run_frame()
    print(abc)
    abc.c.test()
    mem_module:main()
    print(core_mod)
    print(init_mod)
    return core_mod.state.counter
end
