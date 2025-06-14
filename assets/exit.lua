local rgba8 = require("engine.color.rgba8")
---@param engine Engine
---@param event Event
---@param dt number -- delay time
---@diagnostic disable-next-line: lowercase-global
function exit(engine, event, dt)
    engine.audio:pause_all(0)
    engine.ui_ctx:draw_window({
        title = "Are you sure to exit?",
        collapsible = false,
        constrain = false,
        default_open = true,
        drag_to_scroll = false,
        resizable = false,
        title_bar = true,
        movable = false,
        x = 0.0,
        y = -100.0,
        w = 200.0,
        h = 50.0,
        font_name = "fonts/SarasaTermSCNerd-Regular.ttf",
        frame = {
            inner_margin = { left = 5, right = 5, top = 5, bottom = 5 },
            outer_margin = { left = 1, right = 1, top = 1, bottom = 1 },
            rounding = { nw = 5, ne = 5, sw = 5, se = 5 },
            shadow = { offset = { 1, 2 }, blur = 1, spread = 1, color = rgba8.new(0, 0, 0, 0) },
            fill = rgba8.new(0, 0, 0, 0),
            stroke_width = 1,
            stroke_color = rgba8.new(50, 50, 50, 50)
        },
        bg_img_color = rgba8.new(100, 100, 100, 50)
    }, function(ui)
        ui:empty_space(0, 30)
        ui:horizontal(function(ctx)
            if ctx:button("Yes"):clicked() then
                engine.window:exit()
            end
            ctx:empty_space(150, 0)
            if ctx:button("No"):clicked() then
                engine:set_running()
                engine.audio:resume_all(0)
            end
        end)
    end)
end

return exit
