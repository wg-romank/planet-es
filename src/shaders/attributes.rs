
use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum PlanetVertexSemantics {
  #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
  Position,

  #[sem(name = "norm", repr = "[f32; 3]", wrapper = "VertexNormal")]
  Normal,

  #[sem(name = "elevation", repr = "f32", wrapper = "VertexElevation")]
  Elevation,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "PlanetVertexSemantics")]
pub struct PlanetVertex {
  pub position: VertexPosition,
  pub norm: VertexNormal,
  pub elevation: VertexElevation,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum QuadVertexSemantics {
  #[sem(name = "position", repr = "[f32; 2]", wrapper = "QuadPosition")]
  Position,

  #[sem(name = "uv", repr = "[f32; 2]", wrapper = "QuadUv")]
  Uv,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "QuadVertexSemantics")]
pub struct QuadVertex {
  pub position: QuadPosition,
  pub uv: QuadUv,
}

pub type VertexIndex = u32;