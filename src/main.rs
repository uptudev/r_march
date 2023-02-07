use std::mem::size_of;

use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlProfile, GlSwapInterval},
    *,
};
use gl33::global_loader::{self as glob};
use gl33::*;

#[allow(dead_code)]
mod libs;
use libs::gl::{*, Buffer};

fn main() {
    /* Start SDL and run setup */
    let _sdl = Sdl::init(InitFlags::EVERYTHING);
    /* Set context attributes */
    _sdl.set_gl_context_major_version(3)
        .expect("could not set OpenGl major version");
    _sdl.set_gl_context_minor_version(3)
        .expect("could not set OpenGl minor version");
    _sdl.set_gl_profile(GlProfile::Core)
        .expect("could not set OpenGl profile");
    #[cfg(target_os = "macos")]
    {
        _sdl.set_gl_context_flags(ContextFlags::ForwardCompatibility);
    }
    /* Set window attributes */
    let _win_args = CreateWinArgs {
        title: "",
        width: 800,
        height: 600,
        allow_high_dpi: false,
        borderless: false,
        resizable: true,
    };

    /* Window and OpenGL boilerplate */
    let _win = _sdl
        .create_gl_window(_win_args)
        .expect("couldn't make a window and context");
    let _gl: gl33::GlFns;
    unsafe { glob::load_global_gl(&|p| _win.get_proc_address(p)) }

    /* Create Vertex type and _TRIVERTS constant to draw a triangle. */
    type Vertex = [f32; 3];
    const _TRIVERTS: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

    const _VERTEX_GLSL: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
        void main() {
            gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
        }
        "#;
    const _FRAGMENT_GLSL: &str = r#"#version 330 core
        out vec4 final_color;
        void main() {
            final_color = vec4(1.0, 0.5, 0.2, 1.0);
        }
        "#;

    unsafe {
        /* Load OpenGL functions from _win to _gl */
        _gl = gl33::GlFns::load_from(&|p| _win.get_proc_address(p))
            .expect("Failed to load OpenGL functions");
    }

    /* Generate and bind a Vertex Array Object */
    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();
    /* Generate and bind a Vertex Buffer Object */
    let vbo = Buffer::new().expect("Couldn't make a VBO");
    vbo.bind(BufferType::Array);

    clear_color(0.2, 0.3, 0.3, 1.0);

    /* Buffer the triangle vertices into the VAO */
    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&_TRIVERTS),
        GL_STATIC_DRAW,
    );

    unsafe {
        /* Tell OpenGL how to parse the buffered data */
        _gl.VertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0u8,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );

        /* Create vertex GLSL shader */
        let vertex_shader = _gl.CreateShader(GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        _gl.ShaderSource(
            vertex_shader,
            1,
            &(_VERTEX_GLSL.as_bytes().as_ptr().cast()),
            &(_VERTEX_GLSL.len().try_into().unwrap()),
        );
        _gl.CompileShader(vertex_shader);

        /* Error checking vertex shader */
        let mut success = 0;
        _gl.GetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            _gl.GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Compile Error: {}", String::from_utf8_lossy(&v));
        }

        /* Create fragment GLSL shader */
        let fragment_shader = _gl.CreateShader(GL_FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        _gl.ShaderSource(
            fragment_shader,
            1,
            &(_FRAGMENT_GLSL.as_bytes().as_ptr().cast()),
            &(_FRAGMENT_GLSL.len().try_into().unwrap()),
        );
        _gl.CompileShader(fragment_shader);

        /* Error checking fragment shader */
        let mut success = 0;
        _gl.GetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            _gl.GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Compile Error: {}", String::from_utf8_lossy(&v));
        }

        /* Create and attach shaders to shader program */
        let shader_program = _gl.CreateProgram();
        _gl.AttachShader(shader_program, vertex_shader);
        _gl.AttachShader(shader_program, fragment_shader);
        _gl.LinkProgram(shader_program);

        /* Error check shader program */
        let mut success = 0;
        _gl.GetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            _gl.GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        _gl.DeleteShader(vertex_shader);
        _gl.DeleteShader(fragment_shader);
        _gl.UseProgram(shader_program);
        _win.set_swap_interval(GlSwapInterval::Vsync)
            .expect("Error enabling VSync.");
    }
    /* Main running loop */
    'main_loop: loop {
        // handle events this frame
        while let Some(event) = _sdl.poll_events().and_then(parse_event) {
            match event {
                Event::Quit => break 'main_loop,
                _ => (),
            }
        }

        // Draw
        unsafe {
            _gl.Clear(GL_COLOR_BUFFER_BIT);
            _gl.DrawArrays(GL_TRIANGLES, 0, 3);
            _win.swap_window();
        }
    }
}

fn parse_event(e: (Event, u32)) -> Option<Event> {
    Option::from(e.0)
}