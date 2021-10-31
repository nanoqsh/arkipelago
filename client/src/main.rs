mod window;

use self::window::Window;
use eng::{Control, Game, Render};
use glutin::event::{ElementState, MouseButton, VirtualKeyCode};

pub struct App {
    render: Render,
    game: Game,
}

impl App {
    fn draw_frame(&mut self, _: f32) {
        self.game.draw(&mut self.render)
    }

    fn resize(&mut self, (width, height): (u32, u32)) {
        self.render.resize((width, height))
    }

    fn key(&mut self, key: VirtualKeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => (),
            ElementState::Released => return,
        }

        let control = match key {
            VirtualKeyCode::W => Control::Forward,
            VirtualKeyCode::A => Control::Left,
            VirtualKeyCode::S => Control::Back,
            VirtualKeyCode::D => Control::Right,
            _ => return,
        };

        self.game.input(control)
    }

    fn mouse_move(&mut self, (x, y): (f32, f32)) {
        self.game.input(Control::Look(x, y))
    }

    fn mouse(&mut self, _: MouseButton, _: ElementState) {}

    fn scroll(&mut self, (x, y): (f32, f32)) {
        self.game.input(Control::Scroll(x, y))
    }
}

fn main() {
    let (window, render) = Window::new("hui 0.0.1");
    window.run(
        App {
            render: render.init(),
            game: Game::new(),
        },
        30,
        (800, 600),
    )
}
