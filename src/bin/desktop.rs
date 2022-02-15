#[cfg(not(target_arch = "wasm32"))]
mod desktop {
  use std::time::Instant;

  use glfw::{Action, Context as _, Key, SwapInterval, WindowEvent, WindowMode};
  use luminance_front::context::GraphicsContext;
  use luminance_glfw::GL33Context;
  use luminance_glfw::{GlfwSurface, GlfwSurfaceError};

  use florest::geometry::Planet;
  use florest::parameters::{self, RenderParameters};
  use florest::shaders::Render;

  use obj::{Geometry, ObjSet, Object, Primitive, Shape, TVertex, Vertex};
  use obj_exporter as obj;

  fn save_to_obj(planet: Planet, path: &str) {
    let vertices = planet
      .vertices
      .iter()
      .map(|v| v.position.repr)
      .collect::<Vec<[f32; 3]>>();
    let indices = planet.indices.iter().map(|i| *i).collect::<Vec<u32>>();
    let normals = planet
      .vertices
      .iter()
      .map(|v| v.norm.repr)
      .collect::<Vec<[f32; 3]>>();

    let set = ObjSet {
      material_library: None,
      objects: vec![Object {
        name: "Planet".to_owned(),
        vertices: vertices
          .into_iter()
          .map(|v| Vertex {
            x: v[0] as f64,
            y: v[1] as f64,
            z: v[2] as f64,
          })
          .collect(),
        tex_vertices: vec![],
        normals: normals
          .into_iter()
          .map(|v| Vertex {
            x: v[0] as f64,
            y: v[1] as f64,
            z: v[2] as f64,
          })
          .collect(),
        geometry: vec![Geometry {
          material_name: None,
          shapes: indices.chunks(3)
            .into_iter()
            .map(|i| Shape {
              primitive: Primitive::Triangle(
                (i[0] as usize, None, Some(i[0] as usize)),
                (i[1] as usize, None, Some(i[1] as usize)),
                (i[2] as usize, None, Some(i[2] as usize)),
              ),
              groups: vec![],
              smoothing_groups: vec![],
            })
            .collect(),
        }],
      }],
    };

    obj::export_to_file(&set, path).unwrap();
  }

  fn main_loop(start_t: Instant, mut surafce: GlfwSurface) {
    let fb = surafce.context.back_buffer().expect("failed to fetch fb");
    let parameters = RenderParameters::new();
    let planet = Planet::new(&parameters);

    save_to_obj(planet, "planet.obj");
    // let mut r: Render<GL33Context> = Render::from(surafce.context, &parameters, fb);
    // loop {
    //   r.ctxt.window.glfw.poll_events();

    //   let elapsed = start_t.elapsed();
    //   let t = elapsed.as_secs() as f64 + (elapsed.subsec_millis() as f64 * 1e-3);

    //   r.frame(t as f32, &parameters);

    //   r.ctxt.window.swap_buffers();
    // }
  }

  pub fn run() -> () {
    let surface = GlfwSurface::new(|glfw| {
      let (mut window, events) = glfw
        .create_window(800, 800, "Hello", WindowMode::Windowed)
        .ok_or_else(|| GlfwSurfaceError::UserError("e"))?;

      window.make_current();
      window.set_all_polling(true);
      glfw.set_swap_interval(SwapInterval::Sync(1));

      Ok((window, events))
    });

    let start_t = Instant::now();

    match surface {
      Ok(surface) => {
        eprintln!("graphics surface created");
        main_loop(start_t, surface);
      }

      Err(e) => {
        eprintln!("cannot create graphics surface:\n{}", e);
        // exit(1);
      }
    }
  }
}

fn main() {
  #[cfg(not(target_arch = "wasm32"))]
  desktop::run();
}
