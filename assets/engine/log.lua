local inspect = require("engine.inspect")

---@class LOG
---@field name string
---@field date boolean
---@field location boolean
local LOG = {}
LOG.__index = LOG

---@param name string
---@param date boolean
---@param location boolean
---@return LOG
function LOG.new(name, date, location)
    local self = setmetatable({}, LOG)
    self.name = name or ""
    self.date = date or false
    self.location = location or false
    return self
end

function LOG:trace(fmt, ...)
    self:log("Trace", fmt, ...)
end

function LOG:debug(fmt, ...)
    self:log("Debug", fmt, ...)
end

function LOG:info(fmt, ...)
    self:log("Info", fmt, ...)
end

function LOG:warn(fmt, ...)
    self:log("Warn", fmt, ...)
end

function LOG:error(fmt, ...)
    self:log("Error", fmt, ...)
end

--- Formats and prints the log message
---@param level string
---@param fmt string
function LOG:log(level, fmt, ...)
    local args = {...}
    local formated = {}
    for i = 1, select("#", ...) do
        formated[i] = inspect(args[i])
    end
    local msg = string.format(fmt, table.unpack(formated))

    local location_str = ""
    if self.location then
        local info = debug_info(3)
        location_str = string.format("[%s:%d:%s]", info.file, info.line, info.func)
    end

    local date_str = ""
    if self.date then
        date_str = os.date("[%Y-%m-%d %H:%M:%S]") .. " "
    end

    local full_msg = string.format("%s%s %s: %s", date_str, location_str, self.name, msg)
    __logger(level, full_msg)
end

return LOG