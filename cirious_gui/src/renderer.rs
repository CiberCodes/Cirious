use sdl2::render::{WindowCanvas, Texture, TextureQuery};
use std::path::Path;
use sdl2::image::LoadTexture;
use sdl2::rect::{Point, Rect};
use sdl2::ttf::FontStyle;
use sdl2::pixels::Color;
use crate::widgets::{Button, Image, Text};

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn clear_canvas(
    canvas: &mut WindowCanvas
) {
    canvas.clear();
}

pub fn canvas_present(
    canvas: &mut WindowCanvas
) {
    canvas.present();
}

pub fn render_background(canvas: &mut WindowCanvas, texture: &Texture) -> Result<(), String> {
    canvas.copy(&texture, None, None)?;
    Ok(())
}

pub fn render_image(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    sprite: Rect,
    width : i32,
    height: i32,
) -> Image {
    let (canvas_width, canvas_height) = canvas.output_size()
        .expect("Falha ao ler o tamanho do canvas.");

    let screen_position = position + Point::new(
        canvas_width as i32 / 2,
        canvas_height as i32 / 2
    );

    let image = Rect::from_center(
        screen_position,
        width as u32,
        height as u32
    );

    canvas.copy(&texture, sprite, &image)
        .expect("Erro inexperado ao renderizar textura.");

    Image {
        location: image
    }
}

pub fn render_button(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    size: (u32, u32)
) -> Button {
    let (canvas_width, canva_height) = canvas.output_size()
        .expect("Falha ao obter o tamanho do canvas.");

    let screen_position = position + Point::new(
        canvas_width as i32 / 2,
        canva_height as i32 / 2,
    );

    let button = Rect::from_center(
        screen_position,
        size.0,
        size.1,
    );

    canvas.copy(&texture, None, button)
        .expect("Erro inexperado ao renderizar a textura.");

    Button {
        location: image
    }
}

fn get_centered_rect(
    canvas: &mut WindowCanvas,
    rect_width: u32,
    rect_height: u32,
    cons_width: u32,
    cons_height: u32
) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        }
        else {
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    }
    else {
        (rect_width as i32, rect_height as i32)
    };

    let (canvas_width, canvas_height) = canvas.output_size()
        .expect("Falha ao ler o tamanho do canvas.");

    let cx = (canvas_width as i32 - w) / 2;
    let cy = (canvas_height as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

pub fn render_text(
    canvas: &mut WindowCanvas,
    font_path: &Path,
    text: &str,
    size: u16,
    style: FontStyle,
    color: Color,
    position: Point
) {
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(font_path, size).unwrap();
    font.set_style(style);

    let surface = font.render(text)
        .blended(color).unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    render_font(canvas, &texture, position);
}

pub fn render_font(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
) -> Text {
    let (canvas_width, canvas_height) = canvas.output_size()
        .expect("Falha ao ler o tamanho do canvas.");

    let screen_position = position + Point::new(
        canvas_width as i32 / 2,
        canvas_height as i32 / 2
    );

    let TextureQuery { width, height, .. } = texture.query();

    let text = Rect::from_center(
        screen_position,
        width as u32,
        height as u32
    );

    canvas.copy(&texture, None, text)
        .expect("Erro inexperado ao renderizar textura.");

    Text {
        location: text,
    }
}