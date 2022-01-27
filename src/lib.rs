use std::time::Instant;
use std::collections::HashMap;
use std::process::exit;

// use luminance_front::tess::TessError;
// use luminance_front::tess::Interleaved;
// use luminance_front::tess::Tess;
// use luminance_webgl::webgl2::WebGL2;
// use luminance::context::GraphicsContext; 
// use luminance::context::GraphicsContext as _; 

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
pub fn run(canvas_name: &str) {
  let surface = WebSysWebGL2Surface::new(canvas_name);

  match surface {
    Ok(surface) => {
      log!("graphics surface created");
      main_loop(surface);
    }
    Err(e) => {
      log!("cannot create graphics surface:\n{}", e);
      exit(1);
    }
  }
}


fn main_loop(surface: WebSysWebGL2Surface) {
  let start_t = Instant::now();
  let mut ctxt = surface;
  // let events = surface.events_rx;
  let back_buffer = ctxt.back_buffer().expect("back buffer");

  // let shape = Obj::load("Box.obj").unwrap().to_tess(&mut ctxt).unwrap();
  
  // let mut program = ctxt
  //   .new_shader_program::<VertexSemantics, (), ()>()
  //   .from_strings(VS_STR, None, None, FS_STR)
  //   .unwrap()
  //   .ignore_warnings();

  'app: loop {
    // handle events
    // ctxt.window.glfw.poll_events();
    // for (_, event) in glfw::flush_messages(&events) {
    //   match event {
    //     WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
    //     _ => (),
    //   }
    // }
    // rendering code goes here
    let t = start_t.elapsed().as_secs_f32();
    let color = [t.cos(), t.sin(), 0.5, 1.];

    let render = ctxt
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

    // swap buffer chains
    if render.is_ok() {
      // ctxt.window.swap_buffers();
      log!("render ok");
    } else {
      break 'app;
    }
  }
}
