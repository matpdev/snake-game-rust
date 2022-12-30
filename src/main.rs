extern crate piston_window;
extern crate rand;

mod drawing;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use drawing::to_gui_coord_u32;
use game::Game;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

fn main() {
    let (width, height) = (45, 45);

    let mut windows_settings = WindowSettings::new(
        "Rust Snake",
        [to_gui_coord_u32(width), to_gui_coord_u32(height)],
    )
    .exit_on_esc(true);

    windows_settings.set_vsync(true);

    let mut window: PistonWindow = windows_settings.build().unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
