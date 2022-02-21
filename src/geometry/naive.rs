
use crate::shaders::attributes::PlanetVertex;
use crate::shaders::attributes::VertexIndex;
use crate::shaders::attributes::VertexNormal;
use crate::shaders::attributes::VertexPosition;

use crate::parameters::RenderParameters;
use crate::geometry::util::Mesh;

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

#[derive(Clone, Copy)]
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

  const DIRECTIONS_DIRS: [Vek3<f32>; 6] = [
    Vek3::new(1., 0., 0.),
    Vek3::new(0., 1., 0.),
    Vek3::new(0., 0., 1.),
    Vek3::new(-1., 0., 0.),
    Vek3::new(0., -1., 0.),
    Vek3::new(0., 0., -1.),
  ];

  fn direction(&self) -> Vek3<f32> {
    Direction::DIRECTIONS_DIRS[*self as usize]
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

  fn opposite(&self) -> Direction {
    use Direction::*;
    match self {
      Right => Left,
      Up => Down,
      Forward => Backward,
      Left => Right,
      Down => Up,
      Backward => Forward,
    }
  }

  fn neightboor_left(&self) -> Direction {
    use Direction::*;
    match self {
      Right => Backward,
      Up => Left,
      Forward => Left,
      Left => Backward,
      Down => Right,
      Backward => Right,
    }
  }

  fn neightboor_right(&self) -> Direction {
    self.neightboor_left().opposite()
  }

  fn neightboor_up(&self) -> Direction {
    use Direction::*;
    match self {
      Right => Up,
      Up => Forward,
      Forward => Up,
      Left => Up,
      Down => Right,
      Backward => Backward,
    }
  }

  fn neightboor_down(&self) -> Direction {
    self.neightboor_up().opposite()
  }

}

pub struct Planet {
  pub vertices: Vec<PlanetVertex>,
  pub indices: Vec<VertexIndex>,
}

impl Planet {
  fn face_normal(vertices: &[Vek3<f32>], i1: usize, i2: usize, i3: usize) -> Vek3<f32> {
    let v = vertices[i2] - vertices[i1];
    let w = vertices[i3] - vertices[i1];

    //
    // let cos_theta = v.dot(w) / (v.magnitude() * w.magnitude());
    // let sin_theta = (1. - cos_theta.powf(2.)).sqrt();
    // let area = 0.5 * v * v * sin_theta;

    // area *
    Vek3::new(
      v.y * w.z - v.z * w.y,
      v.z * w.x - v.x * w.z,
      v.x * w.y - v.y * w.x,
    )
  }

  fn neighboor_triangles(
    vertices: &[Vek3<f32>],
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
    vertices: &[Vek3<f32>],
    idx: usize,
    res: usize,
  ) -> Vek3<f32> {
    Self::neighboor_triangles(vertices, idx as i32, res as i32)
      .into_iter()
      .map(|(i1, i2, i3)| Self::face_normal(vertices, i1, i2, i3))
      .sum::<Vek3<f32>>()
  }

  fn make_normals(vertices: &Vec<Vek3<f32>>, res: usize, face_stride: usize) -> Vec<Vek3<f32>> {
    let mut result: Vec<Vek3<f32>> = vec![];
    for dir in Direction::DIRECTIONS.iter() {
      let d = *dir as usize;
      let view = &vertices[d * face_stride..(d + 1) * face_stride];

      for i in 0..view.len() {
        let adj = if i % res == 0 {
          // left
          let n = dir.neightboor_left() as usize;
          let neighboor_view = &vertices[n * face_stride..(n + 1) * face_stride];
          let n_idx = i + (res - 1);
          Self::vertex_normal(neighboor_view, n_idx, res)
        } else if i % res == res - 1 {
          // right
          let n = dir.neightboor_right() as usize;
          let neighboor_view = &vertices[n * face_stride..(n + 1) * face_stride];
          let n_idx = i - (res - 1);
          Self::vertex_normal(neighboor_view, n_idx, res)
        // } else if i <= res - 1 {
        //   // up
        //   let n = dir.neightboor_up() as usize;
        //   let neighboor_view = &vertices[n * face_stride..(n + 1) * face_stride];
        //   let n_idx = i + (res - 1) * (res - 1);
        //   Self::vertex_normal(neighboor_view, n_idx, res)
        // } else if i >= res * (res - 1) {
        //   // down
        //   let n = dir.neightboor_down() as usize;
        //   let neighboor_view = &vertices[n * face_stride..(n + 1) * face_stride];
        //   let n_idx = i - (res - 1) * (res - 1);
        //   Self::vertex_normal(neighboor_view, n_idx, res)
        } else {
          Vek3::<f32>::new(0., 0., 0.)
        };

        result.push((Self::vertex_normal(view, i, res) + adj).normalized());
      }
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
        PlanetVertex::new(
          VertexPosition::new(v.into_array()),
          VertexNormal::new(n.into_array()),
          todo!(),
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
}

impl Mesh for Planet {
  fn vertices(&self) -> &[PlanetVertex] {
    &self.vertices
  }

  fn indices(&self) -> &[VertexIndex] {
    &self.indices
  }
}
