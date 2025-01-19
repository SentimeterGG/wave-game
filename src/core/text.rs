use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{ Canvas, Texture, TextureCreator, TextureQuery },
    ttf::{ Font, Sdl2TtfContext },
    video::{ Window, WindowContext },
};
pub struct Text<'text> {
    texture_creator: &'text TextureCreator<WindowContext>,
    texture: Texture<'text>,
    width: u32,
    height: u32,
    position: (i32, i32),
    font: Font<'text, 'static>,
    color: Color,
}
impl<'text> Text<'text> {
    pub fn new(
        texture_creator: &'text TextureCreator<WindowContext>,
        ttf_context: &'text Sdl2TtfContext,
        string: &str,
        font_path: &'text str,
        size: u16,
        position: (i32, i32),
        modulate: Option<Color>
    ) -> Self {
        let font = ttf_context.load_font(font_path, size).expect("Failed to load font:");
        let color = modulate.unwrap_or(Color::RGB(255, 255, 255));
        let surface = font.render(string).blended(color).expect("Failed To Create Text");
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .expect("Failed to Create Texture from surface");
        let TextureQuery { width, height, .. } = texture.query();
        Text { texture_creator, texture, width, height, position, font, color }
    }
    pub fn set_text(&mut self, string: &str) {
        let surface = self.font.render(string).blended(self.color).expect("Failed to create text surface");
        let texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .expect("Failed to create texture from surface");
        let TextureQuery { width, height, .. } = texture.query();

        // Update the struct fields with the new texture and dimensions
        self.texture = texture;
        self.width = width;
        self.height = height;
    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        let center_horizontal: i32 = ((self.width / 2) as i32) * -1;
        let center_vertical: i32 = ((self.height / 2) as i32) * -1;
        canvas
            .copy(
                &self.texture,
                None,
                Rect::new(
                    center_horizontal + self.position.0,
                    center_vertical + self.position.1,
                    self.width,
                    self.height
                )
            )
            .expect("Failed To Render text");
    }
}
