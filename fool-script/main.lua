
local init_test = require('test_module')
register_module(init_test)
print('init_test', init_test.state)

---@return number
---@diagnostic disable-next-line: lowercase-global
function main()
    return init_test.state.aa
end
