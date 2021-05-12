use raylib::math::{Rectangle};

use std::cell::{Cell, RefCell};
use crate::size::{Size, CachedSize};

/// Geometry of the widget obtained from the information contained in the model
pub struct WidgetGeometry {
    /// Computed size of the widget based on content and preferred size (padding is included)
    pub computed_size: Cell<Size>,

    /// Size of the content base on the available space provided
    pub widget_size: RefCell<CachedSize>,

    /// The position and size of the widget in relative coordinate to the parent
    pub widget_layout: Cell<Rectangle>,
    /// The position and size of the content (same as widget_layout but without padding)
    pub content_layout: Cell<Rectangle>,

}

impl WidgetGeometry {
    pub(crate) fn new() -> Self {
        Self {
            content_layout: Cell::new(Default::default()),
            widget_size: RefCell::new(Default::default()),
            widget_layout: Cell::new(Default::default()),
            computed_size: Cell::new(Default::default()),
        }
    }

    pub(crate) fn copy_size(source: &Size, target: &Cell<Rectangle>) {
        let mut target_layout = target.get();
        target_layout.width = source.width();
        target_layout.height = source.height();
        target.set(target_layout);
    }



}
