local inspect = require("engine.inspect")
---@class LOG
LOG = {name = "", date = false, location = false, level = 0}

---@param name string
---@param level string [Trace, Debug, Info, Warn, Error]
---@param date boolean
---@param location boolean
---@return LOG
function LOG:new(name, level, date, location)
    local Self = setmetatable({}, self)
    self.__index = self
    Self.name = name
    Self.date = date or false
    Self.location = location or false
    return Self
end

---@param fmt string
function LOG:trace(fmt, ...)
    self:log(fmt, "Trace", ...)
end
---@param fmt string
function LOG:debug(fmt, ...)
    self:log(fmt, "Debug", ...)
end

---@param fmt string
function LOG:info(fmt, ...)
    self:log(fmt, "Info", ...)
end

---@param fmt string
function LOG:warn(fmt, ...)
    self:log(fmt, "Warn", ...)
end

---@param fmt string
function LOG:error(fmt, ...)
    self:log(fmt, "Error", ...)
end

---@param fmt string
---@param level string
function LOG:log(fmt, level, ...)
    local args = {...}
    local len = select('#', ...)
    local formated = {}
    for ix = 1, len do
        formated[ix] = inspect(args[ix])
    end
    local msg = string.format(fmt, table.unpack(formated))
    -- local info = debug.getinfo(3, "Sl")
    local info = debug_info(3)
    local location_str = ""
    if self.location then
        location_str = string.format("[line: %d - func: %s]", info.line , info.func)
    end
    local full_msg = string.format("%s %s: %s", self.name, location_str, msg)
    if _G.__logger then
        _G.__logger(level, full_msg)
    else
        print(full_msg)
    end
end
return LOG