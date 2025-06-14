--- Window
---@class Engine
---@field ui_ctx UIContext
---@field window Window
---@field graphics Graphics
---@field audio Audio
local Engine = {}

---@class DSLModule
---@field name string
---@field kind "Init" | "Core"
---@field deps string[]
---@field shared_state table
---@field local_state table
---@field init fun(local_state)
---deps.state
---@field update fun(table)
---@param module DSLModule
---run a module on a new thread.
---
---@diagnostic disable-next-line: lowercase-global
function register_threaded_module(module) end

---@diagnostic disable-next-line: lowercase-global
function Engine:set_running() end

---@diagnostic disable-next-line: lowercase-global
function Engine:set_pause() end

---@diagnostic disable-next-line: lowercase-global
function Engine:set_exiting() end


---@return boolean
---@diagnostic disable-next-line: lowercase-global
function Engine:is_running() end

---@return boolean
---@diagnostic disable-next-line: lowercase-global
function Engine:is_pause() end

---@return boolean
---@diagnostic disable-next-line: lowercase-global
function Engine:is_exiting() end