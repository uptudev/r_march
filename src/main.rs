use std::mem::size_of;

use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlProfile, GlSwapInterval},
    *,
};
use gl33::global_loader::{self as glob};
use gl33::{GL_COLOR_BUFFER_BIT, GL_FLOAT, GL_STATIC_DRAW, GL_TRIANGLES};
#[allow(dead_code)]
mod libs;
use libs::gl::{Buffer, ShaderProgram, *};

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

    #[allow(dead_code)]
    const OLD_VERTEX_GLSL: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
        void main() {
            gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
        }
        "#;
    
    #[allow(dead_code)]
    const OLD_FRAGMENT_GLSL: &str = r#"#version 330 core
        out vec4 final_color;
        void main() {
            final_color = vec4(1.0, 0.5, 0.2, 1.0);
        }
        "#;

    let _vertex_glsl_src: String =
        std::fs::read_to_string("./src/shaders/vert.glsl")
            .expect("Couldn't read vertex shader GLSL files.");
    
    let _fragment_glsl_src: String =
        std::fs::read_to_string("./src/shaders/frag.glsl")
            .expect("Couldn't read fragment shader GLSL files.");

    let _vertex_glsl: &str =
        &_vertex_glsl_src.as_str();

    let _fragment_glsl: &str =
        &_fragment_glsl_src.as_str();

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

    /* Tell OpenGL how to parse the buffered data */
    unsafe {
        _gl.VertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            0u8,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
    }

    /* Create shader program and attach shaders from source code strings */
    let shader_program = ShaderProgram::from_vert_frag(OLD_VERTEX_GLSL, OLD_FRAGMENT_GLSL)
        .expect("Shader program failed to be made");
    shader_program.use_program();

    _win.set_swap_interval(GlSwapInterval::Vsync)
        .expect("Error enabling VSync.");

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
        }
        _win.swap_window();
    }
}

fn parse_event(e: (Event, u32)) -> Option<Event> {
    Option::from(e.0)
}
