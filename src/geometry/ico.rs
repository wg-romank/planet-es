use bracket_noise::prelude::FastNoise;
use icosahedron::Polyhedron;

use crate::log;

use crate::{
  parameters::RenderParameters,
  shaders::{ObjVertex, VertexElevation, VertexIndex, VertexNormal, VertexPosition},
};

use crate::geometry::util::Mesh;

pub struct IcoPlanet {
  pub vertices: Vec<ObjVertex>,
  pub indices: Vec<VertexIndex>,
}

impl IcoPlanet {
  pub fn new(parameters: &RenderParameters) -> Self {
    let noise = FastNoise::new();

    let mut ico = Polyhedron::new_isocahedron(parameters.radius, parameters.face_resolution as u32);

    let mut max_height = f32::MIN;
    let mut min_height = f32::MAX;

    let mut hs: Vec<f32> = vec![];

    ico.positions.iter_mut().for_each(|p| {
      let pp = p.0;
      let mesh_offset = parameters.mesh_parameters.evaluate(&noise, pp);
      let res = pp * parameters.radius * (1. + mesh_offset);

      max_height = max_height.max(mesh_offset);
      min_height = min_height.min(mesh_offset);

      hs.push(mesh_offset);

      p.0 = res;
    });

    // log!("{:?}", hs);

    ico.compute_face_normals();
    ico.compute_triangle_normals();

    let vertices = ico
      .positions
      .iter()
      .zip(ico.normals.iter())
      .zip(hs.iter())
      .map(|((p, n), e)| {
        let elevation_normalized = (*e - min_height) / (max_height - min_height);
        ObjVertex::new(
          VertexPosition::new([p.0.x, p.0.y, p.0.z]),
          VertexNormal::new([n.0.x, n.0.y, n.0.z]),
          VertexElevation::new(elevation_normalized),
        )
      })
      .collect();

    let indices = ico
      .cells
      .iter()
      .flat_map(|t| vec![t.a as u32, t.b as u32, t.c as u32])
      .collect();

    Self { vertices, indices }
  }
}

impl Mesh for IcoPlanet {
  fn vertices(&self) -> &[ObjVertex] {
    &self.vertices
  }

  fn indices(&self) -> &[VertexIndex] {
    &self.indices
  }
}
