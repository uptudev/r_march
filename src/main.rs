use beryllium::{
    init::InitFlags,
    video::{CreateWinArgs, GlProfile},
    *, events::Event,
};
use fermium::video::SDL_GL_GetProcAddress;

use gl33::global_loader as _gl;

mod libs;

fn main() {
    /* Start SDL and run setup */
    let sdl = Sdl::init(InitFlags::EVERYTHING);
    /* Set context attributes */
    sdl.set_gl_context_major_version(3)
        .expect("could not set OpenGl major version");
    sdl.set_gl_context_minor_version(3)
        .expect("could not set OpenGl minor version");
    sdl.set_gl_profile(GlProfile::Core)
        .expect("could not set OpenGl profile");
    #[cfg(target_os = "macos")]
    {
        sdl.set_gl_context_flags(ContextFlags::ForwardCompatibility);
    }
    /* Set window attributes */
    let _win_args = CreateWinArgs {
        title: "",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: true,
    };

    /* Window and OpenGL boilerplate */
    let _win = sdl
        .create_gl_window(_win_args)
        .expect("couldn't make a window and context");

    unsafe {
        _gl::load_global_gl(
            &|p| 
            SDL_GL_GetProcAddress(p.cast::<i8>()) as _);
        _gl::glClearColor(0.2, 0.3, 0.3, 1.0);
    }
    /* Main event polling loop */
    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events().and_then(parse_event) {
            match event {
                Event::Quit => break 'main_loop,
                _ => (),
            }
        }
    // now the events are clear
    
    // here's where we could change the world state and draw.
    }
}

fn parse_event(e:(Event, u32)) -> Option<Event>{
   Option::from(e.0) 
}