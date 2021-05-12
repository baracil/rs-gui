use std::cell::Cell;
use std::ops::BitAnd;
use std::ops::Deref;

use generational_arena::Index;
use raylib::prelude::*;
use std::ops::Add;
use crate::alignment::{HAlignment, VAlignment};
use crate::fill::Fill;
use crate::fill::Fill::{Disabled, Enabled};
use crate::gui::{Gui};
use crate::mouse::MouseState;
use crate::padding::Padding;
use crate::size::Size;
use crate::widget::Widget;
use crate::widget_geometry::WidgetGeometry;
use crate::widget_model::WidgetModel;
use crate::widget_operation::{DirtyFlags, LayoutableWidget, WidgetSpecific};
use crate::widget_state::WidgetState;
use crate::background::BackgroundRenderer;
use crate::border::BorderRenderer;
use crate::position::{Coordinate, Position};
use crate::event::Event::{Click};
use crate::text_style::TextStyle;
use std::rc::Rc;
use crate::event::ClickPar;

pub struct WidgetData {
    pub tree_index: Option<Index>,
    pub state: WidgetState,
    pub geometry: WidgetGeometry,
    pub model: WidgetModel,
}


impl WidgetData {
    pub(crate) fn render_background_and_border(&self, d: &mut impl RaylibDraw, offset: &Vector2) {
        let mut chrome_layout = self.widget_layout();
        chrome_layout.x += offset.x;
        chrome_layout.y += offset.y;
        {
            let borrowed_background = self.state.background.borrow();
            if let Some(background) = borrowed_background.as_deref() {
                let armed = self.state.armed.get();
                let hoovered = self.state.hoovered.get();
                let child_hoovered = self.state.child_hoovered.get();
                background.draw(d, &chrome_layout, hoovered && !child_hoovered, armed)
            }
        }
        {
            let borrowed_border = &self.state.border.borrow();
            if let Some(border) = borrowed_border.as_deref() {
                border.draw(d, &chrome_layout)
            }
        }
    }

    pub(crate) fn render_children(&self, gui:&Gui,tree_index:Index, d:&mut impl RaylibDraw, offset:&Vector2) {
        let content_layout = self.content_layout();
        let mut target = offset.clone();
        target.x += content_layout.x;
        target.y += content_layout.y;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                w.render(gui, d, &target);
            }
        }
    }
}


///mode private
impl WidgetData {

    fn disable_fill(&self, gui: &Gui, fill: &Cell<Fill>) {
        if fill.get().is_disabled() {
            return;
        }
        fill.set(Disabled);
        self.invalidate_preferred_size(gui)
    }

    fn enable_fill(&self, gui: &Gui, fill_cell: &Cell<Fill>, fill: Fill) {
        let current_fill = fill_cell.get();
        if current_fill.eq(&fill) {
            return;
        }

        fill_cell.set(fill);
        self.invalidate_preferred_size(gui)
    }


}

///model public
impl WidgetData {
    pub fn action_id<>(&self) -> Option<String> {
        let borrowed = self.model.action_id.borrow();

        match borrowed.as_ref() {
            None => None,
            Some(r) => Some(r.to_owned())
        }
    }
    pub fn set_action_id(&self, action_id: &str) -> &WidgetData {
        self.model.action_id.replace(Some(action_id.to_string()));
        self
    }
    pub fn clear_action_id(&self) -> &WidgetData {
        self.model.action_id.replace(None);
        self
    }

    pub fn preferred_size(&self) -> Size {
        self.model.preferred_size.get()
    }
    pub fn set_preferred_height(&self, gui: &Gui, height: f32) -> &WidgetData {
        let size = self.model.preferred_size.get().with_height(height);
        self.set_preferred_size(gui, size);
        self
    }
    pub fn set_preferred_width(&self, gui: &Gui, width: f32) -> &WidgetData {
        let size = self.model.preferred_size.get().with_width(width);
        self.set_preferred_size(gui, size);
        self
    }
    pub fn set_preferred_size(&self, gui: &Gui, size: Size) -> &WidgetData {
        let current = self.model.preferred_size.get();
        if current.eq(&size) {
            return self;
        }
        self.model.preferred_size.set(size);
        self.invalidate_preferred_size(gui);
        self
    }

    pub fn padding(&self) -> Padding {
        self.model.padding.get()
    }
    pub fn set_padding(&self, gui: &Gui, padding: Padding) -> &WidgetData {
        let current_padding = self.model.padding.get();
        if current_padding.eq(&padding) {
            return self;
        }
        self.model.padding.set(padding);
        self.invalidate_preferred_size(gui);
        self
    }

    pub fn fill_height_enabled(&self) -> bool {
        self.model.fill_height.get().is_enabled()
    }
    pub fn fill_width_enabled(&self) -> bool {
        self.model.fill_width.get().is_enabled()
    }
    pub fn fill_width(&self) -> Fill {
        self.model.fill_width.get()
    }
    pub fn fill_height(&self) -> Fill {
        self.model.fill_height.get()
    }
    pub fn disable_fill_width(&self, gui: &Gui) -> &WidgetData {
        self.disable_fill(gui, &self.model.fill_width);
        self
    }
    pub fn disable_fill_height(&self, gui: &Gui) -> &WidgetData {
        self.disable_fill(gui, &self.model.fill_height);
        self
    }
    pub fn enable_fill_width(&self, gui: &Gui, fill: Fill) -> &WidgetData {
        self.enable_fill(gui, &self.model.fill_width, fill);
        self
    }
    pub fn enable_fill_height(&self, gui: &Gui, fill: Fill) -> &WidgetData {
        self.enable_fill(gui, &self.model.fill_height, fill);
        self
    }

    pub fn set_text_style(&self, text_style_name: &str) -> &WidgetData {
        self.model.text_style_name.replace(text_style_name.to_string());
        self.invalidate_style();
        self
    }
    pub fn set_background_style(&self, background_style_name: &str) -> &WidgetData {
        self.model.back_style_name.replace(background_style_name.to_string());
        self.invalidate_style();
        self
    }
    pub fn set_border_style(&self, border_style: &str) -> &WidgetData {
        self.model.border_style_name.replace(border_style.to_string());
        self.invalidate_style();
        self
    }

    pub fn position(&self) -> Position {
        self.model.position.get()
    }
    pub fn set_position(&self, gui: &Gui, x: &Coordinate, y: &Coordinate) -> &WidgetData {
        let mut current_position = self.model.position.get();
        if current_position.get_x().eq(&x) && current_position.get_y().eq(&y) {
            return self;
        }
        current_position.set_x(x);
        current_position.set_y(y);
        self.model.position.set(current_position);
        self.invalidate_position(gui);
        self
    }
    pub fn set_valignment(&self, gui: &Gui, valignment: VAlignment) -> &WidgetData {
        let current_alignment = self.model.alignment.get();
        self.set_alignment(gui, valignment, current_alignment.horizontal);
        self
    }
    pub fn set_halignment(&self, gui: &Gui, halignment: HAlignment) -> &WidgetData {
        let current_alignment = self.model.alignment.get();
        self.set_alignment(gui, current_alignment.vertical, halignment);
        self
    }
    pub fn set_alignment(&self, gui: &Gui, valignment: VAlignment, haligment: HAlignment) -> &WidgetData{
        let mut current_alignment = self.model.alignment.get();
        if current_alignment.vertical.eq(&valignment) && current_alignment.horizontal.eq(&haligment) {
            return self;
        }
        current_alignment.vertical = valignment;
        current_alignment.horizontal = haligment;
        self.model.alignment.set(current_alignment);
        self.invalidate_position(gui);
        self
    }

    pub fn set_clickable(&self, clickable: bool) -> &WidgetData {
        self.model.clickable.set(clickable);
        self
    }
    pub fn hooverable(&self) -> bool {
        self.model.hooverable.get()
    }
    pub fn set_hooverable(&self, hooverable:bool) -> &WidgetData {
        self.model.hooverable.set(hooverable);
        self
    }

}

/// geometry
impl WidgetData {
    pub fn widget_layout(&self) -> Rectangle {
        self.geometry.widget_layout.get()
    }
    pub fn widget_height(&self) -> f32 {
        self.geometry.widget_size.borrow().size().height()
    }
    pub fn widget_width(&self) -> f32 {
        self.geometry.widget_size.borrow().size().width()
    }

    pub fn content_layout(&self) -> Rectangle {
        self.geometry.content_layout.get()
    }

}

impl WidgetData {

    pub fn get_tree_index(&self) -> Option<Index> {
        self.tree_index
    }

    fn get_parent<'a>(&self, gui: &'a Gui) -> Option<&'a Widget> {
        match self.tree_index {
            None => None,
            Some(idx) => {
                gui.get_parent_widget(idx)
            }
        }
    }



    pub fn text_style(&self) -> Option<Rc<TextStyle>> {
        let borrowed_text_style = self.state.text_style.borrow();
        match borrowed_text_style.as_ref() {
            None => None,
            Some(p) => Some(p.clone())
        }
    }


    pub fn update_style(&self, gui: &Gui) {
        if self.dirty_flag_clean(DirtyFlags::STYLE) {
            return;
        }

        self.update_text_style(gui);
        self.update_background(gui);
        self.update_border(gui);

        self.invalidate_preferred_size(gui)
    }

    fn update_text_style(&self, gui: &Gui) {
        let borrowed = self.model.text_style_name.borrow();
        let text_style = gui.get_text_style(borrowed.deref());
        self.state.text_style.replace(text_style);
    }

    fn update_background(&self, gui: &Gui) {
        let borrowed = self.model.back_style_name.borrow();
        let background = gui.get_background(borrowed.deref());
        self.state.background.replace(background);
    }

    fn update_border(&self, gui: &Gui) {
        let borrowed = self.model.border_style_name.borrow();
        let border = gui.get_border(borrowed.deref());
        self.state.border.replace(border);
    }

}

///dirty flags
impl WidgetData {

    pub fn invalidate_style(&self) {
        self.set_dirty_flag(DirtyFlags::STYLE)
    }

    pub fn invalidate_preferred_size(&self, gui: &Gui) {
        self.invalidate_flag(gui, DirtyFlags::PREFERRED_SIZE);
    }

    pub fn invalidate_content_size(&self, gui: &Gui) {
        self.invalidate_flag(gui, DirtyFlags::CONTENT_SIZE);
    }

    pub fn invalidate_position(&self, gui: &Gui) {
        self.invalidate_flag(gui, DirtyFlags::POSITION);
    }

    fn invalidate_flag(&self, gui: &Gui, flag: DirtyFlags) {
        if self.is_dirty_flag_set(flag) {
            return;
        }
        self.set_dirty_flag(flag);
        if let Some(parent) = self.get_parent(gui) {
            parent.invalidate_flag(gui, flag)
        }
    }

    pub fn set_dirty_flag(&self, flag: DirtyFlags) {
        self.state.dirty_flags.set(self.state.dirty_flags.get() | flag);
    }

    pub fn is_dirty_flag_set(&self, flag: DirtyFlags) -> bool {
        self.state.dirty_flags.get().bitand(flag).eq(&flag)
    }

    pub fn dirty_flag_clean(&self, flag: DirtyFlags) -> bool {
        self.state.dirty_flag_clean(flag)
    }

    pub fn dirty_flag_dirty(&self, flag: DirtyFlags) -> bool {
        !self.state.dirty_flag_clean(flag)
    }



}

impl WidgetData {
    pub fn new() -> Self {
        Self {
            tree_index: None,
            state: WidgetState::new(),
            geometry: WidgetGeometry::new(),
            model: WidgetModel::new(),
        }
    }



    pub fn set_tree_index(&mut self, tree_index: Index) {
        self.tree_index = Some(tree_index);
    }

    pub fn clear_tree_index(&mut self) {
        self.tree_index = None
    }



    pub fn update_hoovered(&self, gui: &Gui, offset: &Vector2, mouse_position: &Vector2) -> bool {
        let mut abs_widget_layout = self.widget_layout();
        abs_widget_layout.x += offset.x;
        abs_widget_layout.y += offset.y;

        let hooverable = self.hooverable();

        let new_hoovered = abs_widget_layout.check_collision_point_rec(mouse_position);
        let old_hoovered = self.get_hoover_state();
        self.set_hoover_state(new_hoovered && hooverable);
        let mut child_hoovered = false;

        match (self.tree_index, old_hoovered, new_hoovered) {
            (_, false, false) | (None, _, _) => {}

            (Some(idx), _, _) => {
                let padding = self.padding();
                let child_offset = Vector2::new(abs_widget_layout.x+padding.left, abs_widget_layout.y+padding.top);
                for child_index in gui.get_widget_children(idx) {
                    if let Some(w) = gui.get_widget(child_index) {
                        child_hoovered |= w.update_hoovered(gui, &child_offset, mouse_position)
                    }
                }
            }
        }
        self.set_child_hoovered_state(child_hoovered);
        new_hoovered
    }

    pub fn wd_update_action(&self, gui:&Gui, offset: &Vector2, mouse_state: &MouseState)  {
        let mut armed = self.state.armed.get();
        let clickable = self.model.clickable.get();
        let hoovered = self.state.hoovered.get();

        if mouse_state.left().is_released() && clickable && hoovered {
            let action_id = self.model.action_id.clone().into_inner();
            match (armed, action_id) {
                (true, Some(action_id)) => {
                    gui.add_event(Click(ClickPar::new(&action_id)))
                }
                _ => {}
            }
        }

        if mouse_state.left().is_pressed() && hoovered {
            armed = clickable;
        }

        armed &= mouse_state.left().is_down();

        self.state.armed.set(armed);

        if let Some(idx) = self.tree_index {
            for child_index in gui.get_widget_children(idx) {
                let content_layout = self.content_layout();
                let child_offset = Vector2::new(content_layout.x+offset.x, content_layout.y+offset.y);
                if let Some(w) = gui.get_widget(child_index) {
                    w.update_action(gui,&child_offset,mouse_state);
                }

            }
        }


    }

    pub fn copy_size_to_layout(&self) {
        let borrowed_widget_size = self.geometry.widget_size.borrow();
        let widget_size = borrowed_widget_size.size();

        {
            WidgetGeometry::copy_size(widget_size, &self.geometry.widget_layout);
            let content_size = widget_size.without_padding(&self.model.padding.get());
            WidgetGeometry::copy_size(&content_size, &self.geometry.content_layout);
        }
    }

    pub fn compute_default_target(&self, available_size: &Size) {
        let position = self.model.position.get().compute_absolute(available_size);
        let offset = self.compute_alignment_offset();

        self.set_widget_target(&position.add(offset));
    }

    pub fn set_widget_target(&self, target: &Vector2) {
        let padding = self.model.padding.get();
        let mut widget_layout = self.geometry.widget_layout.get();
        let mut content_layout = self.geometry.content_layout.get();

        widget_layout.x = target.x;
        widget_layout.y = target.y;

        content_layout.x = target.x + padding.left;
        content_layout.y = target.y + padding.top;

        self.geometry.widget_layout.set(widget_layout);
        self.geometry.content_layout.set(content_layout);
    }

    pub fn compute_alignment_offset(&self) -> Vector2 {
        let borrowed_widget_size = self.geometry.widget_size.borrow();
        let widget_size = borrowed_widget_size.size();
        let alignment = self.model.alignment.get();

        let mut offset = Vector2::default();

        offset.x = widget_size.width() * alignment.horizontal.shift_factor();
        offset.y = widget_size.height() * alignment.vertical.shift_factor();

        offset
    }
}


impl<W: WidgetSpecific> LayoutableWidget for W {

    fn compute_computed_size(&self, gui: &Gui) -> Size {
        if self.widget_data().dirty_flag_dirty(DirtyFlags::PREFERRED_SIZE) {
            let size = self.compute_size(gui);
            let old_size = self.widget_data().geometry.computed_size.replace(size);
            if size.ne(&old_size) {
                self.widget_data().invalidate_content_size(gui);
            }
            return size;
        }
        return self.widget_data().geometry.computed_size.get();
    }
    fn update_content_size(&self, gui: &Gui, available_space: &Size) {
        let content_invalid = {
            let content_cache = self.widget_data().geometry.widget_size.borrow();
            let clean_flag = self.widget_data().dirty_flag_clean(DirtyFlags::CONTENT_SIZE);
            let cache_valid = available_space.eq(content_cache.reference());
            !clean_flag || !cache_valid
        };

        if content_invalid {
            let mut content_size = self.widget_data().geometry.computed_size.get();

            if let Enabled { .. } = self.widget_data().model.fill_width.get() {
                content_size.set_width(available_space.width())
            }
            if let Enabled { .. } = self.widget_data().model.fill_height.get() {
                content_size.set_height(available_space.height())
            }
            content_size.min_mut(&available_space);

            {
                let mut content_cache = self.widget_data().geometry.widget_size.borrow_mut();
                content_cache.set_reference(available_space.clone());
                content_cache.set_size(content_size);
            }

            self.compute_child_content_size(gui, content_size);
            self.widget_data().copy_size_to_layout();
            self.widget_data().invalidate_position(gui);
        }
    }
    fn update_child_positions(&self, gui: &Gui) {
        if self.widget_data().state.dirty_flag_clean(DirtyFlags::POSITION) {
            return;
        }
        self.compute_child_positions(gui);
    }
}

/// state
impl WidgetData {

    pub fn get_hoover_state(&self) -> bool {
        self.state.hoovered.get()
    }

    pub fn set_hoover_state(&self, hoovered:bool) -> &WidgetData {
        self.state.hoovered.set(hoovered);
        self
    }

    pub fn set_child_hoovered_state(&self, hoovered:bool) -> &WidgetData {
        self.state.child_hoovered.set(hoovered);
        self
    }

    pub fn get_child_hoovered_state(&self) -> bool {
        self.state.child_hoovered.get()
    }

}
