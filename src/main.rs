use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::platform::scancode::PhysicalKeyExtScancode;
fn main() {
let event_loop = EventLoop::new().unwrap();
let window = WindowBuilder::new().build(&event_loop).unwrap();

// ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
// dispatched any events. This is ideal for games and similar applications.
event_loop.set_control_flow(ControlFlow::Poll);

// ControlFlow::Wait pauses the event loop if no events are available to process.
// This is ideal for non-game applications that only update in response to user
// input, and uses significantly less power/CPU time than ControlFlow::Poll.
event_loop.set_control_flow(ControlFlow::Wait);

event_loop.run(move |event, elwt| {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            println!("The close button was pressed; stopping");
            elwt.exit();
        },
        Event::AboutToWait => {
            // Application update code.

            // Queue a RedrawRequested event.
            //
            // You only need to call this if you've determined that you need to redraw in
            // applications which do not always need to. Applications that redraw continuously
            // can render here instead.
            window.request_redraw();
        },
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            // Redraw the application.
            //
            // It's preferable for applications that do not render continuously to render in
            // this event rather than in AboutToWait, since rendering in here allows
            // the program to gracefully handle redraws requested by the OS.
        },
        Event::WindowEvent { 
                event: WindowEvent::KeyboardInput { event, .. },
                ..
            } => {
        println!("ScanCode: {:?}", event.physical_key.to_scancode());
        println!("______");
        println!("{:?}", event);
// WindowEvent { window_id: WindowId(WindowId(140350081484064)), event: KeyboardInput { device_id: DeviceId(DeviceId), event: KeyEvent { physical_key: Code(BracketLeft), logical_key: Character("["), text: Some("["), location: Standard, state: Pressed, repeat: false, platform_specific: KeyEventExtra { text_with_all_modifiers: Some("["), key_without_modifiers: Character("[") } }, is_synthetic: false } }
            // It's preferable for applications that do not render continuously to render in
            // this event rather than in AboutToWait, since rendering in here allows
            // the program to gracefully handle redraws requested by the OS.
        },
        x => {
            }
    }
});
}
