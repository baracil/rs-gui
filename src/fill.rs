use crate::fill::Fill::Disabled;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum  Fill {
    Disabled,
    Enabled{
        weight:u32
    },
}

impl Fill {
}

impl Fill {

    pub fn is_disabled(&self) -> bool {
        matches!(self,Disabled)
    }

    pub fn is_enabled(&self) -> bool {
        !self.is_disabled()
    }

    pub fn get_weight(&self) -> u32 {
        match self {
            Disabled => 0,
            Fill::Enabled {weight} => *weight
        }
    }

}
