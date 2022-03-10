use vek::Vec3 as Vek3;
use vek::Vec2 as Vek2;

#[derive(Clone, Copy, Debug)]
pub struct PlanetVertex {
  pub position: Vek3<f32>,
  pub norm: Vek3<f32>,
  pub elevation: f32,
}

impl PlanetVertex {
  pub fn new(position: Vek3<f32>, norm: Vek3<f32>, elevation: f32) -> Self {
    Self { position, norm, elevation }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct QuadVertex {
  pub position: Vek2<f32>,
  pub uv: Vek2<f32>,
}

impl QuadVertex {
  pub fn new(position: Vek2<f32>, uv: Vek2<f32>) -> Self {
    Self { position, uv }
  }
}

pub type VertexIndex = u16;