use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop
        .run(move |event, event_loop_window_target| {
            //*control_flow = ControlFlow::Wait;
            event_loop_window_target.set_control_flow(ControlFlow::Wait);

            match event {
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::MouseMotion { delta } => {
                        println!("Mouse moved by {:?}", delta);
                    }
                    _ => (),
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        event_loop_window_target.exit();
                    }
                    _ => (),
                },
                _ => (),
            }
        })
        .unwrap();
}
