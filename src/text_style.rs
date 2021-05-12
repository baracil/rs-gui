use raylib::prelude::*;
use std::rc::Rc;
use crate::size::Size;
use crate::font::FontInfo;

#[derive(Clone)]
pub struct TextStyle {
    font:Rc<FontInfo>,
    color:Color,
    spacing:f32,
}

impl TextStyle {
    pub fn new(font:Rc<FontInfo>, color:Color, spacing:f32) -> Self {
        Self{font:font.clone(), color, spacing}
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn spacing(&self) -> f32 {
        self.spacing
    }

    pub fn measure_text(&self, text: &str) -> Size {
        self.font.measure_text(text, self.spacing)
    }

    pub fn draw_text(&self, d: &mut impl RaylibDraw, text: &str, position: &Vector2) {
        self.font.draw_text(d,text, position,self.spacing,self.color);
    }


}