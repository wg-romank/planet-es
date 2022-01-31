use console_error_panic_hook;

use luminance::pixel::RGBA32F;
use luminance::shader::types::{Mat44, Vec4, Vec3};
use luminance::shader::Uniform;
use luminance::UniformInterface;
use luminance::texture::{Sampler, Dim2};
use luminance_web_sys::WebSysWebGL2Surface;

use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;

use luminance_front::shader::Program;
use luminance_front::tess::{Interleaved, Mode, Tess, TessError};
use luminance_front::Backend;

use serde::{Deserialize, Serialize};
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
  #[uniform(name = "rotation", unbound)]
  rotation: Uniform<Mat44<f32>>,

  #[uniform(name = "normalMatrix", unbound)]
  normal_matrix: Uniform<Mat44<f32>>,

  #[uniform(name = "color", unbound)]
  color: Uniform<Vec4<f32>>,

  #[uniform(name = "lightPosition", unbound)]
  light_position: Uniform<Vec3<f32>>,
}

macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

use bracket_noise::prelude::FastNoise;
use vek::Vec3 as Vek3;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RenderParameters {
  light_position: Vek3<f32>,
  color: [f32; 4],
  face_resolution: usize,
  radius: f32,
  filters: Vec<MeshFilterParameters>,
}

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct MeshFilterParameters {
  strength: f32,
  roughness: f32,
  min_value: f32,
  center: Vek3<f32>,
  enabled: bool,
}

impl MeshFilterParameters {
  fn evaluate(&self, noise: &FastNoise, point: Vek3<f32>) -> f32 {
    let shifted = point * self.roughness + self.center;

    let noise = (noise.get_noise3d(
      shifted.x,
      shifted.y,
      shifted.z
    ) + 1.) * 0.5;

    f32::max(0., noise - self.min_value) * self.strength
  }
}

#[wasm_bindgen]
impl MeshFilterParameters {
  pub fn generate() -> String {
    serde_json::to_string(&MeshFilterParameters::default()).unwrap()
  }
}

impl Default for MeshFilterParameters {
    fn default() -> Self {
      Self {
          strength: 1.,
          roughness: 0.5,
          min_value: 0.,
          center: Vek3::new(0., 0., 0.),
          enabled: true,
      }
    }
}

fn mk_sphere(
  surface: &mut WebSysWebGL2Surface,
  parameters: &RenderParameters,
) -> Vec<luminance::tess::Tess<Backend, ObjVertex, u32>> {
  let noise = FastNoise::new();

  DIRECTIONS
    .iter()
    .map(|dir| {
      Face::new(dir, parameters, &noise)
        .to_tess(surface)
        .expect("failed to create sphere")
    })
    .collect()
}

#[wasm_bindgen]
pub struct Render {
  surface: WebSysWebGL2Surface,
  sphere: Vec<luminance::tess::Tess<Backend, ObjVertex, u32>>,
  program: Program<VertexSemantics, (), ShaderInterface>,
  parameters: RenderParameters,
}

#[wasm_bindgen]
impl Render {
  pub fn parameters(&self) -> String {
    serde_json::to_string(&self.parameters).unwrap()
  }

  pub fn from(canvas_name: &str, parameters: &str) -> Render {
    console_error_panic_hook::set_once();
    let mut surface = WebSysWebGL2Surface::new(canvas_name).expect("failed to create surface");

    let program = surface
      .new_shader_program::<VertexSemantics, (), ShaderInterface>()
      .from_strings(VS_STR, None, None, FS_STR)
      .expect("failed to create program")
      .ignore_warnings();

    log!("parameters {}", parameters);

    let parameters: RenderParameters = serde_json::from_str(parameters).unwrap();

    let sphere = mk_sphere(&mut surface, &parameters);

    Render {
      surface,
      sphere,
      program,
      parameters,
    }
  }

  pub fn new(canvas_name: &str) -> Render {
    let parameters = RenderParameters {
      light_position: Vek3::new(-0.85, -0.8, -0.75),
      color: [1., 0., 0.5, 1.],
      face_resolution: 32,
      radius: 0.5,
      filters: vec![
        MeshFilterParameters::default(),
      ],
    };

    Render::from(canvas_name, &serde_json::to_string(&parameters).unwrap())
  }

  pub fn update_parameters(&mut self, new_parameters: RenderParameters) {
    if self.parameters != new_parameters {
      self.parameters = new_parameters;
      log!("{:?}", self.parameters);
      self.sphere = mk_sphere(&mut self.surface, &self.parameters);
    }
  }

  pub fn frame(&mut self, elapsed: f32, parameters: &str) {
    log!("parameters frame {}", parameters);
    let new_parameters: RenderParameters = serde_json::from_str(parameters).unwrap();
    self.update_parameters(new_parameters);

    // let color = [elapsed.cos(), elapsed.sin(), 0.5, 1.];
    let color = self.parameters.color;
    let light_position = self.parameters.light_position;

    let sphere = &self.sphere;
    let program = &mut self.program;
    let ctxt = &mut self.surface;

    let mut rotation = vek::mat4::Mat4::identity();
    rotation.rotate_y(elapsed);
    rotation.rotate_x(elapsed / 2.);

    let mut normal_matrix = rotation.clone();
    normal_matrix.invert();
    normal_matrix.transpose();

    // let shadow_map = ctxt.new_framebuffer::<Dim2, RGBA32F, ()>(
    //   [400, 400], 0, Sampler::default()
    // );

    let back_buffer = ctxt.back_buffer().expect("back buffer");

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
              iface.set(&uni.light_position, light_position.into_array().into());

              sphere
                .iter()
                .map(|f| tess_gate.render(f))
                .collect::<Result<(), _>>()
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

const DIRECTIONS: [Vek3<f32>; 6] = [
  Vek3::new(1., 0., 0.),
  Vek3::new(0., 1., 0.),
  Vek3::new(0., 0., 1.),
  Vek3::new(-1., 0., 0.),
  Vek3::new(0., -1., 0.),
  Vek3::new(0., 0., -1.),
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
    C: GraphicsContext<Backend = Backend>,
  {
    ctxt
      .new_tess()
      .set_mode(Mode::Triangle)
      .set_vertices(self.vertices)
      .set_indices(self.indices)
      .build()
  }

  fn new(dir: &Vek3<f32>, parameters: &RenderParameters, noise: &FastNoise) -> Face {
    let mut vertices = Vec::<ObjVertex>::new();
    let mut indices = Vec::<VertexIndex>::new();

    let axis_a = Vek3::new(dir.y, dir.z, dir.x);
    let axis_b = dir.cross(axis_a);

    let res = parameters.face_resolution;

    for y in 0..res {
      for x in 0..res {
        let i = (x + y * res) as u32;
        let scale_x = x as f32 / (res as f32 - 1.);
        let scale_y = y as f32 / (res as f32 - 1.);

        let mut vertex = dir + (scale_x * 2. - 1.) * axis_a + (scale_y * 2. - 1.) * axis_b;
        vertex.normalize();

        // todo: extract function
        let mesh_offset = parameters.filters.iter().fold(0., |v, f| {
          if f.enabled {
            v + f.evaluate(noise, vertex)
          } else {
            v
          }
        });
        vertex = vertex * parameters.radius * (1. + mesh_offset);

        let pos: [f32; 3] = [vertex.x, vertex.y, vertex.z];

        vertices.push(ObjVertex::new(
          VertexPosition::new(pos),
          VertexNormal::new(pos),
        ));

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
