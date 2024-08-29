use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::*;
use winit::window::{Window, WindowBuilder};
use crate::dep::basic;
use crate::dep::basic::state;
use crate::dep::basic::state::State;
use game_loop::game_loop;

pub struct Looper<'a> {
    window: &'a Window,
    state: State<'a>,

    surface_configured: bool,
}


impl<'a> Looper<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let mut state = State::new(&window).await;

        Self {
            window,
            state,
            surface_configured: false,
        }
    }


    pub fn update(&mut self) {
        self.state.update()
    }


    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>{
        self.state.render_quad()
    }

    pub fn handler(&mut self, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == &self.window.id() => {
                if !self.state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                            ..
                        } => return false,
                        WindowEvent::Resized(physical_size) => {
                            self.state.resize(*physical_size);
                            self.surface_configured = true;
                        }
                        // WindowEvent::RedrawRequested => {
                        //     self.state.window().request_redraw();
                        //
                        //     if !self.surface_configured {
                        //         return true;
                        //     }
                        //
                        //     match self.state.render() {
                        //         Ok(_) => {}
                        //         // Reconfigure the surface if it's lost or outdated
                        //         Err(
                        //             wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                        //         ) => self.state.resize(self.state.size),
                        //         // The system is out of memory, we should probably quit
                        //         Err(wgpu::SurfaceError::OutOfMemory) => {
                        //             log::error!("OutOfMemory");
                        //             return false;
                        //         }
                        //         // We're ignoring timeouts
                        //         Err(wgpu::SurfaceError::Timeout) => {
                        //             log::warn!("Surface timeout")
                        //         }
                        //     }
                        // }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        return true;
    }

}

pub async fn run() {
    let n : u32 = 60;
    let frame_duration = Duration::from_secs_f64(1.0 / n as f64);
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().
        with_inner_size(winit::dpi::LogicalSize::new(basic::structure::WIDTH, basic::structure::HEIGHT)).
        build(&event_loop).unwrap();

    let mut looper = Looper::new(&window).await;

    let mut last_update = Instant::now();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run(move |event, control_flow| {


        match event {
            Event::WindowEvent { ref event, window_id, .. } if window_id == looper.window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                     control_flow.exit()
                    }
                    WindowEvent::KeyboardInput {
                        event,
                        ..
                    } => {
                        if event.state == ElementState::Pressed {
                            match event.physical_key {
                                PhysicalKey::Code(KeyCode::KeyQ) | PhysicalKey::Code(KeyCode::Escape)=> {
                                    control_flow.exit();
                                    return
                                }
                                _ => {
                                    if looper.state.keyboard_input(event) {
                                        looper.window.request_redraw();
                                        return
                                    }
                                }

                            }
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        looper.state.resize(*physical_size);
                        looper.surface_configured = true;
                    }
                    WindowEvent::RedrawRequested => {
                        looper.window.request_redraw();
                        if !looper.surface_configured {
                            return;
                        }
                        let now = Instant::now();
                        if now.duration_since(last_update) >= frame_duration {
                            // 渲染逻辑放在这里
                            looper.window.request_redraw();
                            last_update = now;


                            match looper.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    looper.state.resize(looper.state.size);
                                }
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    log::error!("OutOfMemory");
                                    control_flow.exit();
                                }
                                Err(wgpu::SurfaceError::Timeout) => {
                                    log::warn!("Surface timeout")
                                }
                            }
                        }


                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }).unwrap();
}
