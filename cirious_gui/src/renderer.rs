use sdl2::render::{WindowCanvas, Texture};
use std::path::Path;
use sdl2::image::LoadTexture;

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

pub fn texture_create<'r>(
    canvas: &mut WindowCanvas,
    texture_path: &Path,
) -> Texture<'r> {
    let texture_creator = canvas.texture_creator();
    texture_creator.load_texture(texture_path)
        .expect("Não foi possivel carregar textura.")
}
