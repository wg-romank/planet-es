#[cfg(not(target_arch = "wasm32"))]
mod desktop {
  use std::time::Instant;
  
  use glfw::{Action, Context as _, Key, SwapInterval, WindowEvent, WindowMode};
  use luminance_glfw::GL33Context;
  use luminance_glfw::{GlfwSurface, GlfwSurfaceError};
  use luminance_front::context::GraphicsContext;

  use florest::parameters::RenderParameters;
  use florest::shaders::Render;

  fn main_loop(start_t: Instant, mut surafce: GlfwSurface) {

    let fb = surafce.context.back_buffer().expect("failed to fetch fb");
    let parameters = RenderParameters::new();
    let mut r: Render<GL33Context> = Render::from(surafce.context, &parameters, fb);
    loop {
      r.ctxt.window.glfw.poll_events();

      let elapsed = start_t.elapsed();
      let t = elapsed.as_secs() as f64 + (elapsed.subsec_millis() as f64 * 1e-3);

      r.frame(t as f32, &parameters);

      r.ctxt.window.swap_buffers();
    }
  }

  pub fn run() -> () {
    let surface = GlfwSurface::new(|glfw| {
      let (mut window, events) = glfw
        .create_window(800, 800, "Hello", WindowMode::Windowed)
        .ok_or_else(|| GlfwSurfaceError::UserError("e"))?;

      window.make_current();
      window.set_all_polling(true);
      glfw.set_swap_interval(SwapInterval::Sync(1));

      Ok((window, events))
    });

    let start_t = Instant::now();

    match surface {
      Ok(surface) => {
        eprintln!("graphics surface created");
        main_loop(start_t, surface);
      }

      Err(e) => {
        eprintln!("cannot create graphics surface:\n{}", e);
        // exit(1);
      }
    }
  }
}

fn main() {
  #[cfg(not(target_arch = "wasm32"))]
  desktop::run();
}
