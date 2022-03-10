use glsmrs::{Ctx, mesh::Mesh, AttributeVector3, AttributeScalar};

use crate::shaders::attributes::{PlanetVertex, VertexIndex};

pub trait Wavefront: Sized {
  fn vertices(&self) -> &[PlanetVertex];
  fn indices(&self) -> &[VertexIndex];

  fn to_tess(
    &self,
    ctx: &Ctx,
  ) -> Result<Mesh, String> {

    let positions = self.vertices().iter().map(|v| v.position.into_array()).collect::<Vec<[f32; 3]>>();
    let norms = self.vertices().iter().map(|v| v.norm.into_array()).collect::<Vec<[f32; 3]>>();
    let elevations = self.vertices().iter().map(|v| v.elevation).collect::<Vec<f32>>();

    Mesh::new(&ctx, self.indices())?
      .with_attribute::<AttributeVector3>("position", &positions)?
      .with_attribute::<AttributeVector3>("norm", &norms)?
      .with_attribute::<AttributeScalar>("elevation", &elevations)
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
