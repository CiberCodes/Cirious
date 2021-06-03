use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::event::EventPollIterator;

pub struct Window {
    pub sdl: Sdl,
    pub canvas: WindowCanvas,
}

pub enum FullscreenType {
    Desktop,
    Off,
    True,
}

impl FullscreenType {
    fn convert(fullscreen_type: FullscreenType) -> sdl2::video::FullscreenType {
        match fullscreen_type {
            FullscreenType::Desktop => sdl2::video::FullscreenType::Desktop,
            FullscreenType::Off => sdl2::video::FullscreenType::Off,
            FullscreenType::True => sdl2::video::FullscreenType::True,
        }
    }
}

impl Window {
    pub fn new(
        &self, title: &str,
        width: u32,
        heigth: u32,
        borderless: bool
    ) -> Result<Window, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        let mut window = sdl2::video::WindowBuilder::new(
            &video_subsystem,
            title,
            width,
            heigth
        ).opengl().position_centered().build().map_err(|e| e.to_string())?;

        window.set_bordered(borderless);

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(
            Self {sdl, canvas}
        )
    }

    pub fn default() -> Result<Window, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;
        let display = video_subsystem.current_display_mode(0)?;

        let (display_width, display_heigth) = (display.w as u32, display.w as u32);

        let window = sdl2::video::WindowBuilder::new(
            &video_subsystem,
            "Cirious",
            display_width,
            display_heigth
        ).opengl().borderless().position_centered().build().map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(
            Self {sdl, canvas}
        )
    }

    pub fn set_window_title(&mut self, title: &str) {
        self.canvas.window_mut().set_title(title);
    }

    pub fn set_window_size(&mut self, width: u32, heigth: u32) {
        self.canvas.window_mut().set_size(width, heigth);
    }

    pub fn set_window_borderless(&mut self, borderless: bool) {
        self.canvas.window_mut().set_bordered(borderless);
    }

    pub fn set_full_screen(&mut self, fullscreen: FullscreenType) {
        self.canvas.window_mut().set_fullscreen(
            FullscreenType::convert(fullscreen)
        );
    }

    pub fn get_event(&mut self) -> EventPollIterator {
        let mut event_pump = self.sdl.event_pump()?;
        event_pump.poll_iter()
    }
}