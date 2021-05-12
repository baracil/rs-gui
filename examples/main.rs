use raylib::prelude::*;

use gui::gui::Gui;


use gui::alignment::VAlignment::{Top, Center};
use gui::alignment::HAlignment::{Left, Middle};

use gui::widget::Widget::{Pane, VBox, Label, HBox, Slider};
use gui::pane::PanePar;
use gui::size::Size;
use gui::background::Background::Solid;
use gui::border::Border::Line;
use gui::position::Coordinate::{Relative, Absolute};
use gui::vbox::VBoxPar;
use gui::label::LabelPar;
use gui::padding::Padding;
use gui::fill::Fill::Enabled;
use gui::hbox::HBoxPar;
use gui::slider::SliderPar;
use gui::mouse::MouseState;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .vsync()
        .resizable()
        .msaa_4x()
        .title("Gui Test")
        .build();

    let mut gui = Gui::new();
    gui.load_font(&mut rl, &thread,
                  "default",
                  "/home/Bastien Aracil/Downloads/FreckleFace-Regular.ttf",
                  48,
                  200,
    ).expect("Could not load the font");

    gui.load_font(&mut rl, &thread,
                  "small",
                  "/home/Bastien Aracil/Downloads/FreckleFace-Regular.ttf",
                  24,
                  200,
    ).expect("Could not load the font");

    gui.add_text_style("default", "default", Color::BLACK, 0.0);
    gui.add_text_style("small", "small", Color::BLACK, 0.0);
    gui.add_border("default", Line { color: Color::BLACK, thickness: 2.0 });
    gui.add_background("default", Solid { idle_color: Color::DARKBLUE, hoovered_color: Color::SKYBLUE, armed_color: Color::BLUE });
    gui.add_background("red", Solid { idle_color: Color::RED, hoovered_color: Color::ORANGE, armed_color: Color::LIME });
    gui.add_background("yellow", Solid { idle_color: Color::YELLOW, hoovered_color: Color::YELLOW, armed_color: Color::YELLOW });

    let root_pane = {
        let par = PanePar::new();
        par.set_preferred_height(&gui, 200.0)
            .set_preferred_width(&gui, 600.0)
            .set_padding(&gui, Padding::same(10.0))
            .set_position(&gui, &Absolute(0.0), &Absolute(0.0))
            .set_valignment(&gui, Top)
            .set_halignment(&gui, Left)
            .enable_fill_width(&gui, Enabled { weight: 1 })
            .enable_fill_height(&gui, Enabled { weight: 1 })
            .set_action_id("ROOT");
        gui.insert_root(Pane(par))
    };

    let _vbox = {
        let par = VBoxPar::new();
        par.set_spacing(&gui, 20.0)
            .set_position(&gui, &Relative(50.0), &Relative(50.0))
            .set_halignment(&gui, Middle)
            .set_valignment(&gui, Center)
            .set_padding(&gui, Padding::same(10.0))
            .set_background_style("yellow");
        gui.add_child(root_pane, VBox(par))
    };


    let padding = Padding::same(15.0);
    let label_padding = Padding::same(0.0);

    let (_hbox1, _slider) = {
        let _hbox1 = {
            let par = HBoxPar::new();
            par.set_spacing(&gui, 10.0)
                .set_background_style("red")
                .set_padding(&gui, padding)
                .set_position(&gui, &Relative(50.0), &Relative(50.0))
                .set_valignment(&gui, Center)
                .set_halignment(&gui, Middle)
                .set_action_id("HBOX");
            gui.add_child(_vbox, HBox(par))
        };

        let _slider = {
            let par = SliderPar::new();
            par.set_value(&gui, 50.0)
                .set_value_max(&gui, 100.0)
                .set_value_min(&gui, 0.0)
                .set_action_id("slider")
                .set_padding(&gui, label_padding)
                .set_background_style("none")
                .set_border_style("none")
                .set_text_style("small")
            ;
            gui.add_child(_hbox1, Slider(par))
        };

        let _label2 = {
            let par = LabelPar::new();
            par.set_text(&gui, "Label 2")
                .set_action_id("Label2")
                .set_clickable(true)
                .set_padding(&gui, label_padding)
            ;
            gui.add_child(_hbox1, Label(par))
        };

        let _label3 = {
            let par = LabelPar::new();
            par.set_text(&gui, "3")
                .set_padding(&gui, label_padding)
                .set_action_id("Label3")
                .set_clickable(true)
                .enable_fill_width(&gui, Enabled { weight: 1 })
            ;

            gui.add_child(_hbox1, Label(par))
        };
        (_hbox1, _slider)
    };

    let _vbox2 = {
        let _vbox2 = {
            let par = VBoxPar::new();
            par.set_spacing(&gui, 30.0)
                .set_background_style("red")
                .set_padding(&gui, padding)
                .set_position(&gui, &Relative(50.0), &Relative(50.0))
                .set_valignment(&gui, Center)
                .set_halignment(&gui, Middle)
                .set_action_id("VBOX");
            gui.add_child(_vbox, VBox(par))
        };

        let _label1 = {
            let par = LabelPar::new();
            par.set_text(&gui, "Label 1b")
                .set_action_id("Label1 b")
                .set_clickable(true)
                .set_padding(&gui, padding)
            ;
            gui.add_child(_vbox2, Label(par))
        };

        let _label2 = {
            let par = LabelPar::new();
            par.set_text(&gui, "Long label with several words")
                .set_action_id("Label2 b")
                .set_clickable(true)
                .set_padding(&gui, padding)
            ;
            gui.add_child(_vbox2, Label(par))
        };

        let _slide = {
            let par = SliderPar::new();
            par.set_value(&gui,0.0)
                .set_value_min(&gui, -100.0)
                .set_value_max(&gui, 100.0)
                .set_padding(&gui, padding)
                .set_action_id("Slider 3")
                .set_clickable(false)
                .enable_fill_height(&gui, Enabled { weight: 1 })
                .enable_fill_width(&gui, Enabled { weight: 1 })
            ;

            let slider = Slider(par);
            gui.add_child(_vbox2, slider)
        };
        _vbox2
    };


    let mut screen_size: Size = Size::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

    let offset = Vector2 { x: 0.0, y: 0.0 };

    let mut camera = Camera2D::default();
    let mut mouse_state = MouseState::new();


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);


        if d.is_window_resized() {
            screen_size = Size::new(d.get_screen_width() as f32, d.get_screen_height() as f32);
            camera.offset.x = screen_size.width() * 0.5;
            camera.offset.y = screen_size.height() * 0.5;
            camera.target.x = screen_size.width() * 0.5;
            camera.target.y = screen_size.height() * 0.5;
            camera.zoom = 1.0;
        }

        {
            mouse_state.update(&d);


            d.clear_background(Color::WHITE);

            gui.update_states(&mouse_state.mouse_position(), &offset);
            gui.handle_events(&mouse_state, &offset);
            gui.layout(&screen_size);
            gui.render(&mut d, &offset);
        }

         for x in gui.get_events() {

            println!("{:?}",x);

        }
    }
}
