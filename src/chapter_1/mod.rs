pub mod hello_triangle;
pub mod hello_window;
pub mod shaders;
pub mod textures;

pub use glutin;

struct Application {
    event_loop: glutin::event_loop::EventLoop<()>,
    context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
}

impl Application {
    fn setup() -> Result<Application, String> {
        // -------------------- Initialize Context --------------------

        let el = glutin::event_loop::EventLoop::new();

        let cb = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3))) // OpenGL version 3.3
            .with_gl_profile(glutin::GlProfile::Core); // OpenGL Core profile

        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0)) // LogicalSize respects dpi
            .with_title("Learn OpenGL in Rust");

        let wc = cb.build_windowed(wb, &el).map_err(|e| e.to_string())?;

        let current_context;
        unsafe {
            current_context = match wc.make_current().ok() {
                Some(context) => context,
                None => return Err("Could not make context current.".to_string()),
            };
        }

        // -------------------- Load function pointers --------------------

        gl::load_with(|s| current_context.get_proc_address(s));

        Ok(Application {
            event_loop: el,
            context: current_context,
        })
    }
}
