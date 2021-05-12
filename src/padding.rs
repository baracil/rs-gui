
#[derive(Copy, Clone)]
pub struct Padding {
    pub top:f32,
    pub right:f32,
    pub bottom:f32,
    pub left:f32,
}

impl PartialEq for Padding {
    fn eq(&self, other: &Self) -> bool {
        return self.top == other.top && self.right == other.right && self.bottom == other.bottom && self.left == other.left
    }
}

impl Padding {

    pub fn same(padding:f32) -> Self {
        Padding::new(padding,padding,padding,padding)
    }

    pub fn same_on_axis(v_padding:f32, h_padding:f32) -> Self {
        Padding::new(v_padding,h_padding,v_padding,h_padding)
    }

    pub fn new(top:f32, right:f32, bottom:f32, left:f32) -> Self {
        Self{top,right,bottom,left}
    }

    pub fn none() -> Self {
        Padding::same(0.0)
    }

    pub fn h_padding(&self) -> f32 {
        self.left+self.right
    }

    pub fn v_padding(&self) -> f32 {
        self.top+self.bottom
    }

}