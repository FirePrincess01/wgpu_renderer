//! Creates an image with a text label

use rusttype;
use image;

pub struct Label {
    scale: rusttype::Scale,
    vertical_metrics: rusttype::VMetrics,
    color: (u8, u8, u8),
    border: u32,
    image:  image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl Label {
    pub fn new(font: &rusttype::Font, scale: f32, text: &str) -> Self {

        let scale = rusttype::Scale::uniform(scale);
        let vertical_metrics = font.v_metrics(scale);
        let height = (vertical_metrics.ascent - vertical_metrics.descent).ceil() as u32;
        let color = (150, 150, 150);
        let border = 1;

        let glyphs= Self::create_glyphs(font, scale, border, vertical_metrics, text);
        let width = Self::calculate_width(&glyphs);

        let mut image = image::DynamicImage::new_rgba8(
            width + border * 2, 
            height + border * 2).to_rgba8();

        Self::draw_glyphs(&glyphs, &mut image, color);

        Self {
            scale,
            vertical_metrics,
            color,
            border,
            image,
        }
    }

    fn calculate_width(glyphs: &[rusttype::PositionedGlyph]) -> u32
    {
        let mut width = 0.0;
        for glyph in glyphs 
        {
            let scaled_glyph = glyph.unpositioned();
            let horizontal_metrics = scaled_glyph.h_metrics();
            width = width + horizontal_metrics.advance_width;
        }

        width as u32
    }

    fn create_glyphs<'a>(font: &'a rusttype::Font, 
        scale: rusttype::Scale, 
        border: u32,
        vertical_metrics: rusttype::VMetrics, 
        text: &str) -> Vec<rusttype::PositionedGlyph<'a>>
    {
        // layout the glyphs in a line with 20 pixels padding
        let glyphs: Vec<rusttype::PositionedGlyph<'_>> = font
            .layout(text, scale, rusttype::point(border as f32, border as f32 + vertical_metrics.ascent))
            .collect();

        glyphs
    }

    fn clear_image(image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>)
    {
        // let bla = image.enumerate_pixels();
        for (_x, _y, val) in image.enumerate_pixels_mut() {
            *val = image::Rgba([0, 0, 0, 0 as u8]);
        }
    }

    fn draw_glyphs(glyphs: &[rusttype::PositionedGlyph], image: &mut  image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, color: (u8, u8, u8)) 
    {
        Self::clear_image(image);

        // Loop through the glyphs in the text, positing each one on a line
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    let x = x + bounding_box.min.x as u32;
                    let y = y + bounding_box.min.y as u32;
                    if x < image.width() && y < image.height()
                    {
                        image.put_pixel(
                            // Offset the position by the glyph bounding box
                            x,
                            y,
                            // Turn the coverage into an alpha value
                            image::Rgba([color.0, color.1, color.2, (v * 255.0) as u8]),
                        )
                    }
                });
            }
        }
    }

    pub fn update<'a>(&mut self, font: &'a rusttype::Font, text: &str) 
    {
        let glyphs = Self::create_glyphs(font, self.scale, self.border, self.vertical_metrics, text);
        
        Self::draw_glyphs(&glyphs, &mut self.image, self.color);
    }

    pub fn get_image(&self) -> &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
    {
        &self.image
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

}
