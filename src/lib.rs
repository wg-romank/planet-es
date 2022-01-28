use console_error_panic_hook;

use luminance::backend::tess_gate;
use luminance::render_state::RenderState;
use luminance_front::shader::Program;
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;

use luminance::tess::{Interleaved, Mode, TessError};
use luminance_front::tess::Tess;
use luminance_web_sys::WebSysWebGL2Surface;

use luminance_webgl::WebGL2;
use wasm_bindgen::prelude::*;

// use wasm_bindgen::JsCast;
// use web_sys;

// use luminance::tess::Mode;

// use luminance::shader::Program;
// use luminance::render_state::RenderState;

const VS_STR: &str = include_str!("../shaders/vs.glsl");
const FS_STR: &str = include_str!("../shaders/fs.glsl");

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

// use luminance_front::Backend;

macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

#[wasm_bindgen]
pub struct Render {
  surface: WebSysWebGL2Surface,
  sphere: Vec<luminance::tess::Tess<WebGL2, ObjVertex, u32>>,
  program: Program<VertexSemantics, (), ()>,
}

#[wasm_bindgen]
impl Render {
  pub fn new(canvas_name: &str) -> Render {
    console_error_panic_hook::set_once();

    let mut surface = WebSysWebGL2Surface::new(canvas_name).expect("failed to create surface");
    let sphere = DIRECTIONS.iter().map(|dir| Face::new(dir, 32).to_tess(&mut surface).expect("failed to create sphere")).collect();

    let program = surface
      .new_shader_program::<VertexSemantics, (), ()>()
      .from_strings(VS_STR, None, None, FS_STR)
      .expect("failed to create program")
      .ignore_warnings();

    Render { surface, sphere, program }
  }

  pub fn frame(&mut self, elapsed: f32) {
    let color = [elapsed.cos(), elapsed.sin(), 0.5, 1.];
    let back_buffer = self.surface.back_buffer().expect("back buffer");

    let sphere = &self.sphere;
    let program = &mut self.program;
    let ctxt = &mut self.surface;

    let render = ctxt
      .new_pipeline_gate()
      .pipeline(
        &back_buffer,
        &PipelineState::default().set_clear_color(color),
        |_, mut shd_gate| {
          shd_gate.shade(program, |_, _, mut rdr_gate| {
            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
              sphere.iter().map(|f| tess_gate.render(f)).collect::<Result<(), _>>()
            })
          })
        },
        // |_, _| Ok(()),
      )
      .assume();

    if !render.is_ok() {
      log!("error rendering {:?}", render.into_result());
    }
  }
}

use vek::Vec3;

const DIRECTIONS: [Vec3<f32>; 6] = [
  Vec3::new( 1.,  0.,  0.),
  Vec3::new( 0.,  1.,  0.),
  Vec3::new( 0.,  0.,  1.),
  Vec3::new(-1.,  0.,  0.),
  Vec3::new( 0., -1.,  0.),
  Vec3::new( 0.,  0., -1.),
];

#[derive(Debug)]
struct Face {
  vertices: Vec<ObjVertex>,
  indices: Vec<VertexIndex>,
}

impl Face {
  fn to_tess<C>(
    self,
    ctxt: &mut C,
  ) -> Result<Tess<ObjVertex, VertexIndex, (), Interleaved>, TessError>
  where
    C: GraphicsContext<Backend = WebGL2> {
    ctxt
      .new_tess()
      .set_mode(Mode::Triangle)
      .set_vertices(self.vertices)
      .set_indices(self.indices)
      .build()
  }

  fn new(dir: &Vec3<f32>, res: usize) -> Face {
    let mut vertices = Vec::<ObjVertex>::new();
    let mut indices = Vec::<VertexIndex>::new();

    // for dir in DIRECTIONS.iter() {
      let axis_a = Vec3::new(dir.y, dir.z, dir.x);
      let axis_b = dir.cross(axis_a);

      for y in 0..res {
        for x in 0..res {
          let i = (x + y * res) as u32;
          let scale_x = x as f32 / res as f32;
          let scale_y = y as f32 / res as f32;

          let mut vertex = dir + (scale_x * 2. - 1.) * axis_a + (scale_y * 2. - 1.) * axis_b;
          vertex.normalize();

          vertices.push(ObjVertex::new(VertexPosition::new([vertex.x, vertex.y, vertex.z])));

          if x != res - 1 && y != res - 1 {
            indices.push(i);
            indices.push(i + res as u32 + 1);
            indices.push(i + res as u32);

            indices.push(i);
            indices.push(i + 1);
            indices.push(i + res as u32 + 1);
          }
        }
      }
    // }

    Face { vertices, indices }
  }
}
