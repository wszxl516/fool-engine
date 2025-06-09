--- Window
---@class Engine
---@field ui_ctx UIContext
---@field window Window
local Engine = {}

---@class Margin
---@field left number
---@field right number
---@field top number
---@field bottom number

---@class Rounding
---@field nw number
---@field ne number
---@field sw number
---@field se number

---@class Shadow
---@field offset number[]
---@field blur number
---@field spread number
---@field color Color8

---@class Frame
---@field inner_margin? Margin
---@field outer_margin? Margin
---@field rounding? Rounding
---@field shadow? Shadow
---@field fill? Color8
---@field stroke_width? number
---@field stroke_color? Color8
---@class UiConfig
---@field title string
---@field collapsible? boolean
---@field constrain? boolean
---@field default_open? boolean
---@field drag_to_scroll? boolean
---@field resizable? boolean
---@field title_bar? boolean
---@field movable? boolean
---@field frame? Frame
---@field x number
---@field y number
---@field w number
---@field h number
---@field bg_img string|nil
---@field bg_img_color Color8|nil
---@param config UiConfig
---@param body fun(ctx: UIContext)
---@diagnostic disable-next-line: lowercase-global
function Engine:draw_window(config, body)

end

---@class Scene
---@field style? Style
---@field apply_parent_style? boolean
---@field drawable? SceneNodeKind
---@field children? Scene[]
---@param node Scene
---@diagnostic disable-next-line: lowercase-global
function Engine:draw_shape(node)
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