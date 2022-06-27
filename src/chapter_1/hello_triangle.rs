// The GLSL shader code
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
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

pub fn run() -> Result<(), String> {
    // -------------------- Initialize Context --------------------

    let el = glutin::event_loop::EventLoop::new();

    let cb = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3))) // OpenGL version 3.3
        .with_gl_profile(glutin::GlProfile::Core); // OpenGL Core profile

    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0)) // LogicalSize respects dpi
        .with_title("Learn OpenGL in Rust");

    let wc = cb.build_windowed(wb, &el).map_err(|e| e.to_string())?;

    // Built window context is not current so we make it current
    let current_context;
    unsafe {
        current_context = match wc.make_current().ok() {
            Some(context) => context,
            None => return Err("Could not make context current.".to_string()),
        };
    }

    // -------------------- Load function pointers --------------------

    gl::load_with(|s| current_context.get_proc_address(s));

    // -------------------- Setup Vertex Shader -------------------------

    let (shader_program, vao) = unsafe {
        // Create the vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        // Convert the Rust string to a C string
        let vtx_src_c_string =
            std::ffi::CString::new(VERTEX_SHADER_SOURCE.as_bytes()).map_err(|e| e.to_string())?;
        // Attach the shader source code to the shader object
        gl::ShaderSource(
            vertex_shader,
            1,
            &vtx_src_c_string.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        // Check for shader compile errors
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
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }

        // -------------------- Setup Fragment Shader -------------------------

        // Create the fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        // Convert the Rust string to a C string
        let frag_src_c_string =
            std::ffi::CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).map_err(|e| e.to_string())?;
        // Attach the shader source code to the shader object
        gl::ShaderSource(
            fragment_shader,
            1,
            &frag_src_c_string.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        // Check for shader compile errors
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            println!(
                "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }

        // -------------------- Create Shader Program -------------------------

        let shader_program = gl::CreateProgram();
        // Attach the shaders to the program
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check for program link errors
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            println!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }

        // Delete Shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // -------------------- Setup Vertex Data -------------------------

        // The normalized coordinates for the triangle
        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0, // top
        ];

        // Create a vertex buffer and vertex array object
        let (mut vbo, mut vao) = (0, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        // Bind the VAO first
        gl::BindVertexArray(vao);

        // Bind the buffer object to an array buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // Pass the triangle's vertices to the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            &vertices[0] as *const f32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        // -------------------- Config Vertex Attributes -------------------------

        // Tells OpengGL how it should interpret vertex data
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // Unbind vbo since it's been registered in the call to VertexAttribPointer()
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // Draw polygons in wireframe, not filled in
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader_program, vao)
    };

    // -------------------- Run Event Loop -------------------------

    el.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        use glutin::event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent};
        match event {
            Event::LoopDestroyed => return (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(phys_size) => current_context.resize(phys_size),
                WindowEvent::CloseRequested => {
                    // Sends a LoopDestroyed event and stops the event loop
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
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
                // gl::BindVertexArray(0); // Not necessary for this simple program

                current_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}
