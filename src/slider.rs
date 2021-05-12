use std::cell::{Cell, RefCell};
use crate::widget_data::WidgetData;
use crate::gui::Gui;
use crate::widget_operation::{WidgetSpecific};
use crate::size::Size;
use raylib::prelude::*;
use std::ops::Deref;
use crate::mouse::MouseState;
use crate::event::Event::Drag;
use crate::event::DragPar;

pub struct SliderPar {
    widget_data: WidgetData,
    value: Cell<f32>,
    value_min: Cell<f32>,
    value_max: Cell<f32>,
    value_text:RefCell<String>,
    value_text_size: Cell<Size>,
    drag_in_progress: Cell<bool>,
    drag_value: Cell<f32>,
    drag_starting_position: Cell<Vector2>,
    cursor_layout:Cell<Rectangle>
}

impl Deref for SliderPar {
    type Target = WidgetData;

    fn deref(&self) -> &Self::Target {
        &self.widget_data
    }
}

const SLIDER_BAR_HEIGHT: f32 = 20.0;
const SLIDER_BAR_WIDTH: f32 = 100.0;
const SLIDER_BAR_THICKNESS: f32 = 1.0;
const SLIDER_BAR_COLOR: Color = Color::GRAY;


const SLIDER_CURSOR_SPACING: f32 = 2.0;
const SLIDER_CURSOR_HEIGHT: f32 = 30.0;
const SLIDER_CURSOR_WIDTH: f32 = 5.0;
const SLIDER_CURSOR_THICKNESS: f32 = 1.0;
const SLIDER_CURSOR_COLOR: Color = Color::BLACK;

impl SliderPar {
    pub fn new() -> Self {
        Self {
            widget_data: WidgetData::new(),
            value: Cell::new(50.0),
            value_min: Cell::new(0.0),
            value_max: Cell::new(100.0),
            value_text: RefCell::new("".to_string()),
            value_text_size: Cell::new(Size::empty()),
            drag_in_progress: Cell::new(false),
            drag_value: Cell::new(50.0),
            drag_starting_position: Cell::new(Vector2::default()),
            cursor_layout:Cell::new(Rectangle::default()),
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value.get()
    }

    pub fn get_value_min(&self) -> f32 {
        self.value_min.get()
    }

    pub fn get_value_max(&self) -> f32 {
        self.value_max.get()
    }

    pub fn set_value(&self, gui: &Gui, value: f32) -> &SliderPar {
        self.value.set(value);
        self.invalidate_preferred_size(gui);
        self
    }

    pub fn set_value_min(&self, gui: &Gui, value: f32) -> &SliderPar {
        self.value_min.set(value);
        self.invalidate_preferred_size(gui);
        self
    }

    pub fn set_value_max(&self, gui: &Gui, value: f32) -> &SliderPar {
        self.value_max.set(value);
        self.invalidate_preferred_size(gui);
        self
    }


    fn format_value(&self, value:f32) -> String {
        format!("{:5.1}", value)
    }

    fn measure_value(&self, formatted_value: &str) -> Size {
        match self.text_style() {
            None => Size::empty(),
            Some(ts) => ts.measure_text(formatted_value)
        }
    }

    fn effective_value(&self) -> f32 {
        if self.drag_in_progress.get() {self.drag_value.get()} else {self.value.get()}
    }

}

impl WidgetSpecific for SliderPar {

    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.widget_data
    }

    fn compute_size(&self, _gui: &Gui) -> Size {
        let formatted_value = self.format_value(self.effective_value());
        let text_size = self.measure_value(&formatted_value);
        self.value_text.replace(formatted_value);
        self.value_text_size.set(text_size);
        let bar_size = Size::new(SLIDER_BAR_WIDTH, SLIDER_BAR_HEIGHT);
        return text_size.max(&bar_size).with_padding(&self.padding());
    }

    fn compute_child_content_size(&self, _gui: &Gui, _available_size: Size) {
        let padding = self.padding();
        let mut layout = Rectangle::default();
        layout.width = SLIDER_CURSOR_WIDTH;
        layout.height = self.widget_height() - padding.v_padding() - 2.0*SLIDER_CURSOR_SPACING;
        self.cursor_layout.set(layout)
    }

    fn compute_child_positions(&self, _gui: &Gui) {
        let content_layout = self.content_layout();
        let value = self.effective_value();

        let value_min = self.value_min.get();
        let value_max = self.value_max.get();
        let width_minus_cursor_width = content_layout.width - SLIDER_CURSOR_WIDTH;
        let cursor_position_x = content_layout.x + width_minus_cursor_width * (value-value_min)/(value_max-value_min);
        let cursor_position_y = content_layout.y + SLIDER_CURSOR_SPACING;

        let mut cursor_layout = self.cursor_layout.get();
        cursor_layout.x = cursor_position_x;
        cursor_layout.y = cursor_position_y;
        self.cursor_layout.set(cursor_layout)
    }


    fn update_action(&self, gui: &Gui, offset: &Vector2, mouse_state: &MouseState) {
        self.widget_data.wd_update_action(gui,offset,mouse_state);

        let drag_info = mouse_state.drag_info();

        if mouse_state.right().is_pressed() {
            let value = self.value.get();
            self.drag_in_progress.set(false);
            self.invalidate_preferred_size(gui);
            self.invalidate_position(gui);
            if let Some(action_id) = self.action_id() {
                gui.add_event(Drag(DragPar::cancelled(action_id, value)))
            }

        }


        if drag_info.started() {
            let mut content_layout = self.content_layout();
            content_layout.x += offset.x;
            content_layout.y += offset.y;
            let mouse_position = drag_info.starting_position().clone();
            let inside= content_layout.check_collision_point_rec(mouse_position);

            if inside {
                self.drag_value.set(self.value.get());
                self.drag_in_progress.set(true);
                self.drag_starting_position.set(mouse_position);
            }
        }
        else if drag_info.in_progress() && self.drag_in_progress.get() {
            let mouse_position = drag_info.current_position();
            let cursor_layout = self.cursor_layout.get();
            let content_layout = self.content_layout();
            let value_min = self.value_min.get();
            let value_max = self.value_max.get();

            let available_bar_width = content_layout.width - cursor_layout.width;
            let displacement = (mouse_position.x - (content_layout.x + offset.x + cursor_layout.width*0.5) )/available_bar_width;

            let value = displacement.clamp(0.0,1.0)*(value_max - value_min) + value_min;
            let current_value = self.drag_value.get();
            if current_value != value {
                self.drag_value.set(value);
                self.invalidate_preferred_size(gui);
                self.invalidate_position(gui);
                if let Some(action_id) = self.action_id() {
                    gui.add_event(Drag(DragPar::in_progress(action_id, value)))
                }
            }
        }
        else if drag_info.done() && self.drag_in_progress.get() {
            let value = self.drag_value.get();
            self.value.set(value);
            self.drag_in_progress.set(false);
            self.invalidate_preferred_size(gui);
            self.invalidate_position(gui);
            if let Some(action_id) = self.action_id() {
                gui.add_event(Drag(DragPar::done(action_id, value)))
            }
        }

        self.widget_data.wd_update_action(gui,offset,mouse_state);
    }

    fn render_my_visual(&self, _gui: &Gui, d: &mut impl RaylibDraw, offset: &Vector2) {
        let mut content_layout = self.content_layout();

        content_layout.x += offset.x;
        content_layout.y += offset.y;

        d.draw_rectangle_rec(content_layout, SLIDER_BAR_COLOR);


        {
            // println!("render offset {:?}",offset);
            let mut rectangle = self.cursor_layout.get();
            rectangle.x+=offset.x;
            rectangle.y+=offset.y;
            d.draw_rectangle_rec(rectangle,Color::GREEN)

        }

        {
            if let Some(ts) = self.text_style() {
                let text_size = self.value_text_size.get();
                let mut position = Vector2::new(content_layout.x, content_layout.y);
                position.x += (content_layout.width - text_size.width())*0.5;
                position.y += (content_layout.height - text_size.height())*0.5;

                let borrowed_text = self.value_text.borrow();
                ts.draw_text(d,borrowed_text.as_str(),&position);
            }
        }

    }


}
