---@class Transform
---@field x number
---@field y number
---@field sx number
---@field sy number
---@field rotation number
local Transform = {}
Transform.__index = Transform

---@param x number
---@param y number
---@param sx number
---@param sy number
---@param rotation number
---@return Transform
function Transform.new(x, y, sx, sy, rotation)
    return setmetatable({
        x = x or 0,
        y = y or 0,
        sx = sx or 1,
        sy = sy or 1,
        rotation = rotation or 0
    }, Transform)
end

---@return Transform
function Transform:copy()
    return Transform.new(self.x, self.y, self.sx, self.sy, self.rotation)
end

---@param x number
---@param y number
function Transform:set_position(x, y)
    self.x = x
    self.y = y
end

---@param sx number
---@param sy number
function Transform:set_scale(sx, sy)
    self.sx = sx
    self.sy = sy
end

---@param angle number
function Transform:set_rotation(angle)
    self.rotation = angle
end

---@param px number
---@param py number
---@return number, number
function Transform:apply(px, py)
    local x = px * self.sx
    local y = py * self.sy
    local cos_a = math.cos(self.rotation)
    local sin_a = math.sin(self.rotation)
    local rx = x * cos_a - y * sin_a + self.x
    local ry = x * sin_a + y * cos_a + self.y
    return rx, ry
end

function Transform.__tostring(self)
    return string.format("LuaTransform(pos=%.2f,%.2f scale=%.2f,%.2f rot=%.2frad)",
        self.x, self.y, self.sx, self.sy, self.rotation)
end

return Transform
