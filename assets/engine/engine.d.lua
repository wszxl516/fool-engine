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
---@param grabbed boolean
function Window:set_cursor_grab(grabbed)
end

---@param enable boolean
function Window:set_ime_allowed(enable)
end

---@param position Point
function Window:set_ime_position(position)
end

---@param icon string
---   [Default, Crosshair, Hand, Arrow, Move,
---    Text, Wait, Help, Progress, NotAllowed, ContextMenu, Cell, VerticalText,
---    Alias, Copy, NoDrop, Grab, Grabbing, AllScroll, ZoomIn, ZoomOut, EResize,
---    NResize, NeResize, NwResize, SResize, SeResize, SwResize, WResize, EwResize, NsResize,
---    NeswResize, NwseResize, ColResize, RowResize]
function Window:set_cursor_icon(icon)
end

---@param position Point
function Window:set_cursor_position_points(position)
end

---@param visible boolean
function Window:set_cursor_visible(visible)
end

---@param always_on_top boolean
function Window:set_always_on_top(always_on_top)
end

---@param decorations boolean
function Window:set_decorations(decorations)
end

---@param fullscreen boolean
function Window:set_fullscreen(fullscreen)
end

---@param position Point
function Window:set_ime_position_points(position)
end

---@param width number
---@param height number
function Window:set_inner_size_pixels(width, height)
end

---@param width number
---@param height number
function Window:set_inner_size_points(width, height)
end

---@param size Size|nil
function Window:set_max_inner_size_points(size)
end

---@param maximized boolean
function Window:set_maximized(maximized)
end

---@param size Size|nil
function Window:set_min_inner_size_points(size)
end

---@param minimized boolean
function Window:set_minimized(minimized)
end

---@param position Point
function Window:set_outer_position_pixels(position)
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

---@param path string|nil
function Window:set_window_icon(path)
end

---@return integer
function Window:elapsed_frames()
    return 0
end

---@return integer
function Window:msaa_samples()
    return 0
end

---@param path string
function Window:capture_frame(path)
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

---@return Size
function Window:inner_size_pixels()
    return {}
end

---@return Size
function Window:outer_size_pixels()
    return {}
end

---@return Size
function Window:inner_size_points()
    return {}
end

---@return Size
function Window:outer_size_points()
    return {}
end

---@return [Point]
function Window:rect()
    return {}
end

---@return boolean
function Window:is_fullscreen()
    return false
end

---@param ctx EguiContext
---@param font Font
---@diagnostic disable-next-line: lowercase-global
function Window:set_gui_font(ctx, font)

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
---@field wrap? boolean
---@field noninteractive_fg_color? Color8
---@field hovered_fg_color? Color8
---@field active_fg_color? Color8
---@field inactive_fg_color? Color8,
---@field open_fg_color? Color8
---@param ctx EguiContext
---@param style LuaGuiStyle
---@diagnostic disable-next-line: lowercase-global
function Window:set_gui_style(ctx, style)

end

--- Event
---@class Event
local Event = {}
---  [Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0,
---   A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
---   Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,
---   Snapshot, Scroll, Pause, Insert, Home, Delete, End, PageDown, PageUp,
---   Left, Up, Right, Down, Back, Return, Space, Compose, Caret,
---   Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9, NumpadAdd, NumpadDivide,
---   NumpadDecimal, NumpadComma, NumpadEnter, NumpadEquals, NumpadMultiply, NumpadSubtract,
---   AbntC1, AbntC2, Apostrophe, Apps, Asterisk, At, Ax, Backslash, Calculator, Capital, Colon, Comma, Convert, Equals, Grave, Kana,
---   Kanji, LAlt, LBracket, LControl, LShift, LWin, Mail, MediaSelect, MediaStop, Minus, Mute, MyComputer, NavigateForward, NavigateBackward,
---   NextTrack, NoConvert, OEM102, Period, PlayPause, Plus, Power, PrevTrack, RAlt, RBracket, RControl, RShift, RWin, Semicolon, Slash, Sleep,
---   Stop, Sysrq, Tab, Underline, Unlabeled, VolumeDown, VolumeUp, Wake, WebBack, WebFavorites, WebForward, WebHome, WebRefresh, WebSearch,
---   WebStop, Yen, Copy, Paste, Cut]
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

---@param call_back fun():boolean
function Event:on_exit(call_back) end
---exit engine
function Event:exit() end


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

---@class ImageConfig
---@field show_loading_spinner? boolean
---@field rotate_origin? Point
---@field rotate_angle? number
---@field w? number
---@field h? number
---@field nw? number
---@field ne? number
---@field sw? number
---@field se? number
---@field bg_fill? Color8
---only for image_button
---@field frame? boolean
---@field stroke_width? number
---@field stroke_color? Color8
---@field wrap? boolean
---@param texture Texture
---@param config ImageConfig
function UIContext:image(texture, config) end

---@param texture Texture
---@param config ImageConfig
function UIContext:image_button(texture, config) end

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
---@field extrusion number
---@field color Color8

---@class Stroke
---@field width number
---@field color Color8

---@class Frame
---@field inner_margin? Margin
---@field outer_margin? Margin
---@field rounding? Rounding
---@field shadow? Shadow
---@field fill? Color8
---@field stroke? Stroke
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
---@class EguiContext
---@param config UiConfig
---@param context EguiContext
---@param bg_img Texture|nil
---@param bg_img_color Color8|nil
---@param body fun(ctx: UIContext)
---@diagnostic disable-next-line: lowercase-global
function gui_create_window(config, context, bg_img, bg_img_color, body)

end

_G.ResourceManager = {}
---@class Texture
---@param text string
---@param font_name string
---@param font_size number
---@return Size
function ResourceManager:measure_text(text, font_name, font_size) end

---@param path string
---@return Texture
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
