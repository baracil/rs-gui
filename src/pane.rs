use raylib::prelude::*;

use crate::gui::{Gui};
use crate::widget_data::{WidgetData};
use crate::widget_operation::{ LayoutableWidget, WidgetSpecific};
use crate::size::{Size};
use crate::position::Coordinate::{Absolute};
use std::ops::Deref;
use crate::mouse::MouseState;

pub struct PanePar {
    widget_data: WidgetData,

}

impl PanePar {
    pub fn new() -> Self {
        Self { widget_data: WidgetData::new() }
    }
}

impl Deref for PanePar {
    type Target = WidgetData;

    fn deref(&self) -> &Self::Target {
        &self.widget_data
    }
}

impl WidgetSpecific for PanePar {

    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.widget_data
    }

    fn compute_size(&self, gui: &Gui) -> Size {
        let tree_index = self.get_tree_index();
        if tree_index.is_none() {
            return Size::empty();
        }
        let tree_index = tree_index.unwrap();

        let mut xmin: f32 = 0.0;
        let mut xmax: f32 = 0.0;
        let mut ymin: f32 = 0.0;
        let mut ymax: f32 = 0.0;
        let mut max_size = Size::empty();
        let mut first_x = true;
        let mut first_y = true;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                let preferred = w.compute_computed_size(gui);
                let target = w.position();

                match (target.get_x(), w.fill_width_enabled(), first_x) {
                    (Absolute(value), false, true) => {
                        xmin = *value;
                        xmax = *value + preferred.width();
                        first_x = false
                    }
                    (Absolute(value), false, false) => {
                        xmin = xmin.min(*value);
                        xmax = xmax.max(*value + preferred.width());
                    }
                    (_, _, _) => {
                        max_size.max_width_mut(&preferred);
                    }
                }

                match (target.get_y(), w.fill_height_enabled(), first_y) {
                    (Absolute(value), false, true) => {
                        ymin = *value;
                        ymax = *value + preferred.height();
                        first_y = false
                    }
                    (Absolute(value), false, false) => {
                        ymin = ymin.min(*value);
                        ymax = ymax.max(*value + preferred.height());
                    }
                    (_, _, _) => {
                        max_size.max_height_mut(&preferred);
                    }
                }
            }
        }

        let pref_width = (xmax - xmin).max(max_size.width());
        let pref_height = (ymax - ymin).max(max_size.height());

        let children_size = Size::new(pref_width, pref_height).with_padding(&self.padding());

        let mut user_preferred_size = self.preferred_size();

        user_preferred_size.replace_empty_dimensions_and_max(&children_size);
        user_preferred_size
    }

    fn compute_child_content_size(&self, gui: &Gui, available_size: Size) {
        let tree_index = self.get_tree_index();
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let available_size_for_children = available_size.without_padding(&self.padding());

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                w.update_content_size(gui, &available_size_for_children);
            }
        }
    }

    fn compute_child_positions(&self, gui: &Gui) {
        let tree_index = self.get_tree_index();
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let content_size = {
            let content_layout = self.content_layout();
            Size::new(content_layout.width, content_layout.height)
        };

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {

                w.compute_default_target(&content_size);
                w.update_child_positions(gui)
            }
        }


    }

    fn render_my_visual(&self, _gui: &Gui, _d: &mut impl RaylibDraw, _offset: &Vector2) {}

    fn update_action(&self, gui: &Gui, offset: &Vector2, mouse_state: &MouseState) {
        self.widget_data.wd_update_action(gui,offset,mouse_state);
    }

}

