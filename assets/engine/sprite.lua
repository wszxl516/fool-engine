---@class Sprite
---@field image Texture
---@field frame_w number
---@field frame_h number
---@field sheet_w number
---@field sheet_h number
---@field row number
---@field total_frames number
---@field speed number
---@field current_frame number
---@field tick number
---@field playing boolean
---@field debug boolean
---@field last_uv table
local Sprite = {}
Sprite.__index = Sprite
---@param image Texture
---@param frame_w number
---@param frame_h number
---@param sheet_w number
---@param sheet_h number
---@param row number
---@param total_frames number
---@param speed number
---@return Sprite
function Sprite.new(image, frame_w, frame_h, sheet_w, sheet_h, row, total_frames, speed)
    local self = setmetatable({}, Sprite)
    self.image = image
    self.frame_w = frame_w
    self.frame_h = frame_h
    self.sheet_w = sheet_w
    self.sheet_h = sheet_h
    self.row = row or 0
    self.total_frames = total_frames
    self.speed = speed or 5
    self.current_frame = 0
    self.tick = 0
    self.playing = true
    self.debug = false
    self.last_uv = { { 0, 0 }, { 0, 0 } } -- area_x, area_y
    return self
end

function Sprite:update()
    if not self.playing then return end
    self.tick = self.tick + 1
    if self.tick % self.speed == 0 then
        self.current_frame = (self.current_frame + 1) % self.total_frames

        local fx = self.frame_w / self.sheet_w
        local fy = self.frame_h / self.sheet_h

        local x1 = self.current_frame * fx
        local x2 = x1 + fx
        local y1 = self.row * fy
        local y2 = y1 + fy

        self.last_uv = { { x1, x2 }, { y1, y2 } }
    end
end

function Sprite:draw_debug(canvas, x, y, w, h)
    canvas:draw_rect(
        {
            position = { x = x, y = y, z = 0 },
            width = w,
            heigth = h,
            no_fill = true,
            stroke_color = { r = 255, g = 0, b = 0, a = 255 },
            color = { r = 255, g = 0, b = 0, a = 255 },
            stroke = {
                start_cap = "Square", end_cap = "Square", line_join = "Round", line_width = 1.0, miter_limit = 1.0, tolerance = 0.1 }
        }
    )
    local text = string.format("x: %d y: %d", math.floor(x), math.floor(y));
    canvas:draw_text(text,
        x, y + h / 1.5, nil, 12, { r = 100, g = 0, b = 0, a = 100 },
        {
            line_spacing = 0.0,
            line_wrap = "Whitespace",
            font_size = 12,
            x_align = "Middle",
            y_align = "Middle"
        })
end

---@param canvas Canvas
---@param x number
---@param y number
function Sprite:draw(canvas, x, y)
    canvas:draw_texture(
        self.image, x, y, self.frame_w, self.frame_h,
        {
            radians = { z = 0.0 },
            area_x = self.last_uv[1],
            area_y = self.last_uv[2],
        }
    )
    if self.debug then
        self:draw_debug(canvas, x, y, self.frame_w, self.frame_h)
    end
end

---@param canvas Canvas
---@param x number
---@param y number
---@param row number
---@param col number
function Sprite:draw_frame(canvas, x, y, row, col)
    local fx = self.frame_w / self.sheet_w
    local fy = self.frame_h / self.sheet_h
    local x1 = col * fx
    local x2 = x1 + fx
    local y1 = row * fy
    local y2 = y1 + fy
    canvas:draw_texture(
        self.image, x, y, self.frame_w, self.frame_h,
        {
            radians = { z = 0.0 },
            area_x = { x1, x2 },
            area_y = { y1, y2 },
        }
    )
    if self.debug then
        self:draw_debug(canvas, x, y, self.frame_w, self.frame_h)
    end
end

function Sprite:play()
    self.playing = true
end

function Sprite:pause()
    self.playing = false
end

function Sprite:reset()
    self.current_frame = 0
    self.tick = 0
end

return Sprite
