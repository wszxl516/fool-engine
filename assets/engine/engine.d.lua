---@class Size
---@field w number
---@field h number

--- Draw
--- Canvas
---@class Canvas
local Canvas = {}
-- -@class Color8
-- -@field r? number
-- -@field g? number
-- -@field b? number
-- -@field a? number
---@class LuaTextLayout
---@field line_spacing? number
---@field line_wrap? string ---[ "Character", "Whitespace", "None"]
---@field size? Size
---@field x_align? string ---["Start", "Middle", "End"]
---@field y_align? string ---["Start", "Middle", "End"]
---@field radians? number
---@field gray? number
---@param text string
---@param point Point
---@param color Color8
---@param font Font | nil
---@param font_size number | nil
---@param layout LuaTextLayout
function Canvas:draw_text(text, point, font, font_size, color, layout)
    -- Implementation (in Rust)
end

---@param text string,
---@param font_name string
---@param font_size number
---@return Point
function Canvas:measure_text(text, font_name, font_size)
    -- Implementation (in Rust)
    return {}
end

---@class LuaRadians
---@field x number
---@field y number
---@field z number
---@class StrokeOptions
---@field start_cap string  in {"Butt", "Square", "Round"}
---@field end_cap string in {"Butt", "Square", "Round"}
---@field line_join string in {"Miter", "MiterClip", "Round", "Bevel"}
---@field line_width number
---@field miter_limit number
---@field tolerance number
---@class LuaPolygonOptions
---@field position? {x: number, y: number, z: number}
---@field width? number
---@field height? number
---@field no_fill? boolean
---@field stroke_color? Color8
---@field color? Color8
---@field stroke? StrokeOptions
---@param polygon_options LuaPolygonOptions
function Canvas:draw_rect(polygon_options)
    -- Implementation (in Rust)
end

---@param polygon_options LuaPolygonOptions
function Canvas:draw_ellipse(polygon_options)
    -- Implementation (in Rust)
end

---@param x1 number
---@param y1 number
---@param x2 number
---@param y2 number
---@param w number
---@param head_width number
---@param head_length number
---@param tolerance number 0.0 ~1.0
---@param color Color8
---@param polygon_options LuaPolygonOptions
---only use radians, start_cap, end_cap, line_join, miter_limit
function Canvas:draw_arrow(x1, y1, x2, y2, w, head_width, head_length, tolerance, color, polygon_options)
    -- Implementation (in Rust)
end

---@param x1 number
---@param y1 number
---@param x2 number
---@param y2 number
---@param w number
---@param tolerance number 0.0 ~1.0
---@param color Color8
---@param polygon_options LuaPolygonOptions
---only use radians, start_cap, end_cap, line_join, miter_limit
function Canvas:draw_line(x1, y1, x2, y2, w, tolerance, color, polygon_options)
    -- Implementation (in Rust)
end

---@param x1 number
---@param y1 number
---@param x2 number
---@param y2 number
---@param x3 number
---@param y3 number
---@param color Color8
---@param polygon_options LuaPolygonOptions
---only use radians, start_cap, end_cap, line_join, miter_limit
function Canvas:draw_triangle(x1, y1, x2, y2, x3, y3, color, polygon_options)
    -- Implementation (in Rust)
end

---@class Point
---@field x number
---@field y number
---@param points [Point]
---@param color Color8
---@param radians LuaRadians
---only use radians, start_cap, end_cap, line_join, miter_limit
function Canvas:draw_polyline(points, color, radians)
    -- Implementation (in Rust)
end

---@param points [Point] # Must contain exactly 4 points
---@param color Color8
---@param polygon_options LuaPolygonOptions
---only use radians, start_cap, end_cap, line_join, miter_limit
function Canvas:draw_quad(points, color, polygon_options)
    -- Implementation (in Rust)
end

---@param points [Point]
---@param color Color8
---@param polygon_options LuaPolygonOptions
---only use radians, start_cap, end_cap, line_join, miter_limit
function Canvas:draw_polygon(points, color, polygon_options)
    -- Implementation (in Rust)
end

---@class LuaTextureLayout
---@field radians LuaRadians
---@field area_x number[2] --- 0.0 ~ 1.0
---@field area_y number[2] --- 0.0 ~ 1.0
---@
---@param texture Texture
---@param x number
---@param y number
---@param w number
---@param h number
---@param layout LuaTextureLayout
function Canvas:draw_texture(texture, x, y, w, h, layout)
    -- Implementation (in Rust)
end

---@param color Color8
function Canvas:fill_background(color)
    -- Implementation (in Rust)
end

---@class LuaColoredPoint
---@field p Point
---@field c Color8
---@param colored_points [LuaColoredPoint]
---@param radians LuaRadians
function Canvas:draw_points(colored_points, radians)
    -- Implementation (in Rust)
end

---@diagnostic disable-next-line: lowercase-global
physics = {}
physics.__index = physics

--- Physics
---@class physics
local physics_mt = {}
physics_mt.__index = physics_mt

---@class PhysicsBodyConfig
---@field user_data number
---@field position table
---@field position.x? number
---@field position.y? number
---@field shape table
---@field shape.Ball? table
---@field shape.Ball.radius? number
---@field shape.Cuboid? table
---@field shape.Cuboid.width? number
---@field shape.Cuboid.height? number
---@field body_type string @"Dynamic"|"Fixed"|"KinematicPositionBased"|"KinematicVelocityBased"
---@field rotation? number | nil ---nil for lock_rotations
---@field linear_damping? number
---@field angular_damping? number
---@field gravity_scale? number
---@field additional_mass? number
---@field mass? number
---@field can_sleep? boolean
---@field sleeping? boolean
---@field restitution? number
---@field friction? number
---@field density? number
---@field is_sensor? boolean
---@field active_events? string @"collision_events"|"contact_force_events"|"all"
---@field active_hooks? string @"filter_contact_pairs"|"filter_intersection_pair"|"all"
---
---@class LuaRigidBodyHandle

---@class LuaRigidBody
---@field pos Point --- x, y
---@field angle number
---@field linvel Point --- x, y
---@field angvel number
---@field mass number
---@field is_fixed boolean
---@field user_data number

---@param x_gravity_acceleration number
---@param y_gravity_acceleration number
---@return physics
---@diagnostic disable-next-line: lowercase-global
function physics_init(x_gravity_acceleration, y_gravity_acceleration)
    local self = setmetatable({}, physics_mt)
    -- Initialize physics here using the gravity values
    return self
end

---@param x number
---@param y number
function physics_mt:set_gravity(x, y)
    -- Set gravity logic
end

function physics_mt:update()
end

---@param config PhysicsBodyConfig
---@return LuaRigidBodyHandle
function physics_mt:add_body(config)
    return {} -- placeholder for a real body handle
end

---@param handle LuaRigidBodyHandle
function physics_mt:remove_body(handle)
    return {} -- placeholder for a real body handle
end

---@return LuaRigidBody[]
function physics_mt:get_bodies()
    return {} -- return list of rigid bodies
end

---@param handle LuaRigidBodyHandle
---@return LuaRigidBody|nil
function physics_mt:find_body(handle)
    return nil
end

---@param handle LuaRigidBodyHandle
---@param force Point
function physics_mt:apply_force(handle, force)
    return nil
end

---@param handle LuaRigidBodyHandle
---@param force Point
function physics_mt:apply_impulse(handle, force)
    return nil
end

---@param handle LuaRigidBodyHandle
---@param linvel Point
function physics_mt:set_linvel(handle, linvel)
    -- set linear velocity
end

---@param handle LuaRigidBodyHandle
---@param angvel number
function physics_mt:set_angvel(handle, angvel)
    -- set angular velocity
end

---@param handle LuaRigidBodyHandle
---@param strong boolean
function physics_mt:wake_up(handle, strong)
    -- wake up a body
end

---@param handle LuaRigidBodyHandle
function physics_mt:sleep(handle)
    -- put body to sleep
end

---@param handle LuaRigidBodyHandle
---@return boolean
function physics_mt:is_sleeping(handle)
    return true -- or false
end

---@class CastRayRes
---@field handle LuaRigidBodyHandle
---@field distance number
---@param origin Point
---@param dir Point
---@param max_toi number
---@return CastRayRes
function physics_mt:cast_ray(origin, dir, max_toi)
end

---only for sensor
---@return string[]
function physics_mt:list_ignore_intersection_group()

end

---@param name  string
---@param group LuaRigidBodyHandle[]
function physics_mt:add_ignore_intersection_group(name, group)

end

---@param name  string
function physics_mt:remove_ignore_intersection_group(name)

end

---only for no sensor
---@return string[]
function physics_mt:list_contact_filter_groups()

end

---@param name  string
---@param group LuaRigidBodyHandle[]
function physics_mt:add_contact_filter_group(name, group)

end

---@param name  string
function physics_mt:remove_contact_filter_group(name)

end

---@class CollisionEvent
---@field b1 LuaRigidBodyHandle
---@field b2 LuaRigidBodyHandle
---@field sensor boolean
---@field removed boolean
-- Called every frame when collision state changes
-- One of Started or Stopped will be non-nil
---@class LuaCollisionEvent
---@field started? CollisionEvent
---@field stopped? CollisionEvent

---@param call_back fun(LuaCollisionEvent)
function physics_mt:register_collision_event_callback(call_back)
end

---@class LuaContactForceEvent
---@field b1 LuaRigidBodyHandle
---@field b2 LuaRigidBodyHandle
---@field dt number
---@field total_force_magnitude number
---@param call_back fun(LuaContactForceEvent)
function physics_mt:register_contact_force_event_callback(call_back)
end

---if add_body active_events not empty and call_back is registered
function physics_mt:event_update()

end

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
function Window:load_cursor_icon(icon)
end

---@param icon string
function Window:set_cursor_icon(icon)
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

function Window:exit()
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
--- Event
---@class Event
local Event = {}
---[Backquote, Backslash, BracketLeft, BracketRight, Comma,
--- Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9, Equal,
--- IntlBackslash, IntlRo, IntlYen,
--- KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM,
--- KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ,
--- Minus, Period, Quote, Semicolon, Slash, AltLeft, AltRight, Backspace, CapsLock,
--- ContextMenu, ControlLeft, ControlRight, Enter, SuperLeft, SuperRight, ShiftLeft,
--- ShiftRight, Space,
--- Tab, Convert, KanaMode, Lang1, Lang2, Lang3, Lang4, Lang5, NonConvert, Delete, End,
--- Help, Home, Insert, PageDown, PageUp, ArrowDown, ArrowLeft, ArrowRight, ArrowUp,
--- NumLock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
--- Numpad5, Numpad6, Numpad7, Numpad8, Numpad9, NumpadAdd, NumpadBackspace, NumpadClear, NumpadClearEntry,
--- NumpadComma, NumpadDecimal, NumpadDivide, NumpadEnter, NumpadEqual, NumpadHash,
--- NumpadMemoryAdd, NumpadMemoryClear, NumpadMemoryRecall, NumpadMemoryStore,
--- NumpadMemorySubtract, NumpadMultiply, NumpadParenLeft, NumpadParenRight, NumpadStar, NumpadSubtract, Escape,
--- Fn, FnLock, PrintScreen, ScrollLock, Pause, BrowserBack, BrowserFavorites, BrowserForward, BrowserHome,
--- BrowserRefresh, BrowserSearch, BrowserStop,Eject, LaunchApp1, LaunchApp2, LaunchMail, MediaPlayPause, MediaSelect,
--- MediaStop, MediaTrackNext, MediaTrackPrevious, Power, Sleep, AudioVolumeDown, AudioVolumeMute,
--- AudioVolumeUp,WakeUp, Meta, Hyper,Turbo, Abort, Resume, Suspend, Again, Copy, Cut, Find,
--- Open, Paste, Props, Select, Undo, Hiragana, Katakana
--- F1, F2 ,F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18,
--- F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31, F32, F33, F34, F35,
--- ]
---@param key string
---@return boolean
function Event:key_pressed(key) end

---@param key string
---@return boolean
function Event:key_released(key) end

---@param key string
---@return boolean
function Event:key_held(key) end

---@return table {x = number, y = number}
function Event:get_mouse_position() end

---@param button string "Left"、"Right"、"Middle"
---@return boolean
function Event:mouse_pressed(button) end

---@param button string "Left"、"Right"、"Middle"
---@return boolean
function Event:mouse_released(button) end

---@class mouse_wheel
---@field delta {line: Point | nil, pixel: Point | nil}|nil
---@field touch string | nil
---@return mouse_wheel
---touch：string，["Started"、"Moved"、"Ended"、"Cancelled"]
function Event:mouse_wheel() end

---@return string|nil
function Event:get_char() end

---@return boolean
function Event:mouse_entered() end

---@return boolean
function Event:focused() end

---@class Preedit
---@field content string
---@field pos nil| Point
---@class State
---@field state string "enabled" | "disabled" | "preedit" | "commit"
---@field preedit Preedit
---@field commit string
---@return State
function Event:ime_state() end

---@param call_back fun()
function Event:on_exit(call_back) end


---@class LuaResponse
local LuaResponse = {}
---@return boolean
function LuaResponse:clicked() end

---@return boolean
function LuaResponse:changed() end

---@return boolean
function LuaResponse:double_clicked() end

---@return boolean
function LuaResponse:middle_clicked() end

---@return boolean
function LuaResponse:secondary_clicked() end

---@return boolean
function LuaResponse:hovered() end

---@return boolean
function LuaResponse:dragged() end

---@return boolean
function LuaResponse:has_focus() end

---@return boolean
function LuaResponse:lost_focus() end

---@return boolean
function LuaResponse:gained_focus() end

---@return boolean
function LuaResponse:clicked_elsewhere() end

--- UIContext
---@class UIContext
local UIContext = {}
---@param text string
---@return LuaResponse
function UIContext:label(text) end

---@param label string
---@return LuaResponse
function UIContext:button(label) end

---@class Label
---@field checked boolean
---@field label string
---@param args Label
---@return LuaResponse
function UIContext:checkbox(args) end

---@class TextEdit
---@field id string
---@field content string
---@field single_line boolean
---@field char_limit number
---@field clip_text boolean
---@field rows number
---@field code_editor boolean
---@field password boolean
---@param text_edit TextEdit
---@return LuaResponse
function UIContext:text_edit(text_edit) end

---@class Slider
---@field current number
---@field min number
---@field max number
---@field label string
---@param args Slider
---@return  LuaResponse
function UIContext:slider(args) end

---@class ProgressBar
---@field progress number [0.0 ~ 1.0]
---@field name string
---@field show_percentage boolean
---@field color Color8
---@field animate boolean
---@param args ProgressBar
---@return LuaResponse
function UIContext:progress_bar(args) end

---@class ColorPicker
---@field color Color8
---@param color_picker ColorPicker
---@return LuaResponse
function UIContext:color_picker(color_picker) end

---@param label string
---@param body fun(ctx: UIContext)
---@return LuaResponse
function UIContext:collapsing(label, body) end

---@param body fun(ctx: UIContext)
---@return LuaResponse
function UIContext:horizontal(body) end

---@param body fun(ctx: UIContext)
---@return LuaResponse
function UIContext:vertical(body) end

---@class ComboBox
---@field id  string
---@field selected string
---@field items string[]
---@param combo_box ComboBox
---@return LuaResponse
function UIContext:combo_box(combo_box) end

---@param id string
---@param spacing Size
---@param start_row number
---@param body fun(ctx: UIContext)
---@return LuaResponse
function UIContext:grid(id, spacing, start_row, body) end

---@return LuaResponse
function UIContext:separator() end

---@param text string
---@return LuaResponse
function UIContext:heading(text) end

---@param url string
---@return LuaResponse
function UIContext:hyperlink(url) end

---@param text string
---@return LuaResponse
function UIContext:small(text) end

---@class Radio
---@field selected boolean
---@field text string
---@param args Radio[]
---@param left_to_right boolean
---@return LuaResponse
function UIContext:radio(args, left_to_right) end

---@param selected boolean
---@param label string
---@return LuaResponse
function UIContext:selectable_label(selected, label) end

---@param topdown_or_leftright boolean
---@param reverse boolean
---@param body fun(ctx: UIContext)
---@return LuaResponse
function UIContext:with_layout(topdown_or_leftright, reverse, body) end

---@param size Size
function UIContext:set_max_size(size) end

---@param size Size
function UIContext:set_min_size(size) end

---@param height number
function UIContext:set_row_height(height) end

function UIContext:end_row() end
---@class Rotate
---@field angle number 0.0 - 2π
---@field origin Point 0.0 - 1.0
---@class UV 
---@field min Point
---@field max Point
---@class ImageButtonConfig
---@field img string
---@field label string|nil
---@field show_loading_spinner boolean|nil
---@field img_bg_fill Color8 | nil
---@field img_max_size Size | nil
---@field img_rotate Rotate
---@field frame boolean| nil
---@field tint Color8 | nil
---@field selected boolean | nil
---@field corner_radius Rounding| nil
---@field uv UV|nil
---@field sense "HOVER" | "CLICK" | "DRAG" | "FOCUSABLE" | "ALL" | nil
---@param config ImageButtonConfig
function UIContext:image_button(config) end

---@param config ImageButtonConfig
function UIContext:image(config) end

---@class EguiContext
EguiContext = {}
---@param name string
function EguiContext:set_font(name)
end
---@class TextStyle
---@field Small number
---@field Body number
---@field Monospace number
---@field Button number
---@field Heading number
---@class LuaGuiStyle
---@field text? TextStyle
---@field dark? boolean
---@field animation_time? number
---@field wrap? "Extend"|"Wrap" |"Truncate"|nil
---@field noninteractive_fg_color? Color8
---@field hovered_fg_color? Color8
---@field active_fg_color? Color8
---@field inactive_fg_color? Color8,
---@field open_fg_color? Color8
---@param style LuaGuiStyle
---@diagnostic disable-next-line: lowercase-global
function EguiContext:set_style(style)

end

_G.ResourceManager = {}
---@param path string
function ResourceManager:load_texture(path)

end

---@class Font
---@param path string
---@return Font
---@diagnostic disable-next-line: lowercase-global
function ResourceManager:load_font(path)
    -- Implementation (in Rust)
end

---@class Image
---@param path string
---@return Image
---@diagnostic disable-next-line: lowercase-global
function ResourceManager:load_image(path)
    -- Implementation (in Rust)
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
function register_module(module)end
    
