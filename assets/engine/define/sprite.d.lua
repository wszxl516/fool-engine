---@class Sprite
local Sprite = {}

---@param name string
---@param frames_id number[]
---@param fps number
---@return Animation
---@diagnostic disable-next-line: lowercase-global
function Sprite:create_animation(name, frames_id, fps)
    return {}
end

---@param name string
---@return Animation
---@diagnostic disable-next-line: lowercase-global
function Sprite:get_animation(name)
    return {}
end

---@return string[]
---@diagnostic disable-next-line: lowercase-global
function Sprite:list_animation()
    return {}
end

---@class Animation
local Animation = {}

---@return number
---@diagnostic disable-next-line: lowercase-global
function Animation:count()
    return 0
end

---@return number
---@diagnostic disable-next-line: lowercase-global
function Animation:current()
    return 0
end

---@diagnostic disable-next-line: lowercase-global
function Animation:next()
end

---@param position Point
---@diagnostic disable-next-line: lowercase-global
function Animation:draw(position)
end