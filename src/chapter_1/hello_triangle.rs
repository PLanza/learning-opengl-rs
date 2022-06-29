use gl::types;

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

const FRAGMENT_SHADER_SOURCE_2: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 1.0f, 0.0f, 1.0f);
    }
"#;

pub fn run() -> Result<(), String> {
    // Clears terminal
    print!("{}[2J", 27 as char);

    println!(
        "Choose a sub program to run:

1. Basic
2. Indexed
3. Exercise 1
4. Exercise 2
5. Exercise 3

Type in the number."
    );
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    // Removes endline
    input.pop();

    match input.as_str() {
        "1" => run_basic()?,
        "2" => run_indexed()?,
        "3" => run_exercise_1()?,
        "4" => run_exercise_2()?,
        "5" => run_exercise_3()?,
        _ => println!("Invalid input {}.", input),
    }

    Ok(())
}

pub fn run_basic() -> Result<(), String> {
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

    let (shader_program, _vao) = unsafe {
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
                //gl::BindVertexArray(vao); // Not necessary for this simple program
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
                // gl::BindVertexArray(0); // Not necessary for this simple program

                current_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

pub fn run_indexed() -> Result<(), String> {
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
        let mut success = gl::FALSE as types::GLint;
        let mut info_log = Vec::with_capacity(512);

        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as types::GLint {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut types::GLchar,
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
        if success != gl::TRUE as types::GLint {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut types::GLchar,
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
        if success != gl::TRUE as types::GLint {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut types::GLchar,
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
        let vertices: [f32; 12] = [
            0.5, 0.5, 0.0, // top right
            0.5, -0.5, 0.0, // bottom right
            -0.5, -0.5, 0.0, // bottom left
            -0.5, 0.5, 0.0, // top left
        ];
        let indices = [0, 1, 3, 1, 2, 3];

        // Create a vertex buffer, vertex array object and element buffer
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        // Bind the VAO first
        gl::BindVertexArray(vao);

        // Bind the buffer object to an array buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // Pass the triangle's vertices to the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * 4) as types::GLsizeiptr,
            &vertices[0] as *const f32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * 4) as types::GLsizeiptr,
            &indices[0] as *const i32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        // -------------------- Config Vertex Attributes -------------------------

        // Tells OpengGL how it should interpret vertex data
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * 4 as types::GLsizei,
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
                // Note that the ebo is stored IN the vao so we bind the vao
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                // gl::BindVertexArray(0); // Not necessary for this simple program

                current_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

pub fn run_exercise_1() -> Result<(), String> {
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
        let mut success = gl::FALSE as types::GLint;
        let mut info_log = Vec::with_capacity(512);

        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as types::GLint {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut types::GLchar,
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
        if success != gl::TRUE as types::GLint {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut types::GLchar,
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
        if success != gl::TRUE as types::GLint {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut types::GLchar,
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
        let vertices: [f32; 18] = [
            -0.75, -0.25, 0.0, // left left
            -0.5, 0.25, 0.0, // left top
            -0.25, -0.25, 0.0, // left right
            0.25, -0.25, 0.0, // right left
            0.5, 0.25, 0.0, // right top
            0.75, -0.25, 0.0, // right right
        ];
        let indices = [0, 1, 2, 3, 4, 5];

        // Create a vertex buffer, vertex array object and element buffer
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        // Bind the VAO first
        gl::BindVertexArray(vao);

        // Bind the buffer object to an array buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // Pass the triangle's vertices to the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * 4) as types::GLsizeiptr,
            &vertices[0] as *const f32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * 4) as types::GLsizeiptr,
            &indices[0] as *const i32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        // -------------------- Config Vertex Attributes -------------------------

        // Tells OpengGL how it should interpret vertex data
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * 4 as types::GLsizei,
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
                // Note that the ebo is stored IN the vao so we bind the vao
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                // gl::BindVertexArray(0); // Not necessary for this simple program

                current_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

pub fn run_exercise_2() -> Result<(), String> {
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

    let (shader_program, VAOs) = unsafe {
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
        let mut success = gl::FALSE as types::GLint;
        let mut info_log = Vec::with_capacity(512);

        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as types::GLint {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut types::GLchar,
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
        if success != gl::TRUE as types::GLint {
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut types::GLchar,
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
        if success != gl::TRUE as types::GLint {
            gl::GetProgramInfoLog(
                shader_program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut types::GLchar,
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
        let vertices: [f32; 18] = [
            -0.75, -0.25, 0.0, // left left
            -0.5, 0.25, 0.0, // left top
            -0.25, -0.25, 0.0, // left right
            0.25, -0.25, 0.0, // right left
            0.5, 0.25, 0.0, // right top
            0.75, -0.25, 0.0, // right right
        ];
        let left_indices = [0, 1, 2];
        let right_indices = [3, 4, 5];

        // Create a vertex buffer, vertex array object and element buffer
        let (mut VBOs, mut VAOs, mut EBOs) = ([0, 0], [0, 0], [0, 0]);

        // Generate multiple objects at once
        gl::GenVertexArrays(2, VAOs.as_mut_ptr());
        gl::GenBuffers(2, VBOs.as_mut_ptr());
        gl::GenBuffers(2, EBOs.as_mut_ptr());

        // -------------------- Left Triangle -------------------------

        gl::BindVertexArray(VAOs[0]);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBOs[0]);
        // Pass the triangle's vertices to the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * 4) as types::GLsizeiptr,
            &vertices[0] as *const f32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBOs[0]);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (left_indices.len() * 4) as types::GLsizeiptr,
            &left_indices[0] as *const i32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        // Tells OpengGL how it should interpret vertex data
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * 4 as types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // -------------------- Right Triangle -------------------------

        gl::BindVertexArray(VAOs[1]);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBOs[1]);
        // Pass the triangle's vertices to the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * 4) as types::GLsizeiptr,
            &vertices[0] as *const f32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBOs[1]);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (right_indices.len() * 4) as types::GLsizeiptr,
            &right_indices[0] as *const i32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        // Tells OpengGL how it should interpret vertex data
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * 4 as types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // Unbind vbo since it's been registered in the call to VertexAttribPointer()
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // Draw polygons in wireframe, not filled in
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader_program, VAOs)
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

                // Draw the left triangle
                gl::BindVertexArray(VAOs[0]);
                gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());

                // Draw the right triangle
                gl::BindVertexArray(VAOs[1]);
                gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
                gl::BindVertexArray(0); // Not necessary for this simple program

                current_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

pub fn run_exercise_3() -> Result<(), String> {
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

    let (orange_program, yellow_program, VAOs) = unsafe {
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
        let mut success = gl::FALSE as types::GLint;
        let mut info_log = Vec::with_capacity(512);

        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as types::GLint {
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut types::GLchar,
            );
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }

        // -------------------- Setup Fragment Shaders -------------------------

        // Create the fragment shaders
        let orange_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let yellow_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

        // Convert the Rust string to a C string
        let orng_src_c_string =
            std::ffi::CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).map_err(|e| e.to_string())?;

        // Convert the Rust string to a C string
        let yllw_src_c_string = std::ffi::CString::new(FRAGMENT_SHADER_SOURCE_2.as_bytes())
            .map_err(|e| e.to_string())?;

        // Attach the shader source code to the shader object
        gl::ShaderSource(
            orange_shader,
            1,
            &orng_src_c_string.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(orange_shader);

        // Attach the shader source code to the shader object
        gl::ShaderSource(
            yellow_shader,
            1,
            &yllw_src_c_string.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(yellow_shader);

        // Check for shader compile errors
        // ...

        // -------------------- Create Shader Program -------------------------

        let orange_program = gl::CreateProgram();
        let yellow_program = gl::CreateProgram();

        // Attach the shaders to the program
        gl::AttachShader(orange_program, vertex_shader);
        gl::AttachShader(orange_program, orange_shader);
        gl::LinkProgram(orange_program);

        gl::AttachShader(yellow_program, vertex_shader);
        gl::AttachShader(yellow_program, yellow_shader);
        gl::LinkProgram(yellow_program);

        // Check for program link errors
        // ...

        // Delete Shaders
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(orange_shader);
        gl::DeleteShader(yellow_shader);

        // -------------------- Setup Vertex Data -------------------------

        // The normalized coordinates for the triangle
        let vertices: [f32; 18] = [
            -0.75, -0.25, 0.0, // left left
            -0.5, 0.25, 0.0, // left top
            -0.25, -0.25, 0.0, // left right
            0.25, -0.25, 0.0, // right left
            0.5, 0.25, 0.0, // right top
            0.75, -0.25, 0.0, // right right
        ];
        let left_indices = [0, 1, 2];
        let right_indices = [3, 4, 5];

        // Create a vertex buffer, vertex array object and element buffer
        let (mut VBOs, mut VAOs, mut EBOs) = ([0, 0], [0, 0], [0, 0]);

        // Generate multiple objects at once
        gl::GenVertexArrays(2, VAOs.as_mut_ptr());
        gl::GenBuffers(2, VBOs.as_mut_ptr());
        gl::GenBuffers(2, EBOs.as_mut_ptr());

        // -------------------- Left Triangle -------------------------

        gl::BindVertexArray(VAOs[0]);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBOs[0]);
        // Pass the triangle's vertices to the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * 4) as types::GLsizeiptr,
            &vertices[0] as *const f32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBOs[0]);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (left_indices.len() * 4) as types::GLsizeiptr,
            &left_indices[0] as *const i32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        // Tells OpengGL how it should interpret vertex data
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * 4 as types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // -------------------- Right Triangle -------------------------

        gl::BindVertexArray(VAOs[1]);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBOs[1]);
        // Pass the triangle's vertices to the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * 4) as types::GLsizeiptr,
            &vertices[0] as *const f32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBOs[1]);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (right_indices.len() * 4) as types::GLsizeiptr,
            &right_indices[0] as *const i32 as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );

        // Tells OpengGL how it should interpret vertex data
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * 4 as types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // Unbind vbo since it's been registered in the call to VertexAttribPointer()
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        // Draw polygons in wireframe, not filled in
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (orange_program, yellow_program, VAOs)
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

                // Draw the left triangle
                gl::UseProgram(orange_program);
                gl::BindVertexArray(VAOs[0]);
                gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());

                // Draw the right triangle
                gl::UseProgram(yellow_program);
                gl::BindVertexArray(VAOs[1]);
                gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
                gl::BindVertexArray(0); // Not necessary for this simple program

                current_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}
