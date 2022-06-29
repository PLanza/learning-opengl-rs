pub mod hello_triangle;
pub mod hello_window;

pub use glutin;

struct Application {
    event_loop: glutin::event_loop::EventLoop<()>,
    context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
}
