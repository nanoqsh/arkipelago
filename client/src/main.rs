mod config;
mod window;

use self::{config::Config, window::Window};
use core::net::{Login, Packed};
use eng::{Control, Game, Render};
use glutin::event::{ElementState, MouseButton, VirtualKeyCode};
use std::{
    io::{self, Write},
    net::TcpStream,
    time::Duration,
};

pub struct App {
    game: Game,
    render: Render,
}

impl App {
    fn draw_frame(&mut self, delta: f32) {
        self.game.draw(&mut self.render, delta)
    }

    fn resize(&mut self, size: (u32, u32)) {
        self.render.resize(size, 1);
        self.game.resize(size);
    }

    fn key(&mut self, key: VirtualKeyCode, state: ElementState) {
        if let ElementState::Released = state {
            return;
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

fn login(login: Login) -> Result<(), io::Error> {
    let config = Config::load();
    let addr = config.socket_addr();
    println!("Wait for connection ..");
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(30))?;
    let packed = Packed::new(&login).unwrap();
    stream.write_all(packed.bytes())?;
    stream.flush()
}

fn main() {
    login(Login {
        name: "nano".into(),
        pass: "123".into(),
    })
    .expect("login");

    let (window, render) = Window::new("hui 0.0.1");
    let app = App {
        game: Game::new(&render),
        render,
    };

    window.run(app, 30, (800, 600))
}
