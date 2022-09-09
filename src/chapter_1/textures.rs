use std::path::Path;

use super::Application;
use crate::shader::Shader;

use image::io::Reader as ImageReader;

pub fn run() -> Result<(), String> {
    // Clears terminal
    print!("{}[2J", 27 as char);

    println!(
        "Choose a sub program to run:

1. Load Texture

Type in the number."
    );
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    // Removes endline
    input.pop();

    match input.as_str() {
        "1" => run_load_texture(Application::setup()?)?,
        _ => println!("Invalid input {}.", input),
    }

    Ok(())
}

fn run_load_texture(app: Application) -> Result<(), String> {
    let (shader, _vbo, vao, _ebo, texture) = unsafe {
        use gl::types::{GLfloat, GLsizei, GLsizeiptr};
        use std::mem::size_of;
        use std::os::raw::c_void;

        let shader = Shader::new(
            "shaders/chapter_1/4_1.vs".into(),
            "shaders/chapter_1/4_1.fs".into(),
        )?;

        // -------------------- Setup Vertex Data -------------------------

        let vertices: [f32; 32] = [
            0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
            0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
            -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
        ];
        let indices = [
            0, 1, 3, // first Triangle
            1, 2, 3, // second Triangle
        ];
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<GLfloat>()) as GLsizeiptr,
            indices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // -------------------- Config Vertex Attributes -------------------------

        let stride = 8 * size_of::<GLfloat>() as GLsizei;

        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // color attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // texture coord attribute
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (6 * size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);

        // ------------------------- Load Texture -------------------------

        let mut texture: u32 = 0;
        // Tells OpenGL to create a texture and the bind it to the TEXTURE_2D buffer
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Set Texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Load image from file
        let img = ImageReader::open(Path::new("./textures/chapter_1/container.jpg"))
            .map_err(|e| e.to_string())?
            .decode()
            .map_err(|e| e.to_string())?;
        let (width, height) = (img.width() as i32, img.height() as i32);
        let img_data = img.as_bytes();

        // Create the texture from the image data and generate the mipmaps
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width,
            height,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            img_data.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        drop(img_data);

        (shader, vbo, vao, ebo, texture)
    };

    // -------------------- Run Event Loop -------------------------

    app.event_loop.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

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

                // Bind the texture
                gl::BindTexture(gl::TEXTURE_2D, texture);

                // Draw the triangle
                shader.use_program();
                gl::BindVertexArray(vao);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                //gl::BindVertexArray(0); // Not necessary for this simple program

                app.context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}
