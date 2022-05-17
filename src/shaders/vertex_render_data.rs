use std::collections::HashMap;

use glsmrs::{texture::{UploadedTexture, Viewport}, UniformData, Ctx};
use vek::{Mat4, FrustumPlanes, Vec3 as Vek3, Vec4 as Vek4};

use crate::parameters::RenderParameters;
use crate::shaders::util::to_png_texture;
use crate::shaders::util::tex_unis;

use super::MERCURY;

pub struct VertexRenderData {
  light_model: Mat4<f32>,
  projection: Mat4<f32>,
  view: Mat4<f32>,
  rotation: Mat4<f32>,
  camera_rot_rel: Mat4<f32>,
  camera_rot: Mat4<f32>,
  height_map: Option<UploadedTexture>,
}

impl VertexRenderData {
  pub fn new(ctx: &Ctx, viewport: &Viewport, params: &RenderParameters) -> Result<Self, String> {
    let projection = Self::compute_proj(params, viewport);

    Ok(Self {
      light_model: Self::compute_light_model(params),
      projection: Self::compute_proj(params, viewport),
      view: Self::compute_view(Mat4::identity()),
      rotation: Mat4::identity(),
      camera_rot_rel: Mat4::identity(),
      camera_rot: Mat4::identity(),
      height_map: Some(to_png_texture(ctx, MERCURY)?),
    })
  }

  fn model(&self) -> Mat4<f32> {
    self.projection * self.view
  }

  pub fn compute_unis(&mut self, params: &RenderParameters, elapsed: f32) -> HashMap<&'static str, UniformData> {
    self.rotation = Mat4::identity().rotated_y(elapsed * params.rotation_speed);

    vec![
      ("normalMatrix", UniformData::Matrix4(self.rotation.inverted().transposed().into_col_array())),
      ("rotation", UniformData::Matrix4(self.rotation.into_col_array())),
      ("extrude_scale", UniformData::Scalar(params.texture_parameters.extrude_scale)),
      ("model", UniformData::Matrix4(self.model().into_col_array())),
      ("light_model", UniformData::Matrix4(self.light_model.into_col_array())),
    ].into_iter().chain(Self::hm(&mut self.height_map)).collect()
  }

  fn hm(hm: &mut Option<UploadedTexture>) -> Vec<(&'static str, UniformData)> {
    tex_unis("height_map", "height_map_size", hm)
  }

  pub fn update(&mut self, viewport: &Viewport, params: &RenderParameters) {
    self.projection = Self::compute_proj(params, viewport);
    self.light_model = Self::compute_light_model(params);
  }

  pub fn update_hm(&mut self, ctx: &Ctx, bytes: &[u8]) -> Result<(), String> {
    self.height_map = Some(to_png_texture(ctx, bytes)?);

    Ok(())
  }

  pub fn rotate(&mut self, leftright: f32, topdown: f32) {
    self.camera_rot = self.camera_rot_rel
      .rotated_y(leftright)
      .rotated_z(topdown);
    self.view = Self::compute_view(self.camera_rot);
  }

  pub fn set_rotated(&mut self) {
    self.camera_rot_rel = self.camera_rot.clone();
  }

  fn compute_proj(parameters: &RenderParameters, canvas_viewport: &Viewport) -> Mat4<f32> {
    Mat4::perspective_fov_rh_no(
      parameters.fov / 180. * std::f32::consts::PI,
      canvas_viewport.w as f32,
      canvas_viewport.h as f32,
      0.1,
      10.,
    )
  }

  fn compute_view(rotation: Mat4<f32>) -> Mat4<f32> {
    Mat4::look_at_rh((Vek4::new(2., 0., 0., 0.) * rotation).xyz(), Vek3::zero(), Vek3::unit_y())
  }

  fn compute_light_model(parameters: &RenderParameters) -> Mat4<f32> {
    let light_view: Mat4<f32> = Mat4::look_at_rh(
      parameters.light.diffuse.position,
      Vek3::zero(),
      Vek3::unit_y(),
    );

    let light_projection = Mat4::orthographic_rh_no(FrustumPlanes {
      left: -parameters.light.diffuse.width,
      right: parameters.light.diffuse.width,
      bottom: -parameters.light.diffuse.width,
      top: parameters.light.diffuse.width,
      near: parameters.light.diffuse.near_clip,
      far: parameters.light.diffuse.far_clip,
    });

    light_projection * light_view
  }

}
