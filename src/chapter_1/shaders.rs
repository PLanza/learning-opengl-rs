use super::Application;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    uniform vec4 ourColor;
    void main() {
       FragColor = ourColor;
    }
"#;

pub fn run() -> Result<(), String> {
    // Clears terminal
    print!("{}[2J", 27 as char);

    println!(
        "Choose a sub program to run:

1. Shaders Uniform 

Type in the number."
    );
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    // Removes endline
    input.pop();

    match input.as_str() {
        "1" => run_shaders_uniform(setup()?)?,
        _ => println!("Invalid input {}.", input),
    }

    Ok(())
}


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

fn run_shaders_uniform(app: Application) -> Result<(), String> {
    
    let (shader_program, _vao) = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let vtx_src_c_string =
            std::ffi::CString::new(VERTEX_SHADER_SOURCE.as_bytes()).map_err(|e| e.to_string())?;
        gl::ShaderSource(
            vertex_shader,
            1,
            &vtx_src_c_string.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        let mut success = gl::FALSE as gl::types::GLint;
        let mut info_log = Vec::with_capacity(512);

        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            let error = format!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
            return Err(error);
        }

        // -------------------- Setup Fragment Shader -------------------------

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let frag_src_c_string =
            std::ffi::CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).map_err(|e| e.to_string())?;
        gl::ShaderSource(
            fragment_shader,
            1,
            &frag_src_c_string.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            let error = format!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
            return Err(error);
        }

        // -------------------- Create Shader Program -------------------------

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            let error = format!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
            return Err(error);
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // -------------------- Setup Vertex Data -------------------------

        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];

        let (mut vbo, mut vao) = (0, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            &vertices[0] as *const f32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        // -------------------- Config Vertex Attributes -------------------------

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // Draw polygons in wireframe, not filled in
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader_program, vao)
    };

    // -------------------- Run Event Loop -------------------------

    let start_time = std::time::Instant::now();
    app.event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        // Change the green value every iteration of the loop
        let green_value = (start_time.elapsed().as_secs_f32().sin() / 2.0) + 0.5;
        // Need to convert "ourColor"into a CString
        let uniform_name = 
            std::ffi::CString::new("ourColor").unwrap();
        
        unsafe {
            // Get the uniform vertex location
            let vertex_color_location = gl::GetUniformLocation(shader_program, uniform_name.as_ptr());
            // Make sure the shader program is loaded
            gl::UseProgram(shader_program);
            // Assign to the uniform vertex
            gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);
        }
        // Ask the app to redraw the window
        app.context.window().request_redraw();

        use glutin::event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent};
        match event {
            Event::LoopDestroyed => return (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(phys_size) => app.context.resize(phys_size),
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit
                }
                _ => (),
            },

            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Key(key_input) => match key_input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit
                    }
                    Some(_) => (),
                    None => (),
                },
                _ => (),
            },

            Event::RedrawRequested(_) => unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                // Draw the triangle
                gl::UseProgram(shader_program);
                //gl::BindVertexArray(vao); // Not necessary for this simple program
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
                // gl::BindVertexArray(0); // Not necessary for this simple program

                app.context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

