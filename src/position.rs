use raylib::math::Vector2;
use crate::size::Size;
use crate::position::Coordinate::{Absolute, Relative};

#[derive(Copy, Clone,Eq, PartialEq)]
pub struct Position {
    x:Coordinate,
    y:Coordinate,
}

#[derive(Copy, Clone)]
pub enum Coordinate {
    Absolute(f32),
    Relative(f32)
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Absolute(value1), Absolute (value2)) => value1==value2,
            (Relative(value1), Relative (value2)) => value1==value2,
            (_,_) => false
        }
    }
}
impl Eq for Coordinate {

}


impl Coordinate {

    pub fn is_absolute(&self) -> bool {
        matches!(self,Absolute {..})
    }

    pub fn is_relative(&self) -> bool {
        matches!(self,Relative {..})
    }

    pub fn compute_absolute(&self,available_space:f32) -> f32 {
        match self {
            Coordinate::Absolute( value) => *value,
            Coordinate::Relative ( percent ) => percent*available_space*0.01
        }
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        return Absolute(0.0)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self{x:Default::default(),y:Default::default()}
    }
}

impl Position {

    pub fn new(x:Coordinate,y:Coordinate) -> Self {
        Self{x,y}
    }

    pub fn is_x_absolute(&self) -> bool {
        self.x.is_absolute()
    }
    pub fn is_y_absolute(&self) -> bool {
        self.y.is_absolute()
    }

    pub fn is_x_relative(&self) -> bool {
        self.x.is_relative()
    }
    pub fn is_y_relative(&self) -> bool {
        self.y.is_relative()
    }

    pub fn compute_absolute(&self, available_space:&Size) -> Vector2 {
        let x= self.x.compute_absolute(available_space.width());
        let y= self.y.compute_absolute(available_space.height());
        Vector2{x,y}
    }

    pub fn get_x(&self) -> &Coordinate {
        &self.x
    }

    pub fn get_y(&self) -> &Coordinate {
        &self.y
    }

    pub fn with_x(&self, x:Coordinate) -> Self {
        Self{x,y:self.y}
    }

    pub fn with_y(&self, y:Coordinate) -> Self {
        Self{x:self.x,y}
    }

    pub fn set(&mut self, pos:&Position) -> &mut Position {
        self.x = pos.x;
        self.y = pos.y;
        self
    }

    pub fn set_ex(&mut self, x:&Coordinate, y:&Coordinate) -> &mut Position {
        self.x = x.clone();
        self.y = y.clone();
        self
    }

    pub fn set_x(&mut self, x:&Coordinate) -> &mut Position {
        self.x = x.clone();
        self
    }

    pub fn set_y(&mut self, y:&Coordinate) -> &mut Position {
        self.y = y.clone();
        self
    }
}