use glsmrs::{attributes::AttributeVector2, mesh::Mesh, Ctx};

use crate::shaders::attributes::QuadVertex;

use vek::Vec2 as Vek2;

pub mod ico;
pub mod util;

pub fn mk_quad(ctx: &Ctx) -> Result<Mesh, String> {
  let vertices: Vec<QuadVertex> = vec![
    QuadVertex::new(Vek2::new(-1., -1.), Vek2::new(0., 0.)),
    QuadVertex::new(Vek2::new(1., -1.), Vek2::new(1., 0.)),
    QuadVertex::new(Vek2::new(-1., 1.), Vek2::new(0., 1.)),
    QuadVertex::new(Vek2::new(-1., 1.), Vek2::new(0., 1.)),
    QuadVertex::new(Vek2::new(1., -1.), Vek2::new(1., 0.)),
    QuadVertex::new(Vek2::new(1., 1.), Vek2::new(1., 1.)),
  ];

  let indices: Vec<u16> = vec![0, 1, 2, 3, 4, 5];

  let positions = vertices
    .iter()
    .map(|v| v.position.into_array())
    .collect::<Vec<[f32; 2]>>();
  let uvs = vertices
    .iter()
    .map(|v| v.uv.into_array())
    .collect::<Vec<[f32; 2]>>();

  Mesh::new(ctx, &indices)?
    .with_attribute::<AttributeVector2>("position", &positions)?
    .with_attribute::<AttributeVector2>("uv", &uvs)
}
