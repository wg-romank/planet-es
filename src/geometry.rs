use crate::shaders::ObjVertex;
use crate::shaders::VertexIndex;
use crate::shaders::VertexPosition;
use crate::shaders::VertexNormal;

use crate::parameters::RenderParameters;

use luminance_front::Backend;
use luminance_front::tess::{Interleaved, Mode, Tess, TessError};
use luminance::context::GraphicsContext;

use bracket_noise::prelude::FastNoise;
use vek::{Vec3 as Vek3};

#[derive(Debug)]
pub struct Face {
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
        let scale_x = x as f32 / (res as f32 - 1.);
        let scale_y = y as f32 / (res as f32 - 1.);

        let poin_on_unit_sphere = (dir + (scale_x * 2. - 1.) * axis_a + (scale_y * 2. - 1.) * axis_b).normalized();

        let mesh_offset = parameters.mesh_parameters.evaluate(noise, poin_on_unit_sphere);
        let vertex = poin_on_unit_sphere * parameters.radius * (1. + mesh_offset);

        let pos: [f32; 3] = [vertex.x, vertex.y, vertex.z];

        vertices.push(ObjVertex::new(
          VertexPosition::new(pos),
          VertexNormal::new(pos),
        ));

        if x != res - 1 && y != res - 1 {
          let i = (x + y * res) as u32;
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

const DIRECTIONS: [Vek3<f32>; 6] = [
  Vek3::new(1., 0., 0.),
  Vek3::new(0., 1., 0.),
  Vek3::new(0., 0., 1.),
  Vek3::new(-1., 0., 0.),
  Vek3::new(0., -1., 0.),
  Vek3::new(0., 0., -1.),
];

pub fn mk_sphere(
  surface: &mut impl GraphicsContext<Backend = Backend>,
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
