use bracket_noise::prelude::FastNoise;
use icosahedron::Polyhedron;
use vek::Vec3 as Vek3;

use crate::{
  parameters::RenderParameters,
  shaders::{ObjVertex, VertexColor, VertexIndex, VertexNormal, VertexPosition},
};

use crate::geometry::util::Mesh;

pub struct IcoPlanet {
  pub vertices: Vec<ObjVertex>,
  pub indices: Vec<VertexIndex>,
}

impl IcoPlanet {
  pub fn new(parameters: &RenderParameters) -> Self {
    let noise = FastNoise::new();

    let mut ico = Polyhedron::new_isocahedron(
      parameters.radius,
      parameters.face_resolution as u32,
    );

    ico.positions.iter_mut().for_each(|p| {
      // todo: get rid of ugly hack
      let pp = Vek3::new(p.0.x, p.0.y, p.0.z);
      let mesh_offset = parameters
        .mesh_parameters
        .evaluate(&noise, pp);
      let res = pp * parameters.radius * (1. + mesh_offset);

      p.0.x = res.x;
      p.0.y = res.y;
      p.0.z = res.z;
    });

    ico.compute_face_normals();
    ico.compute_triangle_normals();

    let vertices = ico
      .positions
      .iter()
      .zip(ico.normals.iter())
      .map(|(p, n)| {
        ObjVertex::new(
          VertexPosition::new([p.0.x, p.0.y, p.0.z]),
          VertexNormal::new([n.0.x, n.0.y, n.0.z]),
          VertexColor::new([0.4, 0.2, 0.5]),
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
