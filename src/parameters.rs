use bracket_noise::prelude::FastNoise;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

use vek::{Vec3 as Vek3};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RenderMode {
  Normals,
  Uvs,
  Display,
}

impl RenderMode {
  pub fn in_shader(&self) -> f32 {
    match self {
      Self::Normals => 0.,
      Self::Uvs => 1.,
      Self::Display => 2.,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RenderParameters {
  pub mode: RenderMode,
  pub fov: f32,
  pub light_position: Vek3<f32>,
  pub rotate_x_speed: f32,
  pub rotate_y_speed: f32,
  pub debug_shadows: bool,
  pub color: [f32; 4],
  pub face_resolution: usize,
  pub radius: f32,
  pub mesh_parameters: MeshParameters,
}

impl RenderParameters {
   pub fn new() -> Self {
    Self {
      mode: RenderMode::Display,
      fov: 45.,
      light_position: Vek3::new(2.8, 3.57, 4.45),
      rotate_x_speed: 0.,
      rotate_y_speed: 0.,
      debug_shadows: false,
      color: [0.68, 0.48, 0., 1.],
      face_resolution: 32,
      radius: 0.57,
      mesh_parameters: MeshParameters::new(),
    }
   } 
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MeshFilterType {
  Plain,
  Ridge,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MeshFilterParameters {
  tup: MeshFilterType,
  strength: f32,
  roughness: f32,
  min_value: f32,
  center: Vek3<f32>,
  enabled: bool,
}

impl MeshFilterParameters {
  pub fn evaluate(&self, noise: &FastNoise, point: Vek3<f32>) -> f32 {
    let shifted = point * self.roughness + self.center;

    let noise = match self.tup {
      MeshFilterType::Plain => {
        (noise.get_noise3d(
          shifted.x,
          shifted.y,
          shifted.z
        ) + 1.) * 0.5
      },
      MeshFilterType::Ridge => {
        let noise_r = noise.get_noise3d(shifted.x, shifted.y, shifted.z);
        1. - noise_r.abs()
      }
    };

    f32::max(0., noise - self.min_value) * self.strength
  }
}

#[wasm_bindgen]
impl MeshFilterParameters {
  pub fn generate() -> String {
    serde_json::to_string(&MeshFilterParameters::default()).unwrap()
  }
}

impl Default for MeshFilterParameters {
    fn default() -> Self {
      Self {
          tup: MeshFilterType::Plain,
          strength: 0.14,
          roughness: 1.38,
          min_value: 0.54,
          center: Vek3::new(0., 0., 0.),
          enabled: true,
      }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MeshParameters {
  frequency: f32,
  use_first_layer_as_mask: bool,
  filters: Vec<MeshFilterParameters>
}

impl MeshParameters {
  pub fn new() -> Self {
    Self {
      frequency: 0.5,
      use_first_layer_as_mask: false,
      filters: vec![
        MeshFilterParameters::default(),
      ],
    }
  }

  pub fn evaluate(&self, noise: &FastNoise, point: Vek3<f32>) -> f32 {
    if let Some(first) = self.filters.first() {
      let first_value = if first.enabled { first.evaluate(noise, point) } else { 0.0 };

      if first_value > 0. && self.use_first_layer_as_mask || !self.use_first_layer_as_mask {
        self.filters[1..].iter().fold((first_value, self.frequency), |(v, m), f| {
          if f.enabled {
            (v + m * f.evaluate(noise, point), m * self.frequency)
          } else {
            (v, m * self.frequency)
          }
        }).0
      } else { first_value }
    } else { 0. }
  }
}
