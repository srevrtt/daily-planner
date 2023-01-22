use winit::{event_loop::EventLoop, dpi::LogicalSize, window::WindowBuilder, event::Event};
use ui_framework::UI;

// Runs the native application
pub fn run_app() {
  let event_loop = EventLoop::new();

  let wnd = WindowBuilder::new()
    .with_title("Daily Planner")
    .with_inner_size(LogicalSize::new(1280, 720))
    .build(&event_loop)
    .unwrap();

  let ui = UI::init(&wnd);

  event_loop.run(move |event, _, control_flow| {
    control_flow.set_wait();

    match event {
      Event::WindowEvent { 
        event: winit::event::WindowEvent::CloseRequested,
        ..  
      } => {
        control_flow.set_exit();
      },

      Event::MainEventsCleared => {
        wnd.request_redraw();
      },

      Event::RedrawRequested(_) => {
        ui.render();
      }

      _ => {}
    }
  });
}
