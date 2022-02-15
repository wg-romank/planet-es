pub mod geometry;
mod log;

pub mod parameters;
pub mod shaders;

#[cfg(target_arch = "wasm32")]
mod webapp {
  use crate::parameters::RenderParameters;
  use crate::shaders::Render;

  use console_error_panic_hook;

  use luminance_web_sys::WebSysWebGL2Surface;
  use wasm_bindgen::prelude::*;

  #[wasm_bindgen]
  pub struct WebApp {
    render: Render<WebSysWebGL2Surface>,
    parameters: RenderParameters,
  }

  #[wasm_bindgen]
  impl WebApp {
    pub fn parameters(&self) -> String {
      serde_json::to_string(&self.parameters).unwrap()
    }

    pub fn from(canvas_name: &str, parameters: &str) -> WebApp {
      console_error_panic_hook::set_once();

      let parameters: RenderParameters =
        serde_json::from_str(parameters).unwrap_or_else(|_| RenderParameters::new());
      let mut surface = WebSysWebGL2Surface::new(canvas_name).expect("failed to create surface");
      let fb = surface.back_buffer().expect("failed to get backbuffer");
      let render = Render::from(surface, &parameters, fb);

      WebApp { render, parameters }
    }

    pub fn frame(&mut self, elapsed: f32, parameters: &str) {
      let new_parameters: RenderParameters = serde_json::from_str(parameters).unwrap();
      if self.parameters != new_parameters {
        self.parameters = new_parameters;
        self.render.update_mesh(&self.parameters);
      }

      self.render.frame(elapsed, &self.parameters);
    }
  }
}
