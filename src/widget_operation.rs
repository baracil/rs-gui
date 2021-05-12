use raylib::prelude::*;

use crate::gui::{Gui};
use crate::mouse::MouseState;
use crate::size::Size;
use crate::widget_data::WidgetData;

bitflags! {
    pub struct DirtyFlags: u32 {
        const STYLE = 1;
        const PREFERRED_SIZE = 2;
        const CONTENT_SIZE = 4;
        const POSITION = 8;

        const ALL = 15;
    }
}

pub trait WidgetSpecific {
    fn widget_data(&self) -> &WidgetData;
    fn widget_data_mut(&mut self) -> &mut WidgetData;

    fn compute_size(&self, gui:&Gui) -> Size;
    fn compute_child_content_size(&self, gui:&Gui, available_size:Size);
    fn compute_child_positions(&self, gui:&Gui);

    fn update_action(&self, gui:&Gui, offset: &Vector2, mouse_state: &MouseState);
    fn render_my_visual(&self, gui:&Gui, d:&mut impl RaylibDraw, offset:&Vector2);

    fn render(&self, gui: &Gui, d: &mut impl RaylibDraw, offset: &Vector2) {
        let widget_data = self.widget_data();
        let tree_index = widget_data.get_tree_index();
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        self.widget_data().render_background_and_border(d, offset);
        self.render_my_visual(gui,d,offset);
        self.widget_data().render_children(gui, tree_index, d, offset);
    }


}

pub trait LayoutableWidget {
    fn compute_computed_size(&self, gui: &Gui) -> Size;
    fn update_content_size(&self, gui: &Gui, available_space: &Size);
    fn update_child_positions(&self, gui:&Gui);
}
