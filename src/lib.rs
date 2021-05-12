#[macro_use]
extern crate bitflags;

pub mod alignment;
// pub mod label;
pub mod mouse;
pub mod widget_operation;
pub mod widget_geometry;
pub mod widget_model;
pub mod widget_state;

pub mod font;
pub mod gui;
pub mod widget_data;
pub mod widget;
pub mod label;
pub mod pane;
pub mod vbox;
pub mod size;
pub mod padding;
pub mod border;
pub mod text_style;
pub mod fill;
pub mod background;
pub mod position;
pub mod event;
pub mod hbox;
pub mod slider;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
