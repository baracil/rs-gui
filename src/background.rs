use raylib::prelude::*;

/// A background
pub trait BackgroundRenderer {
    fn draw(&self, d: &mut impl RaylibDraw, layout: &Rectangle, hoovered:bool, armed:bool);
}


pub enum Background {
    Empty,
    Solid {
        idle_color:Color,
        hoovered_color:Color,
        armed_color:Color
    }
}


impl BackgroundRenderer for Background {

    fn draw(&self, d: &mut impl RaylibDraw, layout: &Rectangle, hoovered:bool, armed:bool) {
        match self {
            Background::Empty => {}
            Background::Solid {idle_color,hoovered_color, armed_color} => {
                let color = match (hoovered,armed) {
                    (_,true) => armed_color,
                    (true,false) => hoovered_color,
                    _ => idle_color
                };
                render_solid_background(d,layout, color)
            }
        }
    }
}

fn render_solid_background(d: &mut impl RaylibDraw, layout: &Rectangle, color: &Color) {
    d.draw_rectangle_rec(layout,color)
}