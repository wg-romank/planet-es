use std::collections::HashMap;
use std::collections::HashSet;

use crate::shaders::ObjVertex;
use crate::shaders::QuadPosition;
use crate::shaders::QuadUv;
use crate::shaders::QuadVertex;
use crate::shaders::VertexIndex;
use crate::shaders::VertexNormal;
use crate::shaders::VertexPosition;

use crate::parameters::RenderParameters;

use luminance::context::GraphicsContext;
use luminance_front::tess::{Mode, Tess, TessError};
use luminance_front::Backend;

use bracket_noise::prelude::FastNoise;
use vek::Vec3 as Vek3;

use crate::log;

#[derive(Debug)]
pub struct Face {
  vertices: Vec<ObjVertex>,
  indices: Vec<VertexIndex>,
  border: HashSet<usize>,
}

impl Face {
  fn face_normal(vertices: &Vec<Vek3<f32>>, i1: usize, i2: usize, i3: usize) -> Vek3<f32> {
    let v = vertices[i2] - vertices[i1];
    let w = vertices[i3] - vertices[i1];

    Vek3::new(
      v.y * w.z - v.z * w.y,
      v.z * w.x - v.x * w.z,
      v.x * w.y - v.y * w.x,
    )
  }

  fn neighboor_triangles(
    vertices: &Vec<Vek3<f32>>,
    i: i32,
    res: i32,
  ) -> Vec<(usize, usize, usize)> {
    vec![
      (i - res - 1, i, i - 1),
      (i - res - 1, i - res, i),
      (i - res, i + 1, i),
      (i, i + 1, i + res + 1),
      (i, i + res + 1, i + res),
      (i - 1, i, i + res),
    ]
    .into_iter()
    .filter(|&(i1, i2, i3)| {
      i1 >= 0
        && i1 < vertices.len() as i32
        && i2 >= 0
        && i2 < vertices.len() as i32
        && i3 >= 0
        && i3 < vertices.len() as i32
    })
    .map(|(i1, i2, i3)| (i1 as usize, i2 as usize, i3 as usize))
    .collect()
  }

  fn vertex_normal(vertices: &Vec<Vek3<f32>>, idx: usize, res: usize) -> Vek3<f32> {
    Face::neighboor_triangles(vertices, idx as i32, res as i32)
      .into_iter()
      .map(|(i1, i2, i3)| Face::face_normal(vertices, i1, i2, i3))
      .sum::<Vek3<f32>>()
      .normalized()
  }

  fn make_normals(vertices: &Vec<Vek3<f32>>, res: usize) -> Vec<Vek3<f32>> {
    let mut result: Vec<Vek3<f32>> = vec![];
    for i in 0..vertices.len() {
      result.push(Face::vertex_normal(vertices, i, res));
    }
    result
  }

  fn make_vertices(
    dir: &Vek3<f32>,
    parameters: &RenderParameters,
    noise: &FastNoise,
  ) -> (Vec<Vek3<f32>>, Vec<VertexIndex>, HashSet<usize>) {
    let mut vertices = Vec::<Vek3<f32>>::new();
    let mut indices = Vec::<VertexIndex>::new();
    let mut border_vertices = HashSet::<usize>::new();

    let mut i = 0;

    let axis_a = Vek3::new(dir.y, dir.z, dir.x);
    let axis_b = dir.cross(axis_a);

    let res = parameters.face_resolution;

    for y in 0..res {
      for x in 0..res {
        let scale_x = x as f32 / (res as f32 - 1.);
        let scale_y = y as f32 / (res as f32 - 1.);

        let poin_on_unit_sphere =
          (dir + (scale_x * 2. - 1.) * axis_a + (scale_y * 2. - 1.) * axis_b).normalized();

        let mesh_offset = parameters
          .mesh_parameters
          .evaluate(noise, poin_on_unit_sphere);
        let vertex = poin_on_unit_sphere * parameters.radius * (1. + mesh_offset);

        vertices.push(vertex);

        if x != res - 1 && y != res - 1 {
          let i = (x + y * res) as u32;
          indices.push(i);
          indices.push(i + res as u32 + 1);
          indices.push(i + res as u32);

          indices.push(i);
          indices.push(i + 1);
          indices.push(i + res as u32 + 1);
        }

        if x == 0 || y == 0 || x == res - 1 || y == res - 1 {
          border_vertices.insert(i);
        }
        i += 1;
      }
    }

    (vertices, indices, border_vertices)
  }

  fn new(dir: &Vek3<f32>, parameters: &RenderParameters, noise: &FastNoise) -> Face {
    let (vs, indices, border) = Face::make_vertices(dir, parameters, noise);
    let normals = Face::make_normals(&vs, parameters.face_resolution);

    log!("vs {} no {}", vs.len(), normals.len());

    let vertices = vs
      .into_iter()
      .zip(normals.into_iter())
      .map(|(v, n)| {
        ObjVertex::new(
          VertexPosition::new(v.into_array()),
          VertexNormal::new(n.into_array()),
        )
      })
      .collect();

    Face {
      vertices,
      indices,
      border,
    }
  }
}

const DIRECTIONS: [Vek3<f32>; 6] = [
  Vek3::new(1., 0., 0.),
  Vek3::new(0., 1., 0.),
  Vek3::new(0., 0., 1.),
  Vek3::new(-1., 0., 0.),
  Vek3::new(0., -1., 0.),
  Vek3::new(0., 0., -1.),
];

pub struct Coordinate {
  res: usize,
  face_idx: usize,
  i: usize,
  j: usize,
}

impl Coordinate {
  fn index_at_border(&self) -> bool {
    let c = &self;
    c.i == 0 || c.j == 0 || c.i == c.res - 1 || c.j == c.res - 1
  }

  fn neightboor_triangles(&self) -> Vec<(Coordinate, Coordinate, Coordinate)> {
    todo!()
  }
}

// impl PartialEq for Coordinate {
//   fn eq(&self, other: &Self) -> bool {
//     if self.index_at_border() && other.index_at_border() {
//       match self.face_idx {
//         0 => todo!()
//       }
//     } else {
//       self.face_idx == other.face_idx && self.i == other.i && self.j == other.j
//     }
//   }
// }

pub struct Planet {
  // right, up, forward, left, down, backward
  // faces: Vec<Face>,
  vertices: Vec<ObjVertex>,
  indices: Vec<VertexIndex>,
}

impl Planet {
  pub fn new(parameters: &RenderParameters) -> Self {
    let noise = FastNoise::new();
    let faces = DIRECTIONS
           .iter()
           .map(|dir| Face::new(dir, parameters, &noise))
           .collect();
    let (vertices, indices) = Self::faces_to_single_mesh(faces);

    Self { vertices, indices }
  }

  fn faces_to_single_mesh(faces: Vec<Face>) -> (Vec<ObjVertex>, Vec<VertexIndex>) {
    let vertices = faces
      .iter()
      .flat_map(|f| f.vertices.iter().map(|&v| v))
      .collect();
    let len = faces[0].vertices.len();
    let indices = faces
      .iter()
      .enumerate()
      .flat_map(|(face_id, f)|
        f
          .indices
          .iter()
          .map(move |idx| *idx + (face_id * len) as u32)
        )
      .collect();

    (vertices, indices)
  }

  fn adjust_normals(faces: Vec<Face>) -> Vec<Face> {
    // for each vertex on the border
    // loop through other borders to see if there are any matching
    // get their triangles
    // compute single normal
    // update existing values
    // let border_ids = faces
    //   .iter()
    //   .enumerate()
    //   .map(|(i, f)| (i, f.border))
    //   .collect::<Vec<(usize, HashSet<usize>)>>();
    todo!();
  }

  pub fn to_tess(
    self,
    surface: &mut impl GraphicsContext<Backend = Backend>,
  ) -> Result<Tess<ObjVertex, u32>, TessError> {
    surface 
      .new_tess()
      .set_mode(Mode::Triangle)
      .set_vertices(self.vertices)
      .set_indices(self.indices)
      .build()
  }
}

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
