use luminance::{tess::{TessError, Mode}, context::GraphicsContext};
use luminance_front::{tess::Tess, Backend};

use crate::shaders::{ObjVertex, VertexIndex};

pub trait Mesh: Sized {
  fn vertices(&self) -> &[ObjVertex];
  fn indices(&self) -> &[VertexIndex];

  fn to_tess(
    &self,
    surface: &mut impl GraphicsContext<Backend = Backend>,
  ) -> Result<Tess<ObjVertex, u32>, TessError> {
    surface
      .new_tess()
      .set_mode(Mode::Triangle)
      .set_vertices(self.vertices())
      .set_indices(self.indices())
      .build()
  }

  fn to_obj(&self) -> String {
    let mut result = String::new();
    result.push_str("o Planet\n");

    self
      .vertices()
      .iter()
      .map(|v| v.position.repr)
      .for_each(|v| result.push_str(&format!("v {} {} {}\n", v[0], v[1], v[2])));
    self
      .vertices()
      .iter()
      .map(|v| v.norm.repr)
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
