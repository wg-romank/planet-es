use bracket_noise::prelude::FastNoise;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use vek::Vec3 as Vek3;

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
pub struct DiffuseLightParameters {
  pub shadow_map_size: u32,
  pub intensity: f32,
  pub near_clip: f32,
  pub far_clip: f32,
  pub width: f32,
  pub position: Vek3<f32>,
  pub debug_shadows: bool,
}

impl DiffuseLightParameters {
  pub fn new() -> Self {
    Self {
      shadow_map_size: 800,
      intensity: 1.,
      near_clip: 5.18,
      far_clip: 7.37,
      width: 1.,
      position: Vek3::new(2.8, 3.57, 4.45),
      debug_shadows: false,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightingParameters {
  pub ambient: f32,
  pub diffuse: DiffuseLightParameters,
}

impl LightingParameters {
  pub fn new() -> Self {
    Self {
      ambient: 0.3,
      diffuse: DiffuseLightParameters::new(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RenderParameters {
  pub mode: RenderMode,
  pub fov: f32,
  pub scale: f32,
  pub sharpness: f32,
  pub light: LightingParameters,
  pub face_resolution: usize,
  pub radius: f32,
  pub mesh_parameters: MeshParameters,
  pub texture_parameters: TextureParameters,
}

impl RenderParameters {
  pub fn new() -> Self {
    Self {
      mode: RenderMode::Display,
      fov: 90.,
      scale: 1.,
      sharpness: 1.,
      light: LightingParameters::new(),
      face_resolution: 4,
      radius: 0.6,
      mesh_parameters: MeshParameters::new(),
      texture_parameters: TextureParameters::new(),
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
      MeshFilterType::Plain => (noise.get_noise3d(shifted.x, shifted.y, shifted.z) + 1.) * 0.5,
      MeshFilterType::Ridge => {
        let noise_r = noise.get_noise3d(shifted.x, shifted.y, shifted.z);
        1. - noise_r.abs()
      }
    };

    f32::max(0., noise - self.min_value) * self.strength
  }
}

#[cfg(target_arch = "wasm32")]
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
  filters: Vec<MeshFilterParameters>,
}

impl MeshParameters {
  pub fn new() -> Self {
    Self {
      frequency: 0.5,
      use_first_layer_as_mask: false,
      filters: vec![MeshFilterParameters::default()],
      // filters: vec![],
    }
  }

  pub fn evaluate(&self, noise: &FastNoise, point: Vek3<f32>) -> f32 {
    if let Some(first) = self.filters.first() {
      let first_value = if first.enabled {
        first.evaluate(noise, point)
      } else {
        0.0
      };

      if first_value > 0. && self.use_first_layer_as_mask || !self.use_first_layer_as_mask {
        self.filters[1..]
          .iter()
          .fold((first_value, self.frequency), |(v, m), f| {
            if f.enabled {
              (v + m * f.evaluate(noise, point), m * self.frequency)
            } else {
              (v, m * self.frequency)
            }
          })
          .0
      } else {
        first_value
      }
    } else {
      0.
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TextureParameters {
  pub color: [f32; 3],
  pub extrude_scale: f32,
}

impl TextureParameters {
  fn new() -> Self {
    Self {
      color: [0.68, 0.48, 0.],
      extrude_scale: 0.1,
    }
  }

}
