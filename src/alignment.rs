use crate::alignment::HAlignment::Middle;
use crate::alignment::VAlignment::Center;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum HAlignment {
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alignment {
    pub vertical: VAlignment,
    pub horizontal: HAlignment,
}

impl Alignment {
    pub fn new() -> Self {
        Self {
            vertical: Center,
            horizontal: Middle,
        }
    }
}

impl VAlignment {
    pub fn shift_factor(&self) -> f32 {
        match self {
            VAlignment::Top => 0.,
            VAlignment::Center => -0.5,
            VAlignment::Bottom => -1.0,
        }
    }
}

impl HAlignment {
    pub fn shift_factor(&self) -> f32 {
        match self {
            HAlignment::Right => -1.0,
            HAlignment::Middle => -0.5,
            HAlignment::Left => 0.0,
        }
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment{vertical:Center,horizontal:Middle}
    }
}