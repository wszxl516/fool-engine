local size       = require("engine.vector2.size")
local rgba8      = require("engine.color.rgba8")

local LOG        = require("engine.log")
local logger     = LOG.new("main", true, true)
local UI         = require("ui")
local ui         = UI:new()
local shape      = require("shape")
local lua_thread = require('lua_thread')
local Physics    = require("Physics")
register_module(lua_thread)

---@param engine Engine
---@diagnostic disable-next-line: lowercase-global
function init(engine)
    local ui_context = engine.ui_ctx
    local window = engine.window
    window:set_title("window")
    window:set_resizable(false)
    window:set_max_inner_size(size.new(800, 800))
    window:set_min_inner_size(size.new(800, 800))
    window:set_fullscreen(false)
    window:set_cursor("image/cursor.png")
    window:set_cursor_grab("None")
    window:set_cursor_visible(false)
    window:set_window_icon("image/linux.png")
    window:set_fps(30)
    ui_context:set_font("fonts/SarasaTermSCNerd-Regular.ttf")
    ui_context:set_style({
        text = {
            Heading = 22.0,
            Body = 18.0,
            Button = 18.0,
            Small = 16.0,
            Monospace = 18.0
        },
        dark = true,
        animation_time = 0.2,
        wrap = "Extend",
        noninteractive_fg_color = rgba8.new(255, 0, 0, 0),
        hovered_fg_color = rgba8.new(255, 255, 255, 0),
        active_fg_color = rgba8.new(0, 0, 0, 0),
        inactive_fg_color = rgba8.new(200, 200, 200, 200),
        open_fg_color = rgba8.new(200, 0, 0, 0)
    })
    logger:debug("window:monitor %s", window:monitor())
    ui:init()
    window:on_exit(function()
        logger:debug("exit from lua")
        return true
    end)
    shape:init(window)
    Physics.new(9.8, 1)
end

---@param engine Engine
---@param event Event
---@param dt number -- delay time
---@diagnostic disable-next-line: lowercase-global
function run_frame(engine, event, dt)
    local window = engine.window
    if event:key_pressed("Insert") then
        logger:debug("Insert pressed")
        window:capture()
    end
    local ime = event:ime_state()
    if ime then
        logger:debug("raw_key %s", ime)
    end
    if event:key_pressed("Escape") then
        logger:debug("Escape pressed exit")
        window:exit()
    end
    ui:view(engine)
    shape:view(engine)
    -- window:set_ime_allowed(true)
    -- window:set_ime_cursor_area(point.new(100,100), size.new(100,100))
end
