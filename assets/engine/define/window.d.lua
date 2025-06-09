--- Window
---@class Window
local Window = {}
---@param grab "None"|"Confined"|"Locked"
function Window:set_cursor_grab(grab)
end

---@param enable boolean
function Window:set_ime_allowed(enable)
end

---@param position Point
---@param size Size
function Window:set_ime_cursor_area(position, size)
end

---@param icon string
--- "default" |"context-menu" "help" | "pointer"| "progress"| "wait" "cell" 
--- "crosshair" | "text" | "vertical-text" | "alias" | "copy" | "move" | "no-drop" 
--- "not-allowed" | "grab" | "grabbing" | "e-resize" | "n-resize" | "ne-resize" 
--- "nw-resize" | "s-resize" | "se-resize" | "sw-resize" | "w-resize" | "ew-resize" 
--- "ns-resize" | "nesw-resize" | "nwse-resize" | "col-resize" | "row-resize" 
--- "all-scroll" | "zoom-in" | "zoom-out" 
function Window:set_cursor(icon)
end

---@param icon string
function Window:set_window_icon(icon)
end
---@param visible boolean
function Window:set_cursor_visible(visible)
end


---@param fullscreen boolean
function Window:set_fullscreen(fullscreen)
end


---@param size Size
function Window:set_max_inner_size(size)
end
---@param size Size
function Window:set_min_inner_size(size)
end
---@param maximized boolean
function Window:set_maximized(maximized)
end

---@param decorations boolean
function Window:set_decorations(decorations)
end

---@param resizable boolean
function Window:set_resizable(resizable)
end

---@param title string
function Window:set_title(title)
end

---@param visible boolean
function Window:set_visible(visible)
end

---@return Size
function Window:inner_size()
    return {}
end

---@return Size
function Window:outer_size()
    return {}
end

---@class monitorInfo
---@field name string
---@field position Point
---@field refresh_rate_millihertz number
---@field scale_factor number
---@field size Size
---@return monitorInfo|nil
function Window:monitor()
end

---@return boolean
function Window:is_fullscreen()
    return false
end

---@param call_back fun():boolean
function Window:on_exit(call_back) end
function Window:exit()
end
---capture screen save to %Y-%m-%d %H:%M:%S.png
function Window:capture()
end
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
---@param context EguiContext
---@param body fun(ctx: UIContext)
---@diagnostic disable-next-line: lowercase-global
function Window:gui_window(config, context, body)

end

---@class Scene
---@field style? Style
---@field apply_parent_style? boolean
---@field drawable? SceneNodeKind
---@field children? Scene[]
---@param node Scene
---@diagnostic disable-next-line: lowercase-global
function Window:draw(node)
end