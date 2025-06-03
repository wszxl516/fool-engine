---@class Color8
---@field r integer
---@field g integer
---@field b integer
---@field a integer
local Color8 = {}
Color8.__index = Color8

---@param r integer
---@param g integer
---@param b integer
---@param a integer?
---@return Color8
function Color8.new(r, g, b, a)
    local self = setmetatable({}, Color8)
    self.r = math.max(0, math.min(255, r or 0))
    self.g = math.max(0, math.min(255, g or 0))
    self.b = math.max(0, math.min(255, b or 0))
    self.a = math.max(0, math.min(255, a or 255))
    return self
end

---@return Color8
function Color8:clone()
    return Color8.new(self.r, self.g, self.b, self.a)
end

---@return table<string, number>
function Color8:to_float()
    return {
        r = self.r / 255,
        g = self.g / 255,
        b = self.b / 255,
        a = self.a / 255,
    }
end

---@return table
function Color8:to_brush()
    return { Solid = { components = { self.r, self.g, self.b, self.a } } }
end

---@return string
function Color8:__tostring()
    return string.format("rgba(%d,%d,%d,%d)", self.r, self.g, self.b, self.a)
end

---@param other Color8
---@return Color8
function Color8:__add(other)
    return Color8.new(
        math.min(self.r + other.r, 255),
        math.min(self.g + other.g, 255),
        math.min(self.b + other.b, 255),
        math.min(self.a + other.a, 255)
    )
end

---@param scalar number
---@return Color8
function Color8:__mul(scalar)
    return Color8.new(
        math.floor(self.r * scalar),
        math.floor(self.g * scalar),
        math.floor(self.b * scalar),
        math.floor(self.a * scalar)
    )
end

---@param scalar number
---@param self Color8
---@return Color8
function Color8.__rmul(scalar, self)
    return self * scalar
end

return setmetatable(Color8, {
    __call = function(_, ...) return Color8.new(...) end
})
