use luminance::{
  pipeline::TextureBinding,
  pixel::Floating,
  shader::{
    types::{Mat44, Vec3},
    Uniform,
  },
  texture::Dim2,
  UniformInterface,
};

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
  #[uniform(name = "rotation", unbound)]
  pub rotation: Uniform<Mat44<f32>>,

  #[uniform(name = "normalMatrix", unbound)]
  pub normal_matrix: Uniform<Mat44<f32>>,

  #[uniform(name = "lightPosition", unbound)]
  pub light_position: Uniform<Vec3<f32>>,

  #[uniform(name = "model", unbound)]
  pub model: Uniform<Mat44<f32>>,

  #[uniform(name = "light_model", unbound)]
  pub light_model: Uniform<Mat44<f32>>,

  #[uniform(unbound)]
  pub shadow_map: Uniform<TextureBinding<Dim2, Floating>>,

  #[uniform(unbound)]
  pub height_map: Uniform<TextureBinding<Dim2, Floating>>,

  #[uniform(unbound)]
  pub mode: Uniform<f32>,

  #[uniform(unbound)]
  pub waves_1: Uniform<TextureBinding<Dim2, Floating>>,

  #[uniform(unbound)]
  pub waves_2: Uniform<TextureBinding<Dim2, Floating>>,

  #[uniform(unbound)]
  pub blend: Uniform<Vec3<f32>>,

  #[uniform(unbound)]
  pub scale: Uniform<f32>,

  #[uniform(unbound)]
  pub sharpness: Uniform<f32>,

  #[uniform(unbound)]
  pub ambient: Uniform<f32>,

  #[uniform(unbound)]
  pub diffuse_intensity: Uniform<f32>,
}

#[derive(Debug, UniformInterface)]
pub struct ShadowShaderInterface {
  #[uniform(name = "rotation", unbound)]
  pub rotation: Uniform<Mat44<f32>>,

  #[uniform(name = "light_model", unbound)]
  pub light_model: Uniform<Mat44<f32>>,
}

#[derive(Debug, UniformInterface)]
pub struct DebugShaderInterface {
  #[uniform(unbound)]
  pub depth_map: Uniform<TextureBinding<Dim2, Floating>>,
}
