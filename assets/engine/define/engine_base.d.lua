_G.ResourceManager = {}
---@param path string
function ResourceManager:preload_ui_texture(path)

end
---@class Font
---@param path string
---@return Font
---@diagnostic disable-next-line: lowercase-global
function ResourceManager:load_font(path)
    return {}
end

---@class Image
---@param path string
---@return Image
---@diagnostic disable-next-line: lowercase-global
function ResourceManager:load_image(path)
    return {}
end

---@class DSLModule
---@field name string
---@field kind "Init" | "Core"
---@field deps string[]
---@field state table
---self.state
---@field init fun(state)
---deps.state
---@field update fun(table)
---@param module DSLModule
---@diagnostic disable-next-line: lowercase-global
function register_module(module) end