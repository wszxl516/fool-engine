---@class Point
---@field x number
---@field y number
local Point = {}
Point.__index = Point

---@param x number
---@param y number
---@return Point
function Point.new(x, y)
    return setmetatable({ x = x or 0, y = y or 0 }, Point)
end

---@return Point
function Point:copy()
    return Point.new(self.x, self.y)
end

---@return Point
function Point.__add(a, b)
    return Point.new(a.x + b.x, a.y + b.y)
end

---@return Point
function Point.__sub(a, b)
    return Point.new(a.x - b.x, a.y - b.y)
end

---@return Point
function Point.__unm(a)
    return Point.new(-a.x, -a.y)
end

---@return string
function Point.__tostring(a)
    return string.format("LuaPoint(x=%.2f, y=%.2f)", a.x, a.y)
end

return Point
