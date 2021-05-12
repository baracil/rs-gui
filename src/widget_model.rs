use crate::padding::Padding;
use std::cell::{Cell, RefCell};
use crate::fill::Fill;
use crate::size::Size;
use crate::fill::Fill::Disabled;
use crate::alignment::Alignment;
use crate::position::Position;

pub struct WidgetModel {

    pub fill_height: Cell<Fill>,
    pub fill_width: Cell<Fill>,

    pub position: Cell<Position>,

    pub text_style_name: RefCell<String>,
    pub back_style_name: RefCell<String>,
    pub border_style_name: RefCell<String>,

    pub preferred_size: Cell<Size>,

    pub focusable: Cell<bool>,
    pub clickable: Cell<bool>,
    pub hooverable: Cell<bool>,
    pub disable: Cell<bool>,

    pub padding: Cell<Padding>,

    pub action_id: RefCell<Option<String>>,

    /// alignment to the target
    pub alignment: Cell<Alignment>,

}

impl WidgetModel {
    pub(crate) fn new() -> Self {
        Self {
            position:Cell::new(Default::default()),
            fill_height: Cell::new(Disabled),
            fill_width: Cell::new(Disabled),
            preferred_size: Cell::new(Default::default()),
            text_style_name: RefCell::new("default".to_string()),
            back_style_name: RefCell::new("default".to_string()),
            border_style_name: RefCell::new("default".to_string()),
            alignment: Cell::new(Default::default()),
            focusable: Cell::new(false),
            hooverable: Cell::new(false),
            clickable: Cell::new(false),
            disable: Cell::new(false),
            padding: Cell::new(Padding::none()),
            action_id: RefCell::new(None),
        }
    }
}
