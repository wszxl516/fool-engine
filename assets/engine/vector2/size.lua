---@class Size
---@field width number
---@field height number
local Size = {}
Size.__index = Size

---@param w number
---@param h number
---@return Size
function Size.new(w, h)
    return setmetatable({ width = w or 0, height = h or 0 }, Size)
end

---@return Size
function Size:copy()
    return Size.new(self.width, self.height)
end

---@return Bounds
function Size:get_bounds()
    local hw = self.width / 2
    local hh = self.height / 2
    return {
        left   = -hw,
        right  =  hw,
        top    =  hh,
        bottom = -hh,
    }
end

---@return string
function Size.__tostring(a)
    return string.format("LuaSize(w=%.2f, h=%.2f)", a.width, a.height)
end

return Size
