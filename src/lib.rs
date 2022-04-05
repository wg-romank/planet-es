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
  use glsmrs::texture::Viewport;
  use glsmrs::util::get_canvas;
  use glsmrs::util::get_ctx_from_canvas;
  use image::ImageFormat;
  use wasm_bindgen::prelude::*;
  use web_sys::WebGlRenderingContext;

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
      let canvas = get_canvas(canvas_name).expect(&format!("no canvas {}", canvas_name));
      let webgl_ctx: WebGlRenderingContext = get_ctx_from_canvas(&canvas, "webgl").expect("webgl not found, really?");
      let ctx = Ctx::new(webgl_ctx).expect("failed to create context, no canvas?");

      let canvas_viewport = Viewport::new(canvas.width(), canvas.height());

      let render = Render::from(ctx, &parameters, canvas_viewport).expect("failed to create render");

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

    pub fn load_texture(&mut self, name: &str, encoded: &str) -> Result<(), String> {
      let mut split = encoded.split(",");
      let format = split.next().unwrap();
      let data = split.next().unwrap();

      use crate::log;

      log!("name {}", name);
      log!("format {}", format);
      let data_binary = decode(data)
        .map_err(|e| format!("unable to decode data {}", e))?;

      self.render.update_texture(&data_binary)
    }

    pub fn rotate(&mut self, leftright: f32, topdown: f32) {
      self.render.rotate(leftright, topdown)
    }

    pub fn set_rotated(&mut self) {
      self.render.set_rotated()
    }

    pub fn export_to_obj(&self) -> String {
      self.render.planet_mesh.to_obj()
    }
  }
}
