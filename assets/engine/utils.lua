---@class utils
utils = {}
---@diagnostic disable-next-line: lowercase-global
---@param min_x number
---@param max_x number
---@param min_y number
---@param max_y number
---@return LuaPoint
function utils:rand_point(min_x, max_x, min_y, max_y)
    return { x = math.random(min_x, max_x), y = math.random(min_y, max_y) }
end

---@diagnostic disable-next-line: lowercase-global
---@return LuaPoint
function utils:rand_color()
    return { r = math.random(0, 255), g = math.random(0, 255), b = math.random(0, 255), a = math.random(0, 255) }
end

---comment
---@param orig any
---@return any
---@diagnostic disable-next-line: lowercase-global
function utils:deepcopy(orig)
    local orig_type = type(orig)
    local copy
    if orig_type == 'table' then
        copy = {}
        for orig_key, orig_value in next, orig, nil do
            copy[utils:deepcopy(orig_key)] = utils:deepcopy(orig_value)
        end
        setmetatable(copy, utils:deepcopy(getmetatable(orig)))
    else -- number, string, boolean, etc
        copy = orig
    end
    return copy
end
---@param first_table table
---@param second_table table
---@return any
function utils:merge_table(first_table, second_table)
    for k,v in pairs(second_table) do first_table[k] = v end
end
return utils