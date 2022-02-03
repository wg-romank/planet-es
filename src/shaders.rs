use luminance::context::GraphicsContext;
use luminance::render_state::RenderState;
use luminance::shader::{Uniform};
use luminance::shader::types::{Mat44, Vec4, Vec3};
use luminance::UniformInterface;
use luminance::pixel::{Depth32F, R8I, Floating};
use luminance::texture::{Sampler, Dim2};
use luminance::pipeline::{TextureBinding, PipelineState};

use luminance_derive::{Semantics, Vertex};

use luminance_front::framebuffer::Framebuffer;
use luminance_front::Backend;
use luminance_front::shader::Program;
use luminance_front::tess::Tess;

use vek::{Vec3 as Vek3, Mat4};

use crate::geometry::mk_sphere;
use crate::parameters::RenderParameters;

use crate::log;

const VS_STR: &str = include_str!("../shaders/vs.glsl");
const FS_STR: &str = include_str!("../shaders/fs.glsl");

const SHADOW_VS_STR: &str = include_str!("../shaders/shadow_vs.glsl");
const SHADOW_FS_STR: &str = include_str!("../shaders/shadow_fs.glsl");


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum VertexSemantics {
  #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
  Position,

  #[sem(name = "norm", repr = "[f32; 3]", wrapper = "VertexNormal")]
  Normal,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct ObjVertex {
  position: VertexPosition,
  norm: VertexNormal,
}

pub type VertexIndex = u32;

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
  #[uniform(name = "rotation", unbound)]
  rotation: Uniform<Mat44<f32>>,

  #[uniform(name = "normalMatrix", unbound)]
  normal_matrix: Uniform<Mat44<f32>>,

  #[uniform(name = "color", unbound)]
  color: Uniform<Vec4<f32>>,

  #[uniform(name = "lightPosition", unbound)]
  light_position: Uniform<Vec3<f32>>,

  #[uniform(name = "projection", unbound)]
  projection: Uniform<Mat44<f32>>,

  #[uniform(unbound)]
  view: Uniform<Mat44<f32>>,

  #[uniform(unbound)]
  light_view: Uniform<Mat44<f32>>,

  #[uniform(unbound)]
  shadow_map: Uniform<TextureBinding<Dim2, Floating>>,
}

#[derive(Debug, UniformInterface)]
struct ShadowShaderInterface {
  #[uniform(name = "rotation", unbound)]
  rotation: Uniform<Mat44<f32>>,

  #[uniform(name = "projection", unbound)]
  projection: Uniform<Mat44<f32>>,

  #[uniform(unbound)]
  light_view: Uniform<Mat44<f32>>,
}

pub struct Render<C> {
  ctxt: C,
  sphere: Vec<Tess<ObjVertex, u32>>,
  program: Program<VertexSemantics, (), ShaderInterface>,
  shadow_program: Program<VertexSemantics, (), ShadowShaderInterface>,
  shadow_fb: Framebuffer<Dim2, R8I, Depth32F>,
  output_fb: Framebuffer<Dim2, (), ()>,
}

impl<C> Render<C> where C : GraphicsContext<Backend = Backend> {
  pub fn from(mut ctxt: C, parameters: &RenderParameters, output_fb: Framebuffer<Dim2, (), ()>) -> Render<C> {
    let program = ctxt
      .new_shader_program::<VertexSemantics, (), ShaderInterface>()
      .from_strings(VS_STR, None, None, FS_STR)
      .expect("failed to create program")
      .ignore_warnings();

    let shadow_program = ctxt
      .new_shader_program::<VertexSemantics, (), ShadowShaderInterface>()
      .from_strings(SHADOW_VS_STR, None, None, SHADOW_FS_STR)
      .expect("failed to create shadow program")
      .ignore_warnings();

    log!("parameters {:?}", parameters);

    let shadow_fb = ctxt.new_framebuffer::<Dim2, R8I, Depth32F>(
      [400, 400], 0, Sampler::default()
    ).expect("unable to create shadow framebuffer");

    let sphere = mk_sphere(&mut ctxt, &parameters);

    Render {
      ctxt,
      sphere,
      program,
      shadow_program,
      shadow_fb,
      output_fb,
    }
  }

  pub fn update_mesh(&mut self, parameters: &RenderParameters) {
      self.sphere = mk_sphere(&mut self.ctxt, parameters);
  }

  pub fn frame(&mut self, elapsed: f32, parameters: &RenderParameters) {
    // let color = [elapsed.cos(), elapsed.sin(), 0.5, 1.];
    let color = parameters.color;
    let light_position = parameters.light_position;

    let sphere = &self.sphere;
    let program = &mut self.program;
    let shadow_program = &mut self.shadow_program;
    let ctxt = &mut self.ctxt;

    let projection = Mat4::perspective_fov_rh_no(
      parameters.fov / 180. * std::f32::consts::PI,
      400.,
      400.,
      0.1,
      10.,
    );

    let light_view: Mat4<f32> = Mat4::look_at_rh(light_position, Vek3::zero(), Vek3::unit_y());
    let rotation = vek::mat4::Mat4::identity()
      .rotated_y(elapsed)
      .rotated_x(elapsed / 2.);

    let shadow_map = &mut self.shadow_fb;

    let shadow_render = ctxt
      .new_pipeline_gate()
      .pipeline(
        shadow_map,
        &PipelineState::default(),
        |_, mut shd_gate| {
          shd_gate.shade(shadow_program, |mut iface, uni, mut rdr_gate| {
            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
              iface.set(&uni.rotation, rotation.into_col_arrays().into());
              iface.set(&uni.projection, projection.into_col_arrays().into());
              iface.set(&uni.light_view, light_view.into_col_arrays().into());

              sphere
                .iter()
                .map(|f| tess_gate.render(f))
                .collect::<Result<(), _>>()
            })
          })
        }
      ).assume();

    if !shadow_render.is_ok() {
      log!("error shadow rendering {:?}", shadow_render.into_result());
    }

    let view: Mat4<f32> = Mat4::look_at_rh(Vek3::new(0., 0., 2.), Vek3::zero(), Vek3::unit_y());
    let normal_matrix = rotation.clone().inverted().transposed();

    let back_buffer = &self.output_fb;

    let render = ctxt
      .new_pipeline_gate()
      .pipeline(
        &back_buffer,
        &PipelineState::default(), //.set_clear_color(color),
        |pipeline, mut shd_gate| {
          let sh_m = pipeline
            .bind_texture(shadow_map.depth_stencil_slot())
            .expect("failed to bind depth texture");

          shd_gate.shade(program, |mut iface, uni, mut rdr_gate| {
            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
              iface.set(&uni.rotation, rotation.into_col_arrays().into());
              iface.set(&uni.normal_matrix, normal_matrix.into_col_arrays().into());
              iface.set(&uni.color, color.into());
              iface.set(&uni.light_position, light_position.into_array().into());

              iface.set(&uni.view, view.into_col_arrays().into());
              iface.set(&uni.projection, projection.into_col_arrays().into());

              iface.set(&uni.light_view, light_view.into_col_arrays().into());
              iface.set(&uni.shadow_map, sh_m.binding());

              sphere
                .iter()
                .map(|f| tess_gate.render(f))
                .collect::<Result<(), _>>()
            })
          })
        },
        // |_, _| Ok(()),
      )
      .assume();

    if !render.is_ok() {
      log!("error rendering {:?}", render.into_result());
    }
  }
}
