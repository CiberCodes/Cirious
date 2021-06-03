use sdl2::Sdl;
use sdl2::render::WindowCanvas;

pub struct Window {
    sdl: Sdl,
    window: sdl2::video::Window,
    canvas: WindowCanvas,
}

impl Window {
    pub fn default() -> Result<Window, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;
        let display = video_subsystem.current_display_mode(0)?;

        let (display_width, display_height) = (display.w as u32, display.w as u32);

        let window = sdl2::video::WindowBuilder::new(
            &video_subsystem,
            "Cirious",
            display_width,
            display_height
        ).opengl().position_centered().build().map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(
            Self {sdl, window, canvas}
        )
    }
}