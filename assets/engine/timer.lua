---@class TimerTask
---@field id integer
---@field delay number
---@field interval number?
---@field func fun()
---@field time number
---@field tag string?
---@field frame number?
---@field repeatable boolean

---@class Timer
---@field _tasks table<integer, TimerTask>
---@field _next_id integer
---@field _paused boolean
---@field _time number
---@field _frame integer
---@field _scale number
local Timer = {}
Timer.__index = Timer

function Timer.new()
    local self = setmetatable({}, Timer)
    self._tasks = {}
    self._next_id = 1
    self._paused = false
    self._time = 0
    self._frame = 0
    self._scale = 1.0
    return self
end

---@param delay number
---@param func fun()
---@param tag string?
---@return integer id
function Timer:after(delay, func, tag)
    return self:_add(delay, func, false, nil, tag)
end

---@param interval number
---@param func fun()
---@param tag string?
---@return integer id
function Timer:every(interval, func, tag)
    return self:_add(interval, func, true, nil, tag)
end

---@param frames integer
---@param func fun()
---@param tag string?
---@return integer id
function Timer:delay_frame(frames, func, tag)
    return self:_add(0, func, false, self._frame + frames, tag)
end

function Timer:_add(delay, func, repeatable, frame_target, tag)
    local id = self._next_id
    self._next_id = id + 1
    self._tasks[id] = {
        id = id,
        delay = delay,
        interval = repeatable and delay or nil,
        func = func,
        time = self._time + delay,
        frame = frame_target,
        repeatable = repeatable,
        tag = tag,
    }
    return id
end

---@param dt number
function Timer:update(dt)
    if self._paused then return end
    self._frame = self._frame + 1
    self._time = self._time + dt * self._scale

    for id, task in pairs(self._tasks) do
        local due = task.frame and self._frame >= task.frame or self._time >= task.time
        if due then
            local ok, err = pcall(task.func)
            if not ok then print("[Timer Error]", err) end

            if task.repeatable then
                task.time = self._time + task.interval
                task.frame = nil
            else
                self._tasks[id] = nil
            end
        end
    end
end

function Timer:pause()
    self._paused = true
end

function Timer:resume()
    self._paused = false
end

---@param id integer
function Timer:cancel_id(id)
    self._tasks[id] = nil
end

---@param fn fun()
function Timer:cancel_fn(fn)
    for id, task in pairs(self._tasks) do
        if task.func == fn then
            self._tasks[id] = nil
        end
    end
end

---@param tag string
function Timer:cancel_tag(tag)
    for id, task in pairs(self._tasks) do
        if task.tag == tag then
            self._tasks[id] = nil
        end
    end
end

function Timer:clear_all()
    self._tasks = {}
end

function Timer:set_time_scale(scale)
    self._scale = scale
end

function Timer:get_time()
    return self._time
end

function Timer:get_frame()
    return self._frame
end

return Timer
