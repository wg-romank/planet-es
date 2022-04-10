pub mod attributes;
pub mod uniforms;

use std::collections::HashMap;
use std::ops::Deref;

use gl::Pipeline;
use gl::Program;
use gl::GL;
use gl::Ctx;
use gl::UniformData;
use gl::mesh::Mesh;
use gl::texture::DepthFrameBuffer;
use gl::texture::EmptyFramebuffer;
use gl::texture::InternalFormat;
use gl::texture::InterpolationMag;
use gl::texture::InterpolationMin;
use gl::texture::Viewport;
use gl::texture::{Framebuffer, UploadedTexture, TextureSpec, ColorFormat};
use image::GenericImageView;
use vek::{FrustumPlanes, Mat4, Vec3 as Vek3};

use crate::geometry::ico::IcoPlanet;
use crate::geometry::mk_quad;
use crate::geometry::naive::Planet;
use crate::geometry::util::Wavefront;
use crate::parameters::RenderParameters;

const VS_STR: &str = include_str!("../../shaders/display.vert");
const FS_STR: &str = include_str!("../../shaders/display.frag");

const SHADOW_VS_STR: &str = include_str!("../../shaders/shadow.vert");
const SHADOW_FS_STR: &str = include_str!("../../shaders/shadow.frag");

const DEBUG_VS_STR: &str = include_str!("../../shaders/quad.vert");
const DEBUG_FS_STR: &str = include_str!("../../shaders/debug_shadows.frag");

// const WAVES_1: &[u8; 355381] = include_bytes!("../../assets/water3.png");

use glsmrs as gl;

fn to_png_texture(ctx: &Ctx, bytes: &[u8]) -> Result<HeigtMap, String> {
  let texture = image::load_from_memory(bytes)
    .map_err(|e| format!("{:?}", e))?;

  let (width, height) = texture.dimensions();
  let tex = texture.into_rgb8();

  let hm = TextureSpec::new(ColorFormat(GL::RGB), [width, height])
    .upload_u8(&ctx, &tex.as_raw())?;

  Ok(HeigtMap {
    texture: hm,
    size: [width as f32, height as f32]
  })
}

pub struct HeigtMap {
  texture: UploadedTexture,
  size: [f32; 2]
}

pub struct Render {
  pub ctxt: Ctx,
  pub planet_mesh: Planet,
  pipeline: Pipeline,
  planet: Mesh,
  quad: Mesh,
  program: Program,
  shadow_program: Program,
  debug_program: Program,
  display_fb: EmptyFramebuffer,
  shadow_fb: DepthFrameBuffer,
  height_map: Option<HeigtMap>,
  // waves_texture1: TextureHandle,
  // waves_texture2: Texture<Dim2, RGBA32F>,
  model: Mat4<f32>,
  light_model: Mat4<f32>,
  canvas_viewport: Viewport,
  rotation: Mat4<f32>,
  rotation_rel: Mat4<f32>,
}

impl Render {
  pub fn from(
    ctxt: Ctx,
    parameters: &RenderParameters,
    canvas_viewport: Viewport,
  ) -> Result<Render, String> {
    let program = gl::Program::new(&ctxt, VS_STR, FS_STR)?;
    let shadow_program = gl::Program::new(&ctxt, SHADOW_VS_STR, SHADOW_FS_STR)?;
    let debug_program = gl::Program::new(&ctxt, DEBUG_VS_STR, DEBUG_FS_STR)?;

    let display_fb = EmptyFramebuffer::new(&ctxt, canvas_viewport);

    let planet_mesh = Planet::new(&parameters);

    let planet = planet_mesh
      .to_tess(&ctxt)
      .expect("failed to create planet");

    let quad = mk_quad(&ctxt).expect("failed to make quad");

    // let waves_texture1 = to_png_texture(&mut ctxt, WAVES_1)?;
    // let waves_texture2 = to_png_texture(&mut ctxt, WAVES_2);

    let pipeline = Pipeline::new(&ctxt);

    Ok(Render {
      ctxt: ctxt.clone(),
      planet_mesh,
      pipeline,
      planet,
      quad,
      program,
      shadow_program,
      debug_program,
      shadow_fb: Self::make_shadow_map(&ctxt, &parameters)?,
      display_fb,
      height_map: None,
      // waves_texture1,
      // waves_texture2,
      model: Self::compute_model(&parameters, &canvas_viewport),
      light_model: Self::compute_light_model(&parameters),
      canvas_viewport,
      rotation: Mat4::identity(),
      rotation_rel: Mat4::identity(),
    })
  }

  pub fn update_mesh(&mut self, parameters: &RenderParameters) {
    self.planet_mesh = Planet::new(&parameters);
    self.planet = self
      .planet_mesh
      .to_tess(&self.ctxt)
      .expect("failed to create planet");

    self.model = Self::compute_model(parameters, &self.canvas_viewport);
    self.light_model = Self::compute_light_model(parameters);
  }

  pub fn update_texture(&mut self, bytes: &[u8]) -> Result<(), String> {
    self.height_map = Some(to_png_texture(&self.ctxt, bytes)?);

    Ok(())
  }

  pub fn rotate(&mut self, leftright: f32, topdown: f32) {
    self.rotation_rel = self.rotation
      .rotated_y(leftright)
      .rotated_z(topdown);
  }

  pub fn set_rotated(&mut self) {
    self.rotation = self.rotation_rel.clone();
  }

  pub fn frame(&mut self, elapsed: f32, parameters: &RenderParameters) {
    let rotation = self.rotation_rel.clone();
    self.shadow_pass(&parameters, &rotation);

    let normal_matrix = rotation.clone().inverted().transposed();

    if parameters.light.diffuse.debug_shadows {
      self.debug_pass();
    } else {
      self.display_pass(&parameters, &rotation, &normal_matrix)
    }
  }
}

impl Render {
  fn hm(height_map: &mut Option<HeigtMap>) -> Vec<(&'static str, UniformData)> {
    if let Some(hm) = height_map.as_mut() {
      vec![("height_map", UniformData::Texture(&mut hm.texture)),
      ("height_map_size", UniformData::Vector2(hm.size))]
    } else {
      vec![]
    }
  }

  fn shadow_pass(&mut self, parameters: &RenderParameters, rotation: &Mat4<f32>) {
    let uni_values = vec![
      ("rotation", UniformData::Matrix4(rotation.into_col_array())),
      ("extrude_scale", UniformData::Scalar(parameters.texture_parameters.extrude_scale)),
      ("light_model", UniformData::Matrix4(self.light_model.into_col_array())),
    ].into_iter().chain(Self::hm(&mut self.height_map)).collect::<HashMap<_, _>>();

    self.pipeline.shade(
      &self.shadow_program,
      uni_values,
      vec![&mut self.planet],
      &mut self.shadow_fb
    ).expect("error shadow pass");
  }

  fn display_pass(
    &mut self,
    parameters: &RenderParameters,
    rotation: &Mat4<f32>,
    normal_matrix: &Mat4<f32>,
  ) {
    let uni_values = vec![
      // todo waves
      // todo height map
      ("rotation", UniformData::Matrix4(rotation.into_col_array())),
      ("normalMatrix", UniformData::Matrix4(normal_matrix.into_col_array())),
      ("lightPosition", UniformData::Vector3(parameters.light.diffuse.position.into_array())),
      ("model", UniformData::Matrix4(self.model.into_col_array())),
      ("light_model", UniformData::Matrix4(self.light_model.into_col_array())),
      ("shadow_map", UniformData::Texture(self.shadow_fb.depth_slot())),
      ("color", UniformData::Vector3(parameters.texture_parameters.color)),
      ("extrude_scale", UniformData::Scalar(parameters.texture_parameters.extrude_scale)),
      ("mode", UniformData::Scalar(parameters.mode.in_shader())),
      // ("blend", UniformData::Vector3(paramters.blend)),
      ("scale", UniformData::Scalar(parameters.scale)),
      ("sharpness", UniformData::Scalar(parameters.sharpness)),
      ("ambient", UniformData::Scalar(parameters.light.ambient)),
      ("diffuse_intensity", UniformData::Scalar(parameters.light.diffuse.intensity)),
    ].into_iter().chain(Self::hm(&mut self.height_map)).collect::<HashMap<_, _>>();

    self.pipeline.shade(
      &self.program,
      uni_values,
      vec![&mut self.planet],
      &mut self.display_fb,
    ).expect("error shadow pass");

  }

  fn debug_pass(&mut self) {
    let uni_values = vec![
      ("depth_map", UniformData::Texture(self.shadow_fb.depth_slot())),
      // ("depth_map", UniformData::Texture(&mut self.height_map)),
    ].into_iter().collect::<HashMap<_, _>>();

    self.pipeline.shade(
      &self.debug_program,
      uni_values,
      vec![&mut self.quad],
      &mut self.display_fb,
    ).expect("error debug pass");
  }
}

impl Render {
  fn compute_model(parameters: &RenderParameters, canvas_viewport: &Viewport) -> Mat4<f32> {
    let projection = Mat4::perspective_fov_rh_no(
      parameters.fov / 180. * std::f32::consts::PI,
      canvas_viewport.w as f32,
      canvas_viewport.h as f32,
      0.1,
      10.,
    );

    let view: Mat4<f32> = Mat4::look_at_rh(Vek3::new(2., 0., 0.), Vek3::zero(), Vek3::unit_y());

    projection * view
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

  fn make_shadow_map(ctxt: &Ctx, parameters: &RenderParameters) -> Result<DepthFrameBuffer, String> {
    let side = parameters.light.diffuse.shadow_map_size;
    let (w, h) = (side, side);
    let shadow_texture_spec = TextureSpec::depth([w, h]);
    let shadow_texture = shadow_texture_spec.upload(ctxt, InternalFormat(GL::UNSIGNED_INT), None)?;

    EmptyFramebuffer::new(&ctxt, gl::texture::Viewport::new(w, h))
      .with_depth_slot(shadow_texture)
  }
}
