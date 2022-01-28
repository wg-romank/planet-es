use console_error_panic_hook;

use luminance::UniformInterface;
use luminance::shader::Uniform;
use luminance::shader::types::{Mat44, Vec4};
use luminance_web_sys::WebSysWebGL2Surface;

use luminance::render_state::RenderState;
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;

use luminance_front::shader::Program;
use luminance_front::tess::{Tess, Mode, TessError, Interleaved};
use luminance_front::Backend;

use wasm_bindgen::prelude::*;

const VS_STR: &str = include_str!("../shaders/vs.glsl");
const FS_STR: &str = include_str!("../shaders/fs.glsl");

use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum VertexSemantics {
  #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
  Position,

  #[sem(name = "norm", repr = "[f32; 3]", wrapper = "VertexNormal")]
  Normal,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "VertexSemantics")]
struct ObjVertex {
  position: VertexPosition,
  norm: VertexNormal,
}

type VertexIndex = u32;


#[derive(Debug, UniformInterface)]
struct ShaderInterface {
  // #[uniform(unbound)]
  // projection: Uniform<Mat44<f32>>,
  // #[uniform(unbound)]
  // view: Uniform<Mat44<f32>>

  #[uniform(name ="rotation", unbound)]
  rotation: Uniform<Mat44<f32>>,

  #[uniform(name = "normalMatrix", unbound)]
  normal_matrix: Uniform<Mat44<f32>>,

  #[uniform(name = "color")]
  color: Uniform<Vec4<f32>>,
}

macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

use bracket_noise::prelude::FastNoise;

#[wasm_bindgen]
pub struct Render {
  noise: FastNoise,
  surface: WebSysWebGL2Surface,
  sphere: Vec<luminance::tess::Tess<Backend, ObjVertex, u32>>,
  program: Program<VertexSemantics, (), ShaderInterface>,
}

#[wasm_bindgen]
impl Render {
  pub fn new(canvas_name: &str) -> Render {
    console_error_panic_hook::set_once();

    let mut noise = FastNoise::new();

    let mut surface = WebSysWebGL2Surface::new(canvas_name).expect("failed to create surface");
    let sphere = DIRECTIONS.iter().map(|dir| Face::new(dir, 32, &noise).to_tess(&mut surface).expect("failed to create sphere")).collect();

    let program = surface
      .new_shader_program::<VertexSemantics, (), ShaderInterface>()
      .from_strings(VS_STR, None, None, FS_STR)
      .expect("failed to create program")
      .ignore_warnings();

    Render { noise, surface, sphere, program }
  }

  pub fn frame(&mut self, elapsed: f32) {
    let color = [elapsed.cos(), elapsed.sin(), 0.5, 1.];
    let back_buffer = self.surface.back_buffer().expect("back buffer");

    let sphere = &self.sphere;
    let program = &mut self.program;
    let ctxt = &mut self.surface;

    let mut rotation = vek::mat4::Mat4::identity();
    rotation.rotate_y(elapsed);
    rotation.rotate_x(elapsed / 2.);

    let mut normal_matrix = rotation.clone();
    normal_matrix.invert();
    normal_matrix.transpose();

    let render = ctxt
      .new_pipeline_gate()
      .pipeline(
        &back_buffer,
        &PipelineState::default(), //.set_clear_color(color),
        |_, mut shd_gate| {
          shd_gate.shade(program, |mut iface, uni, mut rdr_gate| {
            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
              iface.set(&uni.rotation, rotation.into_row_arrays().into());
              iface.set(&uni.normal_matrix, normal_matrix.into_row_arrays().into());
              iface.set(&uni.color, color.into());

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
    C: GraphicsContext<Backend = Backend> {
    ctxt
      .new_tess()
      .set_mode(Mode::Triangle)
      .set_vertices(self.vertices)
      .set_indices(self.indices)
      .build()
  }

  fn new(dir: &Vec3<f32>, res: usize, noise: &FastNoise) -> Face {
    let mut vertices = Vec::<ObjVertex>::new();
    let mut indices = Vec::<VertexIndex>::new();

    let axis_a = Vec3::new(dir.y, dir.z, dir.x);
    let axis_b = dir.cross(axis_a);

    for y in 0..res {
      for x in 0..res {
        let i = (x + y * res) as u32;
        let scale_x = x as f32 / (res as f32 - 1.);
        let scale_y = y as f32 / (res as f32 - 1.);

        let mut vertex = dir + (scale_x * 2. - 1.) * axis_a + (scale_y * 2. - 1.) * axis_b;
        vertex.normalize();

        let n = noise.get_noise3d(vertex.x, vertex.y, vertex.z);
        vertex = vertex * 0.9 + vertex * 0.1 * n;

        let pos: [f32; 3] = [vertex.x, vertex.y, vertex.z];

        vertices.push(
          ObjVertex::new(
            VertexPosition::new(pos),
            VertexNormal::new(pos)
          )
        );

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

    Face { vertices, indices }
  }
}
