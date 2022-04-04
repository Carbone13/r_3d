mod state;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::dpi::PhysicalSize;
use crate::state::State;

fn  main() {
    pollster::block_on(run());
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
                        .with_inner_size(PhysicalSize::new(1280, 720))
                        .with_title("R 3D")
                        .build(&event_loop)
                        .unwrap();

    let mut state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow|
        {
        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::WindowEvent
            {
                ref event,
                window_id,
            }
            if window_id == window.id() => if !state.input(event)
            {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        eprintln!("{}", position.x);
                        state.color = if position.x > 1280.0 / 2.0 {
                            wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }
                        }
                        else {  wgpu::Color {
                            r: 0.7,
                            g: 0.5,
                            b: 0.5,
                            a: 1.0,
                        }}

                    }

                    _ => {}
                }
            }
            _ => {}
        }
    });
}