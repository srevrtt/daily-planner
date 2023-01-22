pub struct UI {
  ctx: raw_gl_context::GlContext
}

impl UI {
  // Initializes the rendering
  pub fn init(wnd: &winit::window::Window) -> UI {
    let conf = raw_gl_context::GlConfig {
      version: (3, 3),
      double_buffer: true,
      vsync: false,
      profile: raw_gl_context::Profile::Core,
      ..Default::default()
    };

    let ctx = raw_gl_context::GlContext::create(&wnd, conf)
      .expect("Error: Failed to create an OpenGL context.");

    let wnd_size = wnd.inner_size();

    unsafe {
      gl::load_with(|s| ctx.get_proc_address(s));
      gl::Viewport(0, 0, wnd_size.width as i32, wnd_size.height as i32);
    }

    Self {
      ctx
    }
  }

  // Renders each UI component
  pub fn render(&self) {
    unsafe {
      gl::ClearColor(0.0, 0.0, 0.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);

      self.ctx.swap_buffers();
    }
  }
}
