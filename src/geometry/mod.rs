use luminance::tess::Mode;
use luminance::{context::GraphicsContext, tess::TessError};
use luminance_front::{tess::Tess, Backend};

use crate::shaders::QuadPosition;
use crate::shaders::QuadUv;
use crate::shaders::QuadVertex;

pub mod naive;
pub mod util;
pub mod ico;

pub fn mk_quad(
  surface: &mut impl GraphicsContext<Backend = Backend>,
) -> Result<Tess<QuadVertex, u32>, TessError> {
  let vertices: Vec<QuadVertex> = vec![
    QuadVertex::new(QuadPosition::new([-1., -1.]), QuadUv::new([0., 0.])),
    QuadVertex::new(QuadPosition::new([1., -1.]), QuadUv::new([1., 0.])),
    QuadVertex::new(QuadPosition::new([-1., 1.]), QuadUv::new([0., 1.])),
    QuadVertex::new(QuadPosition::new([-1., 1.]), QuadUv::new([0., 1.])),
    QuadVertex::new(QuadPosition::new([1., -1.]), QuadUv::new([1., 0.])),
    QuadVertex::new(QuadPosition::new([1., 1.]), QuadUv::new([1., 1.])),
  ];

  let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];

  surface
    .new_tess()
    .set_mode(Mode::Triangle)
    .set_vertices(vertices)
    .set_indices(indices)
    .build()
}
