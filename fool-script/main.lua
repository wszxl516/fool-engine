
local init_test = require('test_module')
local abc = require("a.b");
print(init_test)
register_module(init_test)
print('init_test', init_test.state)
local mem_module = require("mem_module")
---@return number
---@diagnostic disable-next-line: lowercase-global
function main()
    print(abc)
    abc.c.test()
    mem_module:main()
    return init_test.state.aa
end
