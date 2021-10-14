use luminance_front::tess::TessError;
use luminance_front::tess::Interleaved;
use luminance_front::tess::Tess;
use luminance::context::GraphicsContext;
use std::collections::HashMap;
use glfw::{Action, Context as _, Key, WindowEvent};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::process::exit;
use luminance::context::GraphicsContext as _;
use luminance::pipeline::PipelineState;
use std::time::Instant;

use luminance::tess::Mode;

use luminance::shader::Program;
use luminance::render_state::RenderState;

const VS_STR: &str = include_str!("vs.glsl");
const FS_STR: &str = include_str!("fs.glsl");

use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum VertexSemantics {
  #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
  Position,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "VertexSemantics")]
struct ObjVertex {
  position: VertexPosition,
}

type VertexIndex = u32;

use std::fs::File;
use std::io::Read as _;
use std::path::Path;
use try_guard::verify;
use wavefront_obj::obj;

use luminance_front::Backend;

#[derive(Debug)]
struct Obj {
  vertices: Vec<ObjVertex>,
  indices: Vec<VertexIndex>,
}

impl Obj {
  fn to_tess(self, ctxt: &mut impl GraphicsContext<Backend=Backend>) -> Result<Tess<ObjVertex, VertexIndex, (), Interleaved>, TessError> {
    ctxt
      .new_tess()
      .set_mode(Mode::Triangle)
      .set_vertices(self.vertices)
      .set_indices(self.indices)
      .build()
  }

  fn load<P>(path: P) -> Result<Self, String>
  where
    P: AsRef<Path>,
  {
    let file_content = {
      let mut file = File::open(path).map_err(|e| format!("cannot open file: {}", e))?;
      let mut content = String::new();
      file.read_to_string(&mut content).unwrap();
      content
    };
    let obj_set = obj::parse(file_content).map_err(|e| format!("cannot parse: {:?}", e))?;
    let objects = obj_set.objects;

    verify!(objects.len() == 1).ok_or("expecting a single object".to_owned())?;

    let object = objects.into_iter().next().unwrap();

    verify!(object.geometry.len() == 1).ok_or("expecting a single geometry".to_owned())?;

    let geometry = object.geometry.into_iter().next().unwrap();

    println!("loading {}", object.name);
    println!("{} vertices", object.vertices.len());
    println!("{} shapes", geometry.shapes.len());

    // build up vertices; for this to work, we remove duplicated vertices by putting them in a
    // map associating the vertex with its ID
    let mut vertex_cache: HashMap<obj::VTNIndex, VertexIndex> = HashMap::new();
    let mut vertices: Vec<ObjVertex> = Vec::new();
    let mut indices: Vec<VertexIndex> = Vec::new();

    for shape in geometry.shapes {
      if let obj::Primitive::Triangle(a, b, c) = shape.primitive {
        for key in &[a, b, c] {
          if let Some(vertex_index) = vertex_cache.get(key) {
            indices.push(*vertex_index);
          } else {
            let p = object.vertices[key.0];
            let position = VertexPosition::new([p.x as f32, p.y as f32, p.z as f32]);
            let vertex = ObjVertex { position };
            let vertex_index = vertices.len() as VertexIndex;

            vertex_cache.insert(*key, vertex_index);
            vertices.push(vertex);
            indices.push(vertex_index);
          }
        }
      } else {
        return Err("unsupported non-triangle shape".to_owned());
      }
    }

    Ok(Obj { vertices, indices })
  }
}

fn main() {
  // our graphics surface
  let dim = WindowDim::Windowed {
    width: 960,
    height: 540,
  };
  let surface = GlfwSurface::new_gl33("Hello, world!", WindowOpt::default().set_dim(dim));

  match surface {
    Ok(surface) => {
      eprintln!("graphics surface created");
      main_loop(surface);
    }
    Err(e) => {
      eprintln!("cannot create graphics surface:\n{}", e);
      exit(1);
    }
  }
}

fn main_loop(mut surface: GlfwSurface) {
  let start_t = Instant::now();
  let mut ctxt = surface.context;
  let events = surface.events_rx;
  let back_buffer = ctxt.back_buffer().expect("back buffer");

  let shape = Obj::load("Box.obj").unwrap().to_tess(&mut ctxt).unwrap();
  
  let mut program = ctxt
    .new_shader_program::<VertexSemantics, (), ()>()
    .from_strings(VS_STR, None, None, FS_STR)
    .unwrap()
    .ignore_warnings();

  'app: loop {
    // handle events
    ctxt.window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
        _ => (),
      }
    }
    // rendering code goes here
    let t = start_t.elapsed().as_secs_f32();
    let color = [t.cos(), t.sin(), 0.5, 1.];

    let render = ctxt
      .new_pipeline_gate()
      .pipeline(
        &back_buffer,
        &PipelineState::default().set_clear_color(color),
        |_, mut shd_gate| {
          shd_gate.shade(&mut program, |_, _, mut rdr_gate| {
            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
              tess_gate.render(&shape)
            })
          })
        },
      )
      .assume();

    // swap buffer chains
    if render.is_ok() {
      ctxt.window.swap_buffers();
    } else {
      break 'app;
    }
  }
}
