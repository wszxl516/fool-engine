use super::types::{
    LinearRrgba, LuaColor, LuaColoredPoint, LuaPoint, LuaPolygonOptions, LuaRadians, LuaTextLayout,
    LuaTextureLayout,
};
use crate::resource::types::{LuaFont, LuaTexture};
use mlua::{Error as LuaError, UserData, UserDataMethods, Value};
use nannou::{
    draw::Draw,
    geom::{pt2, pt3, Point2, Point3, Range, Rect},
    text::Layout,
};
pub struct LuaCancvas {
    draw: Draw,
}
impl LuaCancvas {
    pub fn new(draw: Draw) -> Self {
        LuaCancvas { draw }
    }
}

impl UserData for LuaCancvas {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "draw_text",
            |_lua,
             this,
             (text, x, y, font, font_size, color, layout): (
                String,
                f32,
                f32,
                Value,
                Option<u32>,
                LuaColor,
                LuaTextLayout,
            )| {
                let mut r_layout = Layout {
                    line_spacing: layout.line_spacing,
                    line_wrap: layout.line_wrap.map(|w| w.into()),
                    justify: layout.x_align.into(),
                    font_size: font_size.unwrap_or(14),
                    font: None,
                    y_align: layout.y_align.into(),
                };
                match font {
                    mlua::Value::UserData(ud) => {
                        let f = ud.borrow::<LuaFont>()?;
                        r_layout.font = Some(f.graphics.clone())
                    }
                    Value::Nil => {}
                    _ => {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: font.type_name(),
                            to: "LuaFont".into(),
                            message: Some("Expected LuaFont as UserData from draw_text".into()),
                        });
                    }
                };

                let text = this.draw.text(&text).x_y(x, y);
                let text = if let (Some(w), Some(h)) = (layout.w, layout.h) {
                    text.w_h(w, h)
                } else {
                    text
                }
                .color::<LinearRrgba>(color.into());
                let text = if let Some(radians) = layout.radians {
                    text.rotate(radians)
                } else {
                    text
                };
                if let Some(gray) = layout.gray {
                    text.gray(gray)
                } else {
                    text
                }
                .layout(&r_layout)
                .finish();
                Ok(())
            },
        );
        methods.add_method(
            "draw_texture",
            |_lua, this, (path, x, y, w, h, layout): (Value, f32, f32, f32, f32, LuaTextureLayout)| {
                let texture = match path.as_userdata(){
                    Some(texture) => texture.borrow::<LuaTexture>()?,
                    None => return  Err(LuaError::RuntimeError("Wrong texture argument!".to_owned()))
                };
                this.draw
                    .texture(&texture.graphics.view().build())
                    .w_h(w, h)
                    .x_y(x, y)
                    .x_radians(layout.radians.x)
                    .y_radians(layout.radians.y)
                    .z_radians(layout.radians.z)
                    .area(Rect{x: Range::new(layout.area_x.0, layout.area_x.1),  y: Range::new(layout.area_y.0, layout.area_y.1)})
                    .finish();
                Ok(())
            },
        );
        methods.add_method(
            "draw_rect",
            |_lua, this, polygon_options: LuaPolygonOptions| {
                this.draw
                    .rect()
                    .x_y(polygon_options.position.x, polygon_options.position.y)
                    .w_h(polygon_options.width, polygon_options.height)
                    .color::<LinearRrgba>(polygon_options.color.unwrap_or_default().into())
                    .x_radians(polygon_options.radians.x)
                    .y_radians(polygon_options.radians.y)
                    .z_radians(polygon_options.radians.z)
                    .polygon_options(polygon_options.into())
                    .finish();
                Ok(())
            },
        );
        methods.add_method(
            "draw_ellipse",
            |_lua, this, polygon_options: LuaPolygonOptions| {
                this.draw
                    .ellipse()
                    .x_y(polygon_options.position.x, polygon_options.position.y)
                    .w_h(polygon_options.width, polygon_options.height)
                    .color::<LinearRrgba>(polygon_options.color.unwrap_or_default().into())
                    .x_radians(polygon_options.radians.x)
                    .y_radians(polygon_options.radians.y)
                    .z_radians(polygon_options.radians.z)
                    .polygon_options(polygon_options.into())
                    .finish();
                Ok(())
            },
        );
        methods.add_method(
            "draw_arrow",
            |_lua,
             this,
             (x1, y1, x2, y2, w, head_width, head_length, tolerance, color, polygon_options): (
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                LuaColor,
                LuaPolygonOptions,
            )| {
                let stroke = polygon_options.stroke.unwrap_or_default();
                this.draw
                    .arrow()
                    .points(pt2(x1, y1), pt2(x2, y2))
                    .weight(w)
                    .head_length(head_length)
                    .head_width(head_width)
                    .color::<LinearRrgba>(color.into())
                    .x_radians(polygon_options.radians.x)
                    .y_radians(polygon_options.radians.y)
                    .z_radians(polygon_options.radians.z)
                    .start_cap(stroke.start_cap)
                    .end_cap(stroke.end_cap)
                    .join(stroke.line_join)
                    .miter_limit(stroke.miter_limit)
                    .tolerance(tolerance)
                    .finish();
                Ok(())
            },
        );
        methods.add_method(
            "draw_line",
            |_lua,
             this,
             (x1, y1, x2, y2, w, tolerance, color, polygon_options): (
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                LuaColor,
                LuaPolygonOptions,
            )| {
                let stroke = polygon_options.stroke.unwrap_or_default();
                this.draw
                    .line()
                    .points(pt2(x1, y1), pt2(x2, y2))
                    .weight(w)
                    .color::<LinearRrgba>(color.into())
                    .x_radians(polygon_options.radians.x)
                    .y_radians(polygon_options.radians.y)
                    .z_radians(polygon_options.radians.z)
                    .start_cap(stroke.start_cap)
                    .end_cap(stroke.end_cap)
                    .join(stroke.line_join)
                    .miter_limit(stroke.miter_limit)
                    .tolerance(tolerance)
                    .finish();
                Ok(())
            },
        );
        methods.add_method(
            "draw_triangle",
            |_lua,
             this,
             (x1, y1, x2, y2, x3, y3, color, polygon_options): (
                f32,
                f32,
                f32,
                f32,
                f32,
                f32,
                LuaColor,
                LuaPolygonOptions,
            )| {
                let stroke = polygon_options.stroke.unwrap_or_default();
                this.draw
                    .tri()
                    .points(pt2(x1, y1), pt2(x2, y2), pt2(x3, y3))
                    .color::<LinearRrgba>(color.into())
                    .x_radians(polygon_options.radians.x)
                    .y_radians(polygon_options.radians.y)
                    .z_radians(polygon_options.radians.z)
                    .start_cap(stroke.start_cap)
                    .end_cap(stroke.end_cap)
                    .join(stroke.line_join)
                    .miter_limit(stroke.miter_limit)
                    .finish();
                Ok(())
            },
        );

        methods.add_method(
            "draw_polygon",
            |_lua, this, (points, color, radians): (Vec<LuaPoint<f32>>, LuaColor, LuaRadians)| {
                let points: Vec<Point2> = points.iter().map(|x| pt2(x.x, x.y)).collect();
                this.draw
                    .polygon()
                    .points(points)
                    .color::<LinearRrgba>(color.into())
                    .x_radians(radians.x)
                    .y_radians(radians.y)
                    .z_radians(radians.z)
                    .finish();
                Ok(())
            },
        );

        methods.add_method(
            "draw_quad",
            |_lua,
             this,
             (points, color, polygon_options): (
                [LuaPoint<f32>; 4],
                LuaColor,
                LuaPolygonOptions,
            )| {
                let stroke = polygon_options.stroke.unwrap_or_default();
                this.draw
                    .quad()
                    .points(
                        pt2(points[0].x, points[0].y),
                        pt2(points[1].x, points[1].y),
                        pt2(points[2].x, points[2].y),
                        pt2(points[3].x, points[3].y),
                    )
                    .color::<LinearRrgba>(color.into())
                    .x_radians(polygon_options.radians.x)
                    .y_radians(polygon_options.radians.y)
                    .z_radians(polygon_options.radians.z)
                    .start_cap(stroke.start_cap)
                    .end_cap(stroke.end_cap)
                    .join(stroke.line_join)
                    .miter_limit(stroke.miter_limit)
                    .finish();
                Ok(())
            },
        );
        methods.add_method(
            "draw_polyline",
            |_lua, this, (points, color, radians): (Vec<LuaPoint<f32>>, LuaColor, LuaRadians)| {
                let points: Vec<Point2> = points.iter().map(|x| pt2(x.x, x.y)).collect();
                this.draw
                    .polyline()
                    .points_closed(points)
                    .color::<LinearRrgba>(color.into())
                    .x_radians(radians.x)
                    .y_radians(radians.y)
                    .z_radians(radians.z)
                    .finish();
                Ok(())
            },
        );
        methods.add_method("fill_background", |_lua, this, color: LuaColor| {
            this.draw.background().color::<LinearRrgba>(color.into());
            Ok(())
        });
        methods.add_method(
            "draw_points",
            |_lua, this, (colored_points, radians): (Vec<LuaColoredPoint>, LuaRadians)| {
                let points: Vec<(Point3, LinearRrgba)> = colored_points
                    .iter()
                    .map(|p| (pt3(p.p.x, p.p.y, 0.0), p.c.into()))
                    .collect();
                this.draw
                    .point_mode()
                    .mesh()
                    .points_colored(points)
                    .x_radians(radians.x)
                    .y_radians(radians.y)
                    .z_radians(radians.z)
                    .finish();
                Ok(())
            },
        );
    }
}
