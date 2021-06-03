use sdl2::rect::{Rect, Point};
use crate::renderer::render_text;
use sdl2::render::WindowCanvas;
use std::path::Path;
use sdl2::ttf::FontStyle;
use sdl2::pixels::Color;

pub struct Button {
    pub location: Rect
}

impl Button {
    pub fn add_text(
        &self,
        canvas: &mut WindowCanvas,
        font_path: &Path,
        text: &str,
        size: u16,
        style: FontStyle,
        color: Color
    ) {
        render_text(
            canvas,
            font_path,
            text,
            size,
            style,
            color,
            Point::new(
                self.location.x(),
                self.location.y()
            )
        )
    }
}

pub struct Image {
    pub location: Rect
}

pub struct Text {
    pub location: Rect
}