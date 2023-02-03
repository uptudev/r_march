use beryllium::{
    init::InitFlags,
    video::{CreateWinArgs, GlProfile},
    *,
};

mod libs;

fn main() {
    /* Start SDL and run setup */
    let sdl = Sdl::init(InitFlags::EVERYTHING);
    /* Set context attributes */
    sdl.set_gl_context_major_version(3);
    sdl.set_gl_context_minor_version(3);
    sdl.set_gl_profile(GlProfile::Core);
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

    /* Make window */
    let _win = sdl
        .create_gl_window(_win_args)
        .expect("couldn't make a window and context");
}
