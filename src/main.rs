// Other files in the same directory can be accessed by declaring them as modules.
mod epd;
mod text_renderer;

use text_renderer::text_renderer::TextRenderer;

fn main() {
    let mut epd = epd::EPD266::new().unwrap();
    let image = [0xff; epd::SCREEN_BYTES];

    //epd.full_update(&image).unwrap();
    //epd.fast_update(&image, &[0; epd::SCREEN_BYTES]).unwrap();
    
    // 152x296 display (not rotating right now) is 25x37
    let mut tr = TextRenderer::new();
    tr.set_char(10, 10, '#').unwrap();
    let f = tr.to_1bpp();

    epd.full_update(&f).unwrap();
}
