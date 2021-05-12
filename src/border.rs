use raylib::prelude::*;

/// A border
pub trait BorderRenderer {
    fn draw(&self, d: &mut impl RaylibDraw, layout: &Rectangle);
}

pub enum Border {
    Empty,
    Line {
        color:Color,
        thickness:f32
    }
}


impl BorderRenderer for Border {
    fn draw(&self, d: &mut impl RaylibDraw, layout: &Rectangle) {
        match self {
            Border::Empty => {}
            Border::Line {color, thickness } => render_line_border(d, layout, color, *thickness)
        }
    }
}

fn render_line_border(d: &mut impl RaylibDraw, layout: &Rectangle, color: &Color, thickness: f32) {
    d.draw_rectangle_lines_ex(layout, thickness as i32, color)
}
