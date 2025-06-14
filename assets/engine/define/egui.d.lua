

---@class LuaResponse
local UiResponse = {}
---@return boolean
function UiResponse:clicked()
    return true
end

---@return boolean
function UiResponse:changed()
    return true
end

---@return boolean
function UiResponse:double_clicked()
    return true
end

---@return boolean
function UiResponse:middle_clicked()
    return true
end

---@return boolean
function UiResponse:secondary_clicked()
    return true
end

---@return boolean
function UiResponse:hovered()
    return true
end

---@return boolean
function UiResponse:dragged()
    return true
end

---@return boolean
function UiResponse:has_focus()
    return true
end

---@return boolean
function UiResponse:lost_focus()
    return true
end

---@return boolean
function UiResponse:gained_focus()
    return true
end

---@return boolean
function UiResponse:clicked_elsewhere()
    return true
end

--- UIContext
---@class UIContext
local UIContext = {}
---@param text string
---@return LuaResponse
function UIContext:label(text)
    return {}
end

---@param label string
---@return LuaResponse
function UIContext:button(label)
    return {}
end

---@class Label
---@field checked boolean
---@field label string
---@param args Label
---@return LuaResponse
function UIContext:checkbox(args)
    return {}
end

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
function UIContext:text_edit(text_edit)
    return {}
end

---@class Slider
---@field current number
---@field min number
---@field max number
---@field label string
---@param args Slider
---@return  LuaResponse
function UIContext:slider(args)
    return {}
end

---@class ProgressBar
---@field progress number [0.0 ~ 1.0]
---@field name string
---@field show_percentage boolean
---@field color Color8
---@field animate boolean
---@param args ProgressBar
---@return LuaResponse
function UIContext:progress_bar(args)
    return {}
end

---@class ColorPicker
---@field color Color8
---@param color_picker ColorPicker
---@return LuaResponse
function UIContext:color_picker(color_picker)
    return {}
end

---@param label string
---@param body fun(ctx: UIContext)
---@return LuaResponse
function UIContext:collapsing(label, body)
    return {}
end

---@param body fun(ctx: UIContext)
---@return LuaResponse
function UIContext:horizontal(body)
    return {}
end

---@param body fun(ctx: UIContext)
---@return LuaResponse
function UIContext:vertical(body)
    return {}
end

---@class ComboBox
---@field id  string
---@field selected string
---@field items string[]
---@param combo_box ComboBox
---@return LuaResponse
function UIContext:combo_box(combo_box)
    return {}
end

---@param id string
---@param spacing Size
---@param start_row number
---@param body fun(ctx: UIContext)
---@return LuaResponse
function UIContext:grid(id, spacing, start_row, body)
    return {}
end

---@return LuaResponse
function UIContext:separator()
    return {}
end
---@param width number
---@param height number
function UIContext:empty_space(width, height)
end
---@param text string
---@return LuaResponse
function UIContext:heading(text)
    return {}
end

---@param url string
---@return LuaResponse
function UIContext:hyperlink(url)
    return {}
end

---@param text string
---@return LuaResponse
function UIContext:small(text)
    return {}
end

---@class Radio
---@field selected boolean
---@field text string
---@param args Radio[]
---@param left_to_right boolean
---@return LuaResponse
function UIContext:radio(args, left_to_right)
    return {}
end

---@param selected boolean
---@param label string
---@return LuaResponse
function UIContext:selectable_label(selected, label)
    return {}
end

---@param topdown_or_leftright boolean
---@param body fun(ctx: UIContext)
---@return LuaResponse
function UIContext:with_layout(topdown_or_leftright, body)
    return {}
end

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
---@field scale number | nil
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

---@param name string
function UIContext:set_font(name)
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
function UIContext:set_style(style)

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
---@param body fun(ctx: UIContext)
---@diagnostic disable-next-line: lowercase-global
function UIContext:draw_window(config, body)

end