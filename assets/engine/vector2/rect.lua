---@class Rect
---@field cx number
---@field cy number
---@field w number
---@field h number
local Rect = {}
Rect.__index = Rect

---@param cx number 
---@param cy number
---@param w number
---@param h number
---@return Rect
function Rect.new(cx, cy, w, h)
    return setmetatable({ cx = cx or 0, cy = cy or 0, w = w or 0, h = h or 0 }, Rect)
end

---@return Rect
function Rect:copy()
    return Rect.new(self.cx, self.cy, self.w, self.h)
end

---@return Bounds
function Rect:get_bounds()
    local hw = self.w / 2
    local hh = self.h / 2
    return {
        left   = self.cx - hw,
        right  = self.cx + hw,
        top    = self.cy + hh,
        bottom = self.cy - hh
    }
end

---@param px number
---@param py number
---@return boolean
function Rect:contains(px, py)
    local b = self:get_bounds()
    return px >= b.left and px <= b.right and py >= b.bottom and py <= b.top
end

function Rect.__tostring(self)
    return string.format("LuaRect(cx=%.2f, cy=%.2f, w=%.2f, h=%.2f)", self.cx, self.cy, self.w, self.h)
end

return Rect
