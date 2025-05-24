---@class Bounds
---@field left number
---@field right number
---@field top number
---@field bottom number
---@class Size
local Size = {}
Size.__index = Size

---@param w number
---@param h number
---@return Size
function Size.new(w, h)
    return setmetatable({ w = w or 0, h = h or 0 }, Size)
end

---@return Size
function Size:copy()
    return Size.new(self.w, self.h)
end

---@return Bounds
function Size:get_bounds()
    local hw = self.w / 2
    local hh = self.h / 2
    return {
        left   = -hw,
        right  =  hw,
        top    =  hh,
        bottom = -hh,
    }
end

---@return string
function Size.__tostring(a)
    return string.format("LuaSize(w=%.2f, h=%.2f)", a.w, a.h)
end

return Size
