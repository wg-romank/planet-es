pub mod attributes;
pub mod uniforms;

use std::collections::HashMap;

use gl::Pipeline;
use gl::Program;
use gl::GL;
use gl::Ctx;
use gl::UniformData;
use gl::mesh::Mesh;
use gl::texture::InternalFormat;
use gl::texture::InterpolationMag;
use gl::texture::InterpolationMin;
use gl::texture::Viewport;
use gl::texture::WrapS;
use gl::texture::WrapT;
use gl::texture::{Framebuffer, TextureHandle, TextureSpec, ColorFormat};
use image::{DynamicImage, GenericImageView, ImageFormat};
use vek::{FrustumPlanes, Mat4, Vec3 as Vek3};

use crate::geometry::ico::IcoPlanet;
use crate::geometry::mk_quad;
use crate::geometry::util::Wavefront;
use crate::parameters::RenderParameters;

use crate::log;

const VS_STR: &str = include_str!("../../shaders/display.vert");
const FS_STR: &str = include_str!("../../shaders/display.frag");

const SHADOW_VS_STR: &str = include_str!("../../shaders/shadow.vert");
const SHADOW_FS_STR: &str = include_str!("../../shaders/shadow.frag");

const DEBUG_VS_STR: &str = include_str!("../../shaders/quad.vert");
const DEBUG_FS_STR: &str = include_str!("../../shaders/debug_shadows.frag");

// const WAVES_1: &[u8; 355381] = include_bytes!("../../assets/water3.png");

use glsmrs as gl;

pub fn to_png_texture(ctx: &Ctx, bytes: &[u8]) -> Result<TextureHandle, String> {
  let texture = image::load_from_memory_with_format(bytes, ImageFormat::Png)
    .map_err(|e| format!("{:?}", e))?;

  // log!("texture {:?}", texture);

  let (width, height) = texture.dimensions();
  let tex = texture
    .as_rgb8()
    .ok_or(format!("not rgb8"))?;

  TextureSpec::new(ColorFormat(GL::RGB), [width, height])
    .wrap_s(WrapS(GL::REPEAT))
    .wrap_t(WrapT(GL::REPEAT))
    .upload_u8(&ctx, &tex.as_raw())
}

pub struct Render {
  pub ctxt: Ctx,
  pub planet_mesh: IcoPlanet,
  pipeline: Pipeline,
  planet: Mesh,
  quad: Mesh,
  program: Program,
  shadow_program: Program,
  debug_program: Program,
  shadow_fb: Framebuffer<(), TextureHandle>,
  height_map: TextureHandle,
  // waves_texture1: TextureHandle,
  // waves_texture2: Texture<Dim2, RGBA32F>,
  model: Mat4<f32>,
  light_model: Mat4<f32>,
}

impl Render {
  pub fn from(
    ctxt: Ctx,
    parameters: &RenderParameters,
  ) -> Result<Render, String> {
    let program = gl::Program::new(&ctxt, VS_STR, FS_STR)?;
    let shadow_program = gl::Program::new(&ctxt, SHADOW_VS_STR, SHADOW_FS_STR)?;
    let debug_program = gl::Program::new(&ctxt, DEBUG_VS_STR, DEBUG_FS_STR)?;

    let shadow_texture_spec = TextureSpec::depth([800, 800]);

    let shadow_texture = shadow_texture_spec.upload(&ctxt, InternalFormat(GL::UNSIGNED_INT), None)?;

    let shadow_fb = Framebuffer::new(&ctxt, gl::texture::Viewport::new(800, 800))?
      .with_depth_slot(&ctxt, shadow_texture);

    let mut height_map_spec = TextureSpec::new(ColorFormat(GL::RGBA), [100, 1]);
    height_map_spec.interpolation_min = InterpolationMin(GL::NEAREST);
    height_map_spec.interpolation_mag = InterpolationMag(GL::NEAREST);

    let height_map = height_map_spec.upload_rgba(&ctxt, &parameters.texture_parameters.to_bytes())?;

    let planet_mesh = IcoPlanet::new(&parameters);

    let planet = planet_mesh
      .to_tess(&ctxt)
      .expect("failed to create planet");

    let quad = mk_quad(&ctxt).expect("failed to make quad");

    // let waves_texture1 = to_png_texture(&mut ctxt, WAVES_1)?;
    // let waves_texture2 = to_png_texture(&mut ctxt, WAVES_2);

    let pipeline = Pipeline::new(&ctxt, Viewport::new(400, 400));

    Ok(Render {
      ctxt,
      planet_mesh,
      pipeline,
      planet,
      quad,
      program,
      shadow_program,
      debug_program,
      shadow_fb,
      height_map,
      // waves_texture1,
      // waves_texture2,
      model: Self::compute_model(&parameters),
      light_model: Self::compute_light_model(&parameters),
    })
  }

  pub fn update_mesh(&mut self, parameters: &RenderParameters) {
    self.planet_mesh = IcoPlanet::new(&parameters);
    self.planet = self
      .planet_mesh
      .to_tess(&self.ctxt)
      .expect("failed to create planet");

    self.model = Self::compute_model(parameters);
    self.light_model = Self::compute_light_model(parameters);
  }

  pub fn update_texture(&mut self, texture: DynamicImage) -> Result<(), String> {
    let (width, height) = texture.dimensions();
    // todo: pret
    let texels = texture
      .as_rgb8()
      .map(|img| {
        img
          .as_raw()
          .chunks(3)
          .map(|c| [c[0] as f32 / 255., c[1] as f32 / 255., c[2] as f32 / 255., 1. as f32])
          .collect::<Vec<[f32; 4]>>()
      })
      .ok_or(format!("not rgba {:?}", texture))?;


    let mut spec = TextureSpec::new(ColorFormat(GL::RGBA), [width, height]);
    spec.interpolation_min = InterpolationMin(GL::NEAREST);
    spec.interpolation_mag = InterpolationMag(GL::NEAREST);
    self.height_map = spec.upload_rgba(&self.ctxt, &texels)?;

    Ok(())
  }

  fn shadow_pass(&mut self, rotation: &Mat4<f32>) {
    let uni_values = vec![
      ("rotation", UniformData::Matrix4(rotation.into_col_array())),
      ("light_model", UniformData::Matrix4(self.light_model.into_col_array()))
    ].into_iter().collect::<HashMap<_, _>>();

    self.pipeline.shade(
      &self.ctxt,
      &self.shadow_program,
      &uni_values,
      vec![&self.planet],
      Some(&mut self.shadow_fb)
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
      ("height_map", UniformData::Texture(self.height_map.clone())),
      ("mode", UniformData::Scalar(parameters.mode.in_shader())),
      // ("blend", UniformData::Vector3(paramters.blend)),
      ("scale", UniformData::Scalar(parameters.scale)),
      ("sharpness", UniformData::Scalar(parameters.sharpness)),
      ("ambient", UniformData::Scalar(parameters.light.ambient)),
      ("diffuse_intensity", UniformData::Scalar(parameters.light.diffuse.intensity)),
    ].into_iter().collect::<HashMap<_, _>>();

    self.pipeline.shade::<(), ()>(
      &self.ctxt,
      &self.program,
      &uni_values,
      vec![&self.planet],
      None,
    ).expect("error shadow pass");

  }

  pub fn debug_pass(&mut self) {
    let uni_values = vec![
      ("depth_map", UniformData::Texture(self.shadow_fb.depth_slot())),
    ].into_iter().collect::<HashMap<_, _>>();

    self.pipeline.shade::<(), ()>(
      &self.ctxt,
      &self.debug_program,
      &uni_values,
      vec![&self.quad],
      None,
    ).expect("error debug pass");
  }

  pub fn compute_model(parameters: &RenderParameters) -> Mat4<f32> {
    let projection = Mat4::perspective_fov_rh_no(
      parameters.fov / 180. * std::f32::consts::PI,
      400.,
      400.,
      0.1,
      10.,
    );

    let view: Mat4<f32> = Mat4::look_at_rh(Vek3::new(2., 0., 0.), Vek3::zero(), Vek3::unit_y());

    projection * view
  }

  pub fn compute_light_model(parameters: &RenderParameters) -> Mat4<f32> {
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

  pub fn frame(&mut self, elapsed: f32, parameters: &RenderParameters) {
    let rotation = vek::mat4::Mat4::identity()
      .rotated_y(elapsed * parameters.rotate_y_speed)
      .rotated_x(elapsed * parameters.rotate_x_speed);

    self.shadow_pass(&rotation);

    let normal_matrix = rotation.clone().inverted().transposed();

    if parameters.light.diffuse.debug_shadows {
      self.debug_pass();
    } else {
      self.display_pass(&parameters, &rotation, &normal_matrix)
    }
  }
}
