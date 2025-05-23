local KeyInput = {}
KeyInput.__index = KeyInput

---@param key string
---@param min number
function KeyInput.new(key, min)
    return setmetatable({
        key = key,
        min_trigger_time = min or 0.1,
        total_time = 0.0,

        is_down = false,
        time_down = 0.0,
        hold_threshold = 0.3,
    }, KeyInput)
end

---@param input Input
---@param dt number
---@return string "none" | "click" | "hold"
function KeyInput:update(input, dt)
    local event = "none"
    local pressed = input:mouse_pressed(self.key)
    local released = input:mouse_released(self.key)

    if pressed and not self.is_down then
        self.is_down = true
        self.time_down = 0.0
        self.total_time = 0.0
    end
    if self.is_down then
        self.time_down = self.time_down + dt
        self.total_time = self.total_time + dt
        if self.time_down >= self.hold_threshold and self.total_time >= self.min_trigger_time then
            self.total_time = 0.0
            event = "hold"
        end
    end

    if released and self.is_down then
        self.is_down = false
        if self.time_down < self.hold_threshold then
            event = "click"
        end
    end

    return event
end

return KeyInput
