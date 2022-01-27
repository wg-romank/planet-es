use std::time::Instant;
use std::collections::HashMap;
use std::process::exit;
use console_error_panic_hook;

// use luminance_front::tess::TessError;
// use luminance_front::tess::Interleaved;
// use luminance_front::tess::Tess;
// use luminance_webgl::webgl2::WebGL2;
// use luminance::context::GraphicsContext; 
// use luminance::context::GraphicsContext as _; 

use js_sys::Date;
use luminance::context::GraphicsContext;
// use glfw::{Action, Context as _, Key, WindowEvent};
// use luminance_glfw::GlfwSurface;
// use luminance_windowing::{WindowDim, WindowOpt};
use luminance::pipeline::PipelineState;

use luminance::texture::Dim2;
use luminance_front::framebuffer::Framebuffer;
use luminance_web_sys::WebSysWebGL2Surface;

use luminance_webgl::WebGL2;
use wasm_bindgen::prelude::*;

// use wasm_bindgen::JsCast;
// use web_sys;

// use luminance::tess::Mode;

// use luminance::shader::Program;
// use luminance::render_state::RenderState;

// const VS_STR: &str = include_str!("../shaders/vs.glsl");
// const FS_STR: &str = include_str!("../shaders/fs.glsl");

use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum VertexSemantics {
  #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
  Position,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "VertexSemantics")]
struct ObjVertex {
  position: VertexPosition,
}

type VertexIndex = u32;

use std::fs::File;
use std::io::Read as _;
use std::path::Path;
use try_guard::verify;
use wavefront_obj::obj;

use luminance_front::Backend;

macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

#[wasm_bindgen]
pub struct Render {
  surface: WebSysWebGL2Surface
}

#[wasm_bindgen]
impl Render {
  pub fn new(canvas_name: &str) -> Render {
    let surface = WebSysWebGL2Surface::new(canvas_name).expect("failed to create surface");

    Render { surface }
  }

  pub fn frame(&mut self, elapsed: f32) {
    let color = [elapsed.cos(), elapsed.sin(), 0.5, 1.];
    let back_buffer = self.surface.back_buffer().expect("back buffer");

    let render = self.surface
      .new_pipeline_gate()
      .pipeline(
        &back_buffer,
        &PipelineState::default().set_clear_color(color),
        // |_, mut shd_gate| {
        //   shd_gate.shade(&mut program, |_, _, mut rdr_gate| {
        //     rdr_gate.render(&RenderState::default(), |mut tess_gate| {
        //       tess_gate.render(&shape)
        //     })
        //   })
        // },
        |_, _| Ok(())
      )
      .assume();

    if !render.is_ok() {
      log!("error rendering {:?}", render.into_result());
    }
  }
}
