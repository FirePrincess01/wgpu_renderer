use rusttype;

pub fn create_font_free_mono() -> rusttype::Font<'static> {
    let font_data = include_bytes!("FreeMono.ttf");
    let font = rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    font
}