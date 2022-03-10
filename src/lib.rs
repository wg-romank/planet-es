pub mod geometry;
mod log;

pub mod parameters;
pub mod shaders;

#[cfg(target_arch = "wasm32")]
mod webapp {
  use crate::parameters::RenderParameters;
  use crate::shaders::Render;
  use crate::geometry::util::Wavefront;
  use base64::decode;

  use console_error_panic_hook;

  use glsmrs::Ctx;
  use image::ImageFormat;
  use wasm_bindgen::prelude::*;

  #[wasm_bindgen]
  pub struct WebApp {
    render: Render,
    parameters: RenderParameters,
  }

  #[wasm_bindgen]
  impl WebApp {
    pub fn from(canvas_name: &str, parameters: &str) -> WebApp {
      console_error_panic_hook::set_once();

      let parameters: RenderParameters =
        serde_json::from_str(parameters).unwrap_or_else(|_| RenderParameters::new());
      let ctx = Ctx::from(canvas_name).expect("failed to create context, no canvas?");

      let render = Render::from(ctx, &parameters).expect("failed to create render");

      WebApp { render, parameters }
    }

    pub fn parameters(&self) -> String {
      serde_json::to_string(&self.parameters).unwrap()
    }

    pub fn frame(&mut self, elapsed: f32, parameters: &str) {
      let new_parameters: RenderParameters = serde_json::from_str(parameters).unwrap();
      if self.parameters != new_parameters {
        self.parameters = new_parameters;
        self.render.update_mesh(&self.parameters);
      }

      self.render.frame(elapsed, &self.parameters);
    }

    pub fn load_texture(&mut self, name: &str, encoded: &str) {
      let mut split = encoded.split(",");
      let format = split.next().unwrap();
      let data = split.next().unwrap();

      use crate::log;
      use image::load_from_memory_with_format;

      log!("name {}", name);
      log!("format {}", format);
      let data_binary = decode(data);
      // log!("decoded {:?}", data_binary);
      let img = load_from_memory_with_format(&data_binary.unwrap(), ImageFormat::Png);
      // log!("img {:?}", img);

      self.render.update_texture(img.unwrap()).expect("failed to update texture")
    }

    pub fn export_to_obj(&self) -> String {
      self.render.planet_mesh.to_obj()
    }
  }
}
