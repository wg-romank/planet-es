use glsmrs::{
  attributes::{AttributeScalar, AttributeVector2, AttributeVector3},
  mesh::Mesh,
  Ctx,
};

use crate::shaders::attributes::{PlanetVertex, VertexIndex};

use vek::Vec3 as Vek3;

const PI: f32 = core::f64::consts::PI as f32;

pub fn xyz_to_latlonuv(p: Vek3<f32>) -> (f32, f32) {
  let lon = f32::atan2(p.x, -p.z); // [-pi, pi]
  let u = 1. - (lon + PI) / (2. * PI);

  let lat = f32::asin(p.y); // [-pi/2, pi/2]
  let v = 1. - (lat + PI / 2.) / PI;

  (u, v)
}

pub trait Wavefront: Sized {
  fn vertices(&self) -> &[PlanetVertex];
  fn indices(&self) -> &[VertexIndex];

  fn to_tess(&self, ctx: &Ctx) -> Result<Mesh, String> {
    let positions = self
      .vertices()
      .iter()
      .map(|v| v.position.into_array())
      .collect::<Vec<[f32; 3]>>();
    let norms = self
      .vertices()
      .iter()
      .map(|v| v.norm.into_array())
      .collect::<Vec<[f32; 3]>>();
    let elevations = self
      .vertices()
      .iter()
      .map(|v| v.elevation)
      .collect::<Vec<f32>>();
    let uvs = self
      .vertices()
      .iter()
      .map(|v| v.uv.into_array())
      .collect::<Vec<[f32; 2]>>();

    Mesh::new(&ctx, self.indices())?
      .with_attribute::<AttributeVector3>("position", &positions)?
      .with_attribute::<AttributeVector3>("norm", &norms)?
      .with_attribute::<AttributeScalar>("elevation", &elevations)?
      .with_attribute::<AttributeVector2>("uv", &uvs)
  }

  fn to_obj(&self) -> String {
    let mut result = String::new();
    result.push_str("o Planet\n");

    self
      .vertices()
      .iter()
      .map(|v| v.position)
      .for_each(|v| result.push_str(&format!("v {} {} {}\n", v[0], v[1], v[2])));
    self
      .vertices()
      .iter()
      .map(|v| v.norm)
      .for_each(|v| result.push_str(&format!("vn {} {} {}\n", v[0], v[1], v[2])));

    self.indices().chunks(3).for_each(|v| {
      result.push_str(&format!(
        "f {x}//{x} {y}//{y} {z}//{z}\n",
        x = v[0] + 1,
        y = v[1] + 1,
        z = v[2] + 1,
      ))
    });

    result
  }
}
