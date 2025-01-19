use sdl2::{
    image::LoadTexture,
    mouse::MouseState,
    rect::FRect,
    render::{ Canvas, Texture, TextureCreator },
    video::{ Window, WindowContext },
};

use crate::core::utils::{ self, is_off_screen, Vector2 };

pub struct Bullet<'bullet> {
    texture: Texture<'bullet>,
    pub rect: FRect,
    velocity: (f32, f32),
    speed: f32,
    rotation: f32,
    look_once: bool,
    mouse_state: MouseState,
    pub destroy : bool,
}

impl<'bullet> Bullet<'bullet> {
    pub fn new(
        position: (f32, f32),
        texture_creator: &'bullet TextureCreator<WindowContext>,
        mouse_state: MouseState
    ) -> Self {
        let mut texture = texture_creator
            .load_texture("sprite.png")
            .expect("Failed to load Bullet Texture");
        texture.set_color_mod(70, 130, 180);
        Bullet {
            texture,
            rect: FRect::new(position.0, position.1, 15.0, 15.0),
            velocity: (1.0, 0.0),
            speed: 250.0,
            rotation: 0.0,
            look_once: true,
            mouse_state,
            destroy : false,
        }
    }

    pub fn process(&mut self, delta_time: f32) {
        if self.look_once == true {
            self.rotation = utils::look_at(
                (self.rect.x, self.rect.y),
                (self.mouse_state.x() as f32, self.mouse_state.y() as f32)
            );
            self.look_once = false;
        }
        self.rect.x += self.velocity.rotated(self.rotation).0 * self.speed * delta_time;
        self.rect.y += self.velocity.rotated(self.rotation).1 * self.speed * delta_time;
    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        if is_off_screen((self.rect.x, self.rect.y)) {
            self.destroy = true;
        }
        canvas.copy_f(&self.texture, None, self.rect).expect("Failed to render bullet");
    }
}
