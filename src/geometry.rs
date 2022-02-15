use crate::shaders::ObjVertex;
use crate::shaders::QuadPosition;
use crate::shaders::QuadUv;
use crate::shaders::QuadVertex;
use crate::shaders::VertexColor;
use crate::shaders::VertexIndex;
use crate::shaders::VertexNormal;
use crate::shaders::VertexPosition;

use crate::parameters::RenderParameters;

use luminance_front::context::GraphicsContext;
use luminance_front::tess::{Mode, Tess, TessError};
use luminance_front::Backend;

use bracket_noise::prelude::FastNoise;
use vek::Vec3 as Vek3;

#[derive(Debug)]
pub struct Face {
  vertices: Vec<Vek3<f32>>,
  indices: Vec<VertexIndex>,
  uvs: Vec<(f32, f32)>,
}

impl Face {
  fn make_vertices(
    d: &Direction,
    parameters: &RenderParameters,
    noise: &FastNoise,
  ) -> (Vec<Vek3<f32>>, Vec<VertexIndex>, Vec<(f32, f32)>) {
    let mut vertices = Vec::<Vek3<f32>>::new();
    let mut indices = Vec::<VertexIndex>::new();
    let mut uvs = Vec::<(f32, f32)>::new();

    let dir = d.direction();
    let axis_a = d.axis_a().direction();
    let axis_b = d.direction().cross(axis_a);

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
        uvs.push((scale_x, scale_y));
      }
    }

    (vertices, indices, uvs)
  }

  fn new(dir: &Direction, parameters: &RenderParameters, noise: &FastNoise) -> Face {
    let (vertices, indices, uvs) = Face::make_vertices(dir, parameters, noise);

    Face {
      vertices,
      indices,
      uvs,
    }
  }
}

enum Direction {
  Right, Up, Forward,
  Left, Down, Backward,
}

impl Direction {
  const DIRECTIONS: [Direction; 6] = [
    Direction::Right,
    Direction::Up,
    Direction::Forward,
    Direction::Left,
    Direction::Down,
    Direction::Backward
  ];

  fn direction(&self) -> Vek3<f32> {
    match self {
      Direction::Right => Vek3::new(1., 0., 0.),
      Direction::Up => Vek3::new(0., 1., 0.),
      Direction::Forward => Vek3::new(0., 0., 1.),
      Direction::Left => Vek3::new(-1., 0., 0.),
      Direction::Down => Vek3::new(0., -1., 0.),
      Direction::Backward => Vek3::new(0., 0., -1.),
    }
  }

  fn axis_a(&self) -> Direction {
    use Direction::*;
    match self {
      Right => Forward,
      Up => Right,
      Forward => Left,
      Left => Backward,
      Down => Left,
      Backward => Right,
    }
  }

  fn neightboors_x(&self) -> Vec<Direction> {
    use Direction::*;
    match self {
      Right => vec![],
      Up => vec![],
      Forward => vec![],
      Left => vec![],
      Down => vec![],
      Backward => vec![],
    }
  }

}

pub struct Coordinate {
  res: usize,
  face_stride: usize,
  face_id: usize,
  i: usize,
  j: usize,
}

impl Coordinate {
  fn new(idx: usize, res: usize, face_stride: usize) -> Self {
    todo!()
  }

  fn is_index_at_border(&self) -> bool {
    let c = &self;
    c.i == 0 || c.j == 0 || c.i == c.res - 1 || c.j == c.res - 1
  }

  fn is_corner_point(&self) -> bool {
    self.i == self.j
  }

  fn neightboor_triangles(&self) -> Vec<(Coordinate, Coordinate, Coordinate)> {
    if self.is_index_at_border() {
      if self.is_corner_point() {
        // 3 meshes
        todo!()
      } else {
        // 2 meshes
        match self.face_id {
          // left border
          0 if self.j == 0 => todo!("neighboor id 5"),
          0 if self.j == self.res - 1 => todo!("neighboor id 2"),
          0 if self.i == 0 => todo!("neighboor id 1"),
          0 if self.i == self.res - 1 => todo!("neighboor id 4"),

          _ => panic!("unexpected"),
        }
      }
    } else {
      todo!()
    }
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

#[derive(Clone)]
pub struct Planet {
  pub vertices: Vec<ObjVertex>,
  pub indices: Vec<VertexIndex>,
}

impl Planet {
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

  fn vertex_normal(
    vertices: &Vec<Vek3<f32>>,
    idx: usize,
    res: usize,
    face_stride: usize,
  ) -> Vek3<f32> {
    Self::neighboor_triangles(vertices, idx as i32, res as i32)
      .into_iter()
      .map(|(i1, i2, i3)| Self::face_normal(vertices, i1, i2, i3))
      .sum::<Vek3<f32>>()
      .normalized()
  }

  fn make_normals(vertices: &Vec<Vek3<f32>>, res: usize, face_stride: usize) -> Vec<Vek3<f32>> {
    let mut result: Vec<Vek3<f32>> = vec![];
    for i in 0..vertices.len() {
      result.push(Self::vertex_normal(vertices, i, res, face_stride));
    }
    result
  }

  pub fn new(parameters: &RenderParameters) -> Self {
    let noise = FastNoise::new();
    let faces = Direction::DIRECTIONS
      .iter()
      .map(|dir| Face::new(dir, parameters, &noise))
      .collect::<Vec<Face>>();
    let face_stride = faces[0].vertices.len();
    let (vs, indices, uvs) = Self::faces_to_single_mesh(faces, face_stride);

    let normals = Self::make_normals(&vs, parameters.face_resolution, face_stride);

    let vertices = vs
      .into_iter()
      .zip(normals.into_iter())
      .zip(uvs.into_iter())
      .map(|((v, n), u)| {
        ObjVertex::new(
          VertexPosition::new(v.into_array()),
          VertexNormal::new(n.into_array()),
          VertexColor::new([u.0, u.1, 0.5]),
        )
      })
      .collect();

    Self { vertices, indices }
  }

  fn faces_to_single_mesh(
    faces: Vec<Face>,
    face_stride: usize,
  ) -> (Vec<Vek3<f32>>, Vec<VertexIndex>, Vec<(f32, f32)>) {
    let vertices = faces
      .iter()
      .flat_map(|f| f.vertices.iter().map(|&v| v))
      .collect();

    let uvs = faces
      .iter()
      .flat_map(|f| f.uvs.iter().map(|&u| u))
      .collect();
    let indices = faces
      .iter()
      .enumerate()
      .flat_map(|(face_id, f)| {
        f.indices
          .iter()
          // right, up, forward, left, down, backward
          .map(move |idx| *idx + (face_id * face_stride) as u32)
      })
      .collect();

    (vertices, indices, uvs)
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

  pub fn to_obj(&self) -> String {
    let mut result = String::new();
    result.push_str("o Planet\n");

    self
      .vertices
      .iter()
      .map(|v| v.position.repr)
      .for_each(|v| result.push_str(&format!("v {} {} {}\n", v[0], v[1], v[2])));
    self
      .vertices
      .iter()
      .map(|v| v.norm.repr)
      .for_each(|v| result.push_str(&format!("vn {} {} {}\n", v[0], v[1], v[2])));

    self.indices.chunks(3).for_each(|v| {
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
