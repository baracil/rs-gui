use raylib::prelude::*;

pub struct MouseState {
    left: MouseButtonState,
    middle: MouseButtonState,
    right: MouseButtonState,
    mouse_position: Vector2,
    drag_info: DragInfo,
}

#[derive(Debug)]
pub struct MouseButtonState {
    button:raylib::consts::MouseButton,
    ///the button is pushed
    down: bool,
    /// the button is not pushed
    up: bool,
    ///the button switch from up to down since the last frame
    pressed: bool,
    ///the button switch from down to up since the last frame
    released: bool,
}
#[derive(Debug)]
pub struct DragInfo {
    in_progress:bool,
    started:bool,
    done:bool,
    starting_position:Vector2,
    current_position:Vector2,
    delta:Vector2,
}


impl MouseState {
    pub fn new() -> Self {
        Self {
            mouse_position:Vector2::default(),
            right: MouseButtonState::new(raylib::consts::MouseButton::MOUSE_RIGHT_BUTTON),
            middle: MouseButtonState::new(raylib::consts::MouseButton::MOUSE_MIDDLE_BUTTON),
            left: MouseButtonState::new(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON),
            drag_info: Default::default()
        }
    }

    pub fn update(&mut self, d:&RaylibDrawHandle) {
        self.left.update(d);
        self.middle.update(d);
        self.right.update(d);

        self.mouse_position = d.get_mouse_position();

        self.drag_info.update_drag_info(&self.left, &self.mouse_position)
    }

    pub fn update_2d(&mut self, d:&RaylibDrawHandle, camera:Camera2D) {
        self.left.update(d);
        self.middle.update(d);
        self.right.update(d);

        let mouse_position = d.get_mouse_position();
        self.mouse_position = d.get_screen_to_world2D(mouse_position,camera);

        self.drag_info.update_drag_info(&self.left, &self.mouse_position)
    }

    pub fn mouse_position(&self) -> &Vector2 {
        &self.mouse_position
    }

    pub fn left(&self) -> &MouseButtonState {
        &self.left
    }
    pub fn middle(&self) -> &MouseButtonState {
        &self.middle
    }
    pub fn right(&self) -> &MouseButtonState {
        &self.right
    }
    pub fn drag_info(&self) -> &DragInfo {
        &self.drag_info
    }
}

impl Default for DragInfo {
    fn default() -> Self {
        Self{
            in_progress:false,
            started:false,
            done:false,
            starting_position:Default::default(),
            current_position:Default::default(),
            delta:Default::default(),
        }
    }
}

impl DragInfo {
    fn update_drag_info(&mut self, button_state:&MouseButtonState, mouse_position:&Vector2) {
        self.current_position = mouse_position.clone();
        if button_state.is_pressed() {
            self.in_progress = true;
            self.done = false;
            self.starting_position = mouse_position.clone();
        }

        if button_state.is_down() {
            self.started = button_state.is_pressed();
            self.delta.x = self.current_position.x - self.starting_position.x;
            self.delta.y = self.current_position.y - self.starting_position.y;
        }

        if button_state.is_released() {
            self.in_progress = false;
        }

        if button_state.is_up() {
            self.done = button_state.is_released();
        }
    }


    pub fn in_progress(&self) -> bool {
        self.in_progress
    }


    pub fn done(&self) -> bool {
        self.done
    }
    pub fn starting_position(&self) -> &Vector2 {
        &self.starting_position
    }
    pub fn current_position(&self) -> &Vector2 {
        &self.current_position
    }
    pub fn delta(&self) -> &Vector2 {
        &self.delta
    }
    pub fn started(&self) -> bool {
        self.started
    }
}

impl MouseButtonState {
    pub fn new(button: raylib::consts::MouseButton) -> Self {
        Self {
            button,
            down: false,
            up: false,
            pressed: false,
            released: false,
        }
    }

    fn update(&mut self,d:&RaylibDrawHandle) {
        self.down = d.is_mouse_button_down(self.button);
        self.up = d.is_mouse_button_up(self.button);
        self.pressed = d.is_mouse_button_pressed(self.button);
        self.released = d.is_mouse_button_released(self.button);
    }

    pub(crate) fn is_down(&self) -> bool {
        self.down
    }
    pub(crate) fn is_up(&self) -> bool {
        self.up
    }
    pub(crate) fn is_pressed(&self) -> bool {
        self.pressed
    }
    pub(crate) fn is_released(&self) -> bool {
        self.released
    }
}
