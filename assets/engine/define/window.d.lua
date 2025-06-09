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

---@param fps number
---@diagnostic disable-next-line: lowercase-global
function Window:set_fps(fps)
end