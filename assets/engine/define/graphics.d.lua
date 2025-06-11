---@class Vec2
---@field x number
---@field y number

---@class Color
---@field r number
---@field g number
---@field b number
---@field a number

---@class RoundedRectRadii
---@field top_left number
---@field top_right number
---@field bottom_right number
---@field bottom_left number

---@class MoveTo
---@field tag "MoveTo"
---@field x number
---@field y number

---@class LineTo
---@field tag "LineTo"
---@field x number
---@field y number

---@class QuadTo
---@field tag "QuadTo"
---@field x1 number
---@field y1 number
---@field x number
---@field y number

---@class CurveTo
---@field tag "CurveTo"
---@field x1 number
---@field y1 number
---@field x2 number
---@field y2 number
---@field x number
---@field y number

---@class ClosePath
---@field tag "ClosePath"

---@class PathEl 
---@field MoveTo MoveTo|nil
---@field LineTo LineTo|nil
---@field QuadTo QuadTo|nil
---@field CurveTo CurveTo|nil
---@field ClosePath ClosePath| nil

---@class Ellipse
---@field center Point
---@field radii Vec2
---@field rotation number

---@class Circle
---@field center Point
---@field radius number
---@field rotation number

---@class Line
---@field p0 Point
---@field p1 Point

---@class Rect
---@field p0 Point
---@field size Size

---@class RoundedRect
---@field p0 Point
---@field size Size
---@field radii RoundedRectRadii

---@class Triangle
---@field a Point
---@field b Point
---@field c Point

---@class QuadBez
---@field a Point
---@field b Point
---@field c Point

---@class CubicBez
---@field a Point
---@field b Point
---@field c Point
---@field d Point

---@class BezPath
---@field elements PathEl[]

---@class Point
---@field pos Point

---@class Arc
---@field center Point
---@field radii Vec2
---@field start_angle number
---@field sweep_angle number
---@field rotation number

---@class Image
---@field position Point
---@field image string

---@class PointLight
---@field center Point
---@field radius number
---@field rotation number
---@field opacity number
---@field color Color

---@class LightMask
---@field screen_size Size
---@field lights { [1]: Point, [2]: number }[]  -- tuple: (Point, f64)
---@field darkness_alpha integer

---@class Text
---@field position Point
---@field text string

---@class SceneNodeKind
---@field Ellipse Ellipse|nil
---@field Circle Circle|nil
---@field Line Line|nil
---@field Rect Rect|nil
---@field RoundedRect RoundedRect|nil
---@field Triangle Triangle|nil
---@field QuadBez QuadBez|nil
---@field CubicBez CubicBez|nil
---@field BezPath BezPath|nil
---@field Point Point|nil
---@field Arc Arc|nil
---@field Image Image|nil
---@field PointLight PointLight|nil
---@field LightMask LightMask|nil
---@field Text Text|nil

---@class Join 
--- Bevel,
--- A straight line connecting the segments.
--- Miter
--- The segments are extended to their natural intersection point.
--- Round
--- An arc between the segments.

---@class Cap
--- Butt
--- Flat cap.
--- Square
--- Square cap with dimensions equal to half the stroke width.
--- Round
--- Rounded cap with radius equal to half the stroke width.

---@class Stoke
--- Width of the stroke.
---@field width number
--- Style for connecting segments of the stroke.
---@field join "Bevel" | "Miter" | "Round" | nil
--- Limit for miter joins.
---@field miter_limit number,
--- Style for capping the beginning of an open subpath.
---@field start_cap "Butt" | "Square" | "Round" | nil ,
--- Style for capping the end of an open subpath.
---@field end_cap "Butt" | "Square" | "Round" | nil,
--- Lengths of dashes in alternating on/off order.
---@field dash_pattern number[] vector4,
--- Offset of the first dash.
---@field dash_offset number,

---@class StokeStyle
---@field stoke Stoke
---@field brush Brush
---@class Solid
---@field components number[] 4

---@class Brush
---@field Solid Solid
---@class Affine number[] 6

---@class Style
---@field translation? Affine
---@field fill? Brush,
---@field fill_rule? "NonZero" | "EvenOdd"
---@field stoke? StokeStyle| nil
---@field opacity? number 0.0-1.0
---@field visible? boolean
---@field z_index? number
---@field tag? string | nil
--- // for text
---@field font? string | nil
---@field font_size? number | nil
---@field hint? boolean | nil
---@field align? "Left"| "Center" | "Right" | nil
---@field line_spacing? number | nil
---@field vertical? boolean | nil