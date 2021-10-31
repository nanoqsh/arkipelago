use crate::App;
use eng::Render;

type EventLoop = glutin::event_loop::EventLoop<()>;
type Context = glutin::WindowedContext<glutin::PossiblyCurrent>;

pub struct Window {
    context: Context,
    event_loop: EventLoop,
}

impl Window {
    pub fn new<T>(title: T) -> (Self, Render)
    where
        T: Into<String>,
    {
        unsafe {
            let event_loop = glutin::event_loop::EventLoop::new();
            let builder = glutin::window::WindowBuilder::new()
                .with_title(title)
                .with_resizable(true);

            let context = glutin::ContextBuilder::new()
                .with_vsync(true)
                .with_gl(glutin::GlRequest::Specific(
                    glutin::Api::OpenGl,
                    eng::GL_VERSION,
                ))
                .build_windowed(builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();

            let render = eng::Render::new(|s| context.get_proc_address(s) as *const _);
            let window = Self {
                context,
                event_loop,
            };
            (window, render)
        }
    }

    pub fn run(self, mut app: App, fps: u32, size: (u32, u32)) -> ! {
        use glutin::{
            event::{DeviceEvent, Event, KeyboardInput, MouseScrollDelta, StartCause, WindowEvent},
            event_loop::ControlFlow,
        };
        use std::time::{Duration, Instant};

        let micros = if fps == 0 { 0 } else { 1_000_000 / fps as u64 };
        let mut last = Instant::now();

        let context = self.context;
        self.event_loop.run(move |event, _, flow| {
            match event {
                Event::WindowEvent { event, .. } => {
                    return match event {
                        WindowEvent::Resized(size) => {
                            context.resize(size);
                            app.resize(size.into());
                        }
                        WindowEvent::CloseRequested => {
                            *flow = ControlFlow::Exit;
                        }
                        WindowEvent::MouseWheel {
                            delta: MouseScrollDelta::LineDelta(x, y),
                            ..
                        } => app.scroll((x, y)),
                        WindowEvent::MouseInput { state, button, .. } => app.mouse(button, state),
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state,
                                    virtual_keycode: Some(key),
                                    ..
                                },
                            ..
                        } => app.key(key, state),
                        _ => (),
                    }
                }
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta: (x, y) },
                    ..
                } => {
                    app.mouse_move((x as f32, y as f32));
                    return;
                }
                Event::NewEvents(cause) => match cause {
                    StartCause::ResumeTimeReached { .. } | StartCause::Poll => {
                        let now = Instant::now();
                        app.draw_frame(now.duration_since(last).as_secs_f32());
                        last = now;
                        context.swap_buffers().unwrap();
                    }
                    StartCause::WaitCancelled {
                        requested_resume, ..
                    } => {
                        *flow = ControlFlow::WaitUntil(requested_resume.unwrap());
                        return;
                    }
                    StartCause::Init => {
                        app.resize(size);
                        context.resize(size.into());
                        last = Instant::now();
                    }
                },
                _ => return,
            }

            *flow = match micros {
                0 => ControlFlow::Poll,
                _ => ControlFlow::WaitUntil(last + Duration::from_micros(micros)),
            };
        })
    }
}
