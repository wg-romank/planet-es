use bracket_noise::prelude::FastNoise;
use icosahedron::Polyhedron;

use crate::{
  parameters::RenderParameters,
  shaders::attributes::{PlanetVertex, VertexIndex},
};

use crate::geometry::util::Wavefront;

use vek::Vec3 as Vek3;
use vek::Vec2 as Vek2;

pub struct IcoPlanet {
  pub vertices: Vec<PlanetVertex>,
  pub indices: Vec<VertexIndex>,
}

impl IcoPlanet {
  pub fn new(parameters: &RenderParameters) -> Self {
    let noise = FastNoise::new();

    let mut ico = Polyhedron::new_isocahedron(parameters.radius, parameters.face_resolution as u32);

    let mut max_height = f32::MIN;
    let mut min_height = f32::MAX;

    let mut hs: Vec<f32> = vec![];

    let pi = core::f64::consts::PI as f32;

    let uvs = ico.positions.iter().map(|p| {
      let lat = f32::asin(p.0.y);
      let lon = f32::atan2(p.0.x, -p.0.z);

      let u = lat / (2. * pi);
      let v = (lon / pi + 1.) / 2.;

      (u, v)
    }).collect::<Vec<(f32, f32)>>();

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
      .zip(uvs.iter())
      .map(|(((p, n), e), uv)| {
        let elevation_normalized = (*e - min_height) / (max_height - min_height);
        PlanetVertex::new(
          Vek3::new(p.0.x, p.0.y, p.0.z),
          Vek3::new(n.0.x, n.0.y, n.0.z),
          elevation_normalized,
          Vek2::new(uv.0, uv.1),
        )
      })
      .collect();

    let indices = ico
      .cells
      .iter()
      .flat_map(|t| vec![t.a as VertexIndex, t.b as VertexIndex, t.c as VertexIndex])
      .collect();

    Self { vertices, indices }
  }
}

impl Wavefront for IcoPlanet {
  fn vertices(&self) -> &[PlanetVertex] {
    &self.vertices
  }

  fn indices(&self) -> &[VertexIndex] {
    &self.indices
  }
}
