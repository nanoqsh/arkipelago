mod window;

use self::window::Window;
use eng::{Control, Game, Render};
use glutin::event::{ElementState, MouseButton, VirtualKeyCode};

pub struct App {
    render: Render,
    game: Game,
}

impl App {
    fn draw_frame(&mut self, delta: f32) {
        self.game.draw(&mut self.render, delta)
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
    let (render, data) = render.init();
    let app = App {
        render,
        game: Game::new(data),
    };

    window.run(app, 30, (800, 600))
}
