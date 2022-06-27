pub fn run() -> Result<(), String> {
    // Provides a way to retrieve events from the system and
    // from the windows that were registered to the event loop.
    let event_loop = glutin::event_loop::EventLoop::new();

    // Sets up the window's context
    let context_builder = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3))) // OpenGL version 3.3
        .with_gl_profile(glutin::GlProfile::Core); // OpenGL Core profile

    // Sets up the window parameters
    let window_builder = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0)) // LogicalSize respects dpi
        .with_title("Learn OpenGL in Rust");

    // Builds the window based on the context and parameters
    let window_context = context_builder
        .build_windowed(window_builder, &event_loop)
        .map_err(|e| e.to_string())?;

    // Built window context is not current so we make it current
    let current_context;
    unsafe {
        current_context = match window_context.make_current().ok() {
            Some(context) => context,
            None => return Err("Could not make context current.".to_string()),
        };
    }

    // Loads the OpenGL function pointers
    gl::load_with(|symbol| current_context.get_proc_address(symbol));

    // "move" captures a closure's environment by value
    event_loop.run(move |event, _, control_flow| {
        // When the loop iteration finishes, immediately begin a new iteration
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        use glutin::event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent};
        match event {
            Event::LoopDestroyed => return (),
            Event::WindowEvent { event, .. } => match event {
                // Resizes the window context together with the window
                WindowEvent::Resized(phys_size) => current_context.resize(phys_size),
                // When window X is clicked
                WindowEvent::CloseRequested => {
                    // Sends a LoopDestroyed event and stops the event loop
                    *control_flow = glutin::event_loop::ControlFlow::Exit
                }
                _ => (),
            },

            Event::DeviceEvent { event, .. } => match event {
                // Gets the key's semantic code
                DeviceEvent::Key(key_input) => match key_input.virtual_keycode {
                    // Close on keyboard press Escape
                    Some(VirtualKeyCode::Escape) => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit
                    }
                    Some(_) => (),
                    None => (),
                },
                _ => (),
            },

            // Is triggered when the window's contents have been invalidated (e.g. window resize)
            Event::RedrawRequested(_) => unsafe {
                // Sets color to clear into background
                gl::ClearColor(0.8863, 0.5294, 0.2627, 1.0);
                // Clears all buffers enabled for color writting
                gl::Clear(gl::COLOR_BUFFER_BIT);
                current_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}
