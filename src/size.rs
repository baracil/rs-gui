use crate::padding::Padding;
use raylib::prelude::Vector2;

#[derive(Copy, Clone,Default,Debug)]
pub struct Size {
    width: f32,
    height: f32,
}

impl PartialEq<Size> for Size {
    fn eq(&self, other: &Size) -> bool {
        self.width == other.width && self.height == other.height
    }

}

impl Eq for Size {
}

impl Size {
    pub fn with_height(&self, height: f32) -> Size {
        Size{width:self.width, height:height.max(0.0)}
    }

    pub fn with_width(&self, width: f32) -> Size {
        Size{width:width.max(0.0), height:self.height}
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn max(&self, other:&Size) -> Size {
        Self{width:self.width.max(other.width),height:self.height.max(other.height)}
    }

    pub fn min(&self, other:&Size) -> Size {
        Self{width:self.width.min(other.width),height:self.height.min(other.height)}
    }

    pub fn max_width_mut(&mut self, other:&Size) -> &mut Size {
        let width = self.width;
        self.width = width.max(other.width);
        self
    }

    pub fn max_height_mut(&mut self, other:&Size) -> &mut Size {
        let height= self.height;
        self.height = height.max(other.height);
        self
    }

    pub fn max_mut(&mut self, other:&Size) -> &mut Size {
        let width = self.width;
        let height= self.height;
        self.width = width.max(other.width);
        self.height = height.max(other.height);
        self
    }

    pub fn min_mut(&mut self, other:&Size) -> &mut Size {
        let width = self.width;
        let height= self.height;
        self.width = width.min(other.width);
        self.height = height.min(other.height);
        self
    }

    pub fn with_replaced_empty_dimensions(&self, other:&Size) -> Size {
        let width = if self.width <= 0.0 {other.width} else {self.width};
        let height = if self.height <= 0.0 {other.height} else {self.height};
        Size{width,height}
    }

    pub fn replace_empty_dimensions(&mut self, other:&Size) -> &mut Size {
        let width = if self.width <= 0.0 {other.width} else {self.width};
        let height = if self.height <= 0.0 {other.height} else {self.height};
        self.width = width;
        self.height = height;
        self
    }

    pub fn replace_empty_dimensions_and_max(&mut self, other:&Size) -> &mut Size {
        let width = if self.width <= 0.0 {other.width} else {self.width};
        let height = if self.height <= 0.0 {other.height} else {self.height};
        self.width = width.max(other.width);
        self.height = height.max(other.height);
        self
    }

    pub fn new(width:f32, height:f32) -> Self {
        Size{width:width.max(0.0), height:height.max(0.0)}
    }

    pub fn from_vector2(vector:&Vector2) -> Self {
        Size{width:vector.x, height:vector.y}
    }

    pub fn empty() -> Self {
        Size::new(0.0,0.0)
    }

    pub fn is_empty(&self) -> bool {
        return self.width<=0.0 || self.height<=0.0
    }

    pub fn without_padding(&self, padding:&Padding) -> Size {
        let width = self.width - padding.h_padding();
        let height = self.height - padding.v_padding();
        return Size::new(width,height)
    }

    pub fn with_padding(&self, padding:&Padding) -> Size {
        let width = self.width + padding.h_padding();
        let height = self.height + padding.v_padding();
        return Size::new(width,height)
    }

    pub fn width_border(&self, border:f32) -> Size {
        Size::new(self.width+2.0*border, self.height+2.0*border)
    }

    pub fn set_width(&mut self, width:f32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height:f32) {
        self.height = height;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct CachedSize {
    size:Size,
    reference:Size
}

impl CachedSize {
    pub fn new(size:Size, reference:Size) -> Self {
        Self{size,reference}
    }

    pub fn as_reference(&self, other:&Size) -> bool {
        self.reference.eq(other)
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn reference(&self) -> &Size {
        &self.reference
    }

    pub fn set_reference(&mut self, size:Size) {
        self.reference = size;
    }

    pub fn set_size(&mut self, size:Size) {
        self.size = size;
    }
}