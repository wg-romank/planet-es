pub mod attributes;
pub mod uniforms;

use luminance::pipeline::PipelineState;
use luminance::pixel::{Depth32F, RGBA32F};
use luminance::render_state::RenderState;
use luminance::shader::types::Mat44;
use luminance::texture::{Dim2, MagFilter, MinFilter, Sampler, TexelUpload};

use luminance_front::context::GraphicsContext;
use luminance_front::framebuffer::Framebuffer;
use luminance_front::shader::Program;
use luminance_front::tess::Tess;
use luminance_front::Backend;

use luminance_front::texture::Texture;
use vek::{FrustumPlanes, Mat4, Vec3 as Vek3};

use crate::geometry::ico::IcoPlanet;
use crate::geometry::mk_quad;
use crate::geometry::util::Mesh;
use crate::parameters::RenderParameters;

use crate::shaders::attributes::{PlanetVertex, PlanetVertexSemantics, QuadVertex, QuadVertexSemantics};
use crate::shaders::uniforms::{DebugShaderInterface, ShaderInterface, ShadowShaderInterface};

use crate::log;

const VS_STR: &str = include_str!("../../shaders/display_vs.glsl");
const FS_STR: &str = include_str!("../../shaders/display_fs.glsl");

const SHADOW_VS_STR: &str = include_str!("../../shaders/shadow_vs.glsl");
const SHADOW_FS_STR: &str = include_str!("../../shaders/shadow_fs.glsl");

const DEBUG_VS_STR: &str = include_str!("../../shaders/debug_vs.glsl");
const DEBUG_FS_STR: &str = include_str!("../../shaders/debug_shadows_fs.glsl");

pub struct Render<C> {
  pub ctxt: C,
  pub planet_mesh: IcoPlanet,
  planet: Tess<PlanetVertex, u32>,
  quad: Tess<QuadVertex, u32>,
  program: Program<PlanetVertexSemantics, (), ShaderInterface>,
  shadow_program: Program<PlanetVertexSemantics, (), ShadowShaderInterface>,
  debug_program: Program<QuadVertexSemantics, (), DebugShaderInterface>,
  shadow_fb: Framebuffer<Dim2, RGBA32F, Depth32F>,
  output_fb: Framebuffer<Dim2, (), ()>,
  height_map: Texture<Dim2, RGBA32F>,
  model: Mat44<f32>,
  light_model: Mat44<f32>,
}

impl<C> Render<C>
where
  C: GraphicsContext<Backend = Backend>,
{
  pub fn from(
    mut ctxt: C,
    parameters: &RenderParameters,
    output_fb: Framebuffer<Dim2, (), ()>,
  ) -> Render<C> {
    let program = ctxt
      .new_shader_program::<PlanetVertexSemantics, (), ShaderInterface>()
      .from_strings(VS_STR, None, None, FS_STR)
      .expect("failed to create program")
      .ignore_warnings();

    let shadow_program = ctxt
      .new_shader_program::<PlanetVertexSemantics, (), ShadowShaderInterface>()
      .from_strings(SHADOW_VS_STR, None, None, SHADOW_FS_STR)
      .expect("failed to create shadow program")
      .ignore_warnings();

    let debug_program = ctxt
      .new_shader_program::<QuadVertexSemantics, (), DebugShaderInterface>()
      .from_strings(DEBUG_VS_STR, None, None, DEBUG_FS_STR)
      .expect("failed to create debug program")
      .ignore_warnings();

    log!("parameters {:?}", parameters);

    let mut shadow_sampler = Sampler::default();

    shadow_sampler.min_filter = MinFilter::Nearest;
    shadow_sampler.mag_filter = MagFilter::Nearest;

    let shadow_fb = ctxt
      .new_framebuffer::<Dim2, RGBA32F, Depth32F>([800, 800], 0, shadow_sampler)
      .expect("unable to create shadow framebuffer");

    let height_map = ctxt
      .new_texture::<Dim2, RGBA32F>(
        [100, 1],
        Sampler::default(),
        TexelUpload::BaseLevel {
          texels: &parameters.texture_parameters.to_bytes(),
          mipmaps: 0,
        },
      )
      .expect("failed to create height map");

    let planet_mesh = IcoPlanet::new(&parameters);

    let planet = planet_mesh
      .to_tess(&mut ctxt)
      .expect("failed to create planet");

    let quad = mk_quad(&mut ctxt).expect("failed to make quad");

    Render {
      ctxt,
      planet_mesh,
      planet,
      quad,
      program,
      shadow_program,
      debug_program,
      shadow_fb,
      output_fb,
      height_map,
      model: Self::compute_model(&parameters),
      light_model: Self::compute_light_model(&parameters),
    }
  }

  pub fn update_mesh(&mut self, parameters: &RenderParameters) {
    self.planet_mesh = IcoPlanet::new(&parameters);
    self.planet = self
      .planet_mesh
      .to_tess(&mut self.ctxt)
      .expect("failed to create planet");

    self.height_map = self
      .ctxt
      .new_texture::<Dim2, RGBA32F>(
        [100, 1],
        Sampler::default(),
        TexelUpload::BaseLevel {
          texels: &parameters.texture_parameters.to_bytes(),
          mipmaps: 0,
        },
      )
      .expect("failed to create height map");

    self.model = Self::compute_model(parameters);
    self.light_model = Self::compute_light_model(parameters);
  }

  fn shadow_pass(&mut self, rotation: &Mat4<f32>) {
    let ctxt = &mut self.ctxt;
    let shadow_program = &mut self.shadow_program;
    let shadow_map = &mut self.shadow_fb;
    let planet = &self.planet;
    let light_model = &self.light_model;

    let shadow_render = ctxt
      .new_pipeline_gate()
      .pipeline(shadow_map, &PipelineState::default(), |_, mut shd_gate| {
        shd_gate.shade(shadow_program, |mut iface, uni, mut rdr_gate| {
          rdr_gate.render(&RenderState::default(), |mut tess_gate| {
            iface.set(&uni.rotation, rotation.into_col_arrays().into());
            iface.set(&uni.light_model, *light_model);

            tess_gate.render(planet)
          })
        })
      })
      .assume();

    if !shadow_render.is_ok() {
      log!("error shadow rendering {:?}", shadow_render.into_result());
    }
  }

  fn display_pass(
    &mut self,
    parameters: &RenderParameters,
    rotation: &Mat4<f32>,
    normal_matrix: &Mat4<f32>,
  ) {
    let ctxt = &mut self.ctxt;
    let program = &mut self.program;
    let shadow_map = &mut self.shadow_fb;
    let height_map = &mut self.height_map;
    let back_buffer = &self.output_fb;
    let planet = &self.planet;
    let light_model = &self.light_model;
    let model = &self.model;

    let render = ctxt
      .new_pipeline_gate()
      .pipeline(
        &back_buffer,
        &PipelineState::default(),
        |pipeline, mut shd_gate| {
          let sh_m = pipeline
            .bind_texture(shadow_map.depth_stencil_slot())
            .expect("failed to bind depth texture");
          let hi_m = pipeline
            .bind_texture(height_map)
            .expect("failed to bind height map");

          shd_gate.shade(program, |mut iface, uni, mut rdr_gate| {
            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
              iface.set(&uni.rotation, rotation.into_col_arrays().into());
              iface.set(&uni.normal_matrix, normal_matrix.into_col_arrays().into());
              iface.set(
                &uni.light_position,
                parameters.light.diffuse.position.into_array().into(),
              );

              iface.set(&uni.model, *model);
              iface.set(&uni.light_model, *light_model);

              iface.set(&uni.shadow_map, sh_m.binding());
              iface.set(&uni.height_map, hi_m.binding());

              iface.set(&uni.mode, parameters.mode.in_shader());

              tess_gate.render(planet)
            })
          })
        },
      )
      .assume();

    if !render.is_ok() {
      log!("error rendering {:?}", render.into_result());
    }
  }

  pub fn debug_pass(&mut self) {
    let ctxt = &mut self.ctxt;
    let program = &mut self.debug_program;
    let shadow_map = &mut self.shadow_fb;
    let back_buffer = &self.output_fb;
    let quad = &self.quad;

    let render = ctxt
      .new_pipeline_gate()
      .pipeline(
        &back_buffer,
        &PipelineState::default(),
        |pipeline, mut shd_gate| {
          let sh_m = pipeline
            .bind_texture(shadow_map.depth_stencil_slot())
            .expect("failed to bind depth texture");

          shd_gate.shade(program, |mut iface, uni, mut rdr_gate| {
            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
              iface.set(&uni.depth_map, sh_m.binding());

              tess_gate.render(quad)
            })
          })
        },
      )
      .assume();

    if !render.is_ok() {
      log!("error rendering {:?}", render.into_result());
    }
  }

  pub fn compute_model(parameters: &RenderParameters) -> Mat44<f32> {
    let projection = Mat4::perspective_fov_rh_no(
      parameters.fov / 180. * std::f32::consts::PI,
      400.,
      400.,
      0.1,
      10.,
    );

    let view: Mat4<f32> = Mat4::look_at_rh(Vek3::new(2., 0., 0.), Vek3::zero(), Vek3::unit_y());

    (projection * view).into_col_arrays().into()
  }

  pub fn compute_light_model(parameters: &RenderParameters) -> Mat44<f32> {
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

    (light_projection * light_view).into_col_arrays().into()
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