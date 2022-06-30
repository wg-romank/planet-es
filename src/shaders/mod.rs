pub mod attributes;
pub mod uniforms;
pub mod vertex_render_data;
pub mod util;

use std::collections::HashMap;

use gl::Pipeline;
use gl::Program;
use gl::GL;
use gl::Ctx;
use gl::UniformData;
use gl::mesh::Mesh;
use gl::texture::DepthFrameBuffer;
use gl::texture::EmptyFramebuffer;
use gl::texture::InternalFormat;
use gl::texture::Viewport;
use gl::texture::{Framebuffer, UploadedTexture, TextureSpec};

use crate::shaders::vertex_render_data::VertexRenderData;
use crate::shaders::util::to_png_texture;
use crate::shaders::util::tex_unis;
use crate::geometry::ico::IcoPlanet;
use crate::geometry::mk_quad;
use crate::geometry::util::Wavefront;
use crate::parameters::RenderParameters;

const VS_STR: &str = include_str!("../../shaders/display.vert");
const FS_STR: &str = include_str!("../../shaders/display.frag");

const DEBUG_VS_STR: &str = include_str!("../../shaders/quad.vert");
const DEBUG_FS_STR: &str = include_str!("../../shaders/debug_shadows.frag");

const MERCURY: &[u8] = include_bytes!("../../assets/mercury.jpg");
// const WAVES_1: &[u8; 355381] = include_bytes!("../../assets/water3.png");

use glsmrs as gl;

pub struct Render {
  pub ctxt: Ctx,
  pub planet_mesh: IcoPlanet,
  pipeline: Pipeline,
  planet: Mesh,
  quad: Mesh,
  program: Program,
  shadow_program: Program,
  debug_program: Program,
  display_fb: EmptyFramebuffer,
  shadow_fb: DepthFrameBuffer,
  color_map: Option<UploadedTexture>,
  // waves_texture1: TextureHandle,
  // waves_texture2: Texture<Dim2, RGBA32F>,
  canvas_viewport: Viewport,
  pub vertex_render_data: VertexRenderData,
}

impl Render {
  pub fn from(
    ctxt: Ctx,
    parameters: &RenderParameters,
    canvas_viewport: Viewport,
  ) -> Result<Render, String> {
    let program = gl::Program::new(&ctxt, VS_STR, FS_STR)?;
    let shadow_program = gl::Program::new(&ctxt, VS_STR, "void main() {}")?;
    let debug_program = gl::Program::new(&ctxt, DEBUG_VS_STR, DEBUG_FS_STR)?;

    let display_fb = EmptyFramebuffer::new(&ctxt, canvas_viewport);

    let planet_mesh = IcoPlanet::new(&parameters);

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
      shadow_fb: Self::make_shadow_map(&ctxt, parameters)?,
      display_fb,
      color_map: None,
      // waves_texture1,
      // waves_texture2,
      canvas_viewport,
      vertex_render_data: VertexRenderData::new(&ctxt, &canvas_viewport, parameters)?,
    })
  }

  pub fn update_mesh(&mut self, parameters: &RenderParameters) {
    self.planet_mesh = IcoPlanet::new(&parameters);
    self.planet = self
      .planet_mesh
      .to_tess(&self.ctxt)
      .expect("failed to create planet");

    self.vertex_render_data.update(&self.canvas_viewport, parameters)
  }

  pub fn update_hm(&mut self, bytes: &[u8]) -> Result<(), String> {
    self.vertex_render_data.update_hm(&self.ctxt, bytes)
  }

  pub fn update_cm(&mut self, bytes: &[u8]) -> Result<(), String> {
    self.color_map = Some(to_png_texture(&self.ctxt, bytes)?);

    Ok(())
  }

  pub fn frame(&mut self, elapsed: f32, parameters: &RenderParameters) {
    self.shadow_pass(&parameters, elapsed);

    if parameters.light.diffuse.debug_shadows {
      self.debug_pass();
    } else {
      self.display_pass(&parameters, elapsed)
    }
  }
}

impl Render {
  fn cm(cm: &mut Option<UploadedTexture>) -> Vec<(&'static str, UniformData)> {
    tex_unis("color_map", "color_map_size", cm)
  }

  fn shadow_pass(&mut self, parameters: &RenderParameters, elapsed: f32) {
    let uni_values = self.vertex_render_data.compute_unis(parameters, elapsed);

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
    elapsed: f32,
  ) {
    let uni_values = vec![
      // todo waves
      // todo height map
      ("lightPosition", UniformData::Vector3(parameters.light.position.into_array())),
      ("specular_strength", UniformData::Scalar(parameters.light.specular.specular_strength)),
      ("specular_falloff", UniformData::Scalar(parameters.light.specular.specular_falloff)),
      ("shadow_map", UniformData::Texture(self.shadow_fb.depth_slot())),
      ("color", UniformData::Vector3(parameters.texture_parameters.color)),
      ("mode", UniformData::Scalar(parameters.mode.in_shader())),
      // ("blend", UniformData::Vector3(paramters.blend)),
      ("scale", UniformData::Scalar(parameters.scale)),
      ("sharpness", UniformData::Scalar(parameters.sharpness)),
      ("ambient", UniformData::Scalar(parameters.light.ambient)),
      ("diffuse_intensity", UniformData::Scalar(parameters.light.diffuse.intensity)),
    ].into_iter().chain(self.vertex_render_data.compute_unis(parameters, elapsed)).chain(Self::cm(&mut self.color_map)).collect::<HashMap<_, _>>();

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
  fn make_shadow_map(ctxt: &Ctx, parameters: &RenderParameters) -> Result<DepthFrameBuffer, String> {
    let side = parameters.light.diffuse.shadow_map_size;
    let (w, h) = (side, side);
    let shadow_texture_spec = TextureSpec::depth([w, h]);
    let shadow_texture = shadow_texture_spec.upload(ctxt, InternalFormat(GL::UNSIGNED_INT), None)?;

    EmptyFramebuffer::new(&ctxt, gl::texture::Viewport::new(w, h))
      .with_depth_slot(shadow_texture)
  }
}
