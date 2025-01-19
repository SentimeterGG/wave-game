use fixed::types::U4F4;
use sdl2::{
    image::LoadTexture,
    keyboard::{ KeyboardState, Scancode },
    mouse::{ MouseButton, MouseState },
    rect::FRect,
    render::{ Canvas, Texture, TextureCreator },
    video::{ Window, WindowContext },
};

use crate::core::utils::{ self, Timer, Vector2 };

use super::bullet::Bullet;
pub static COLOR: (u8, u8, u8) = (70, 130, 180);
pub struct Player<'a> {
    pub rect: FRect,
    speed: f32,
    accel: f32,
    direction: (f32, f32),
    velocity: (f32, f32),
    texture: Texture<'a>,
    texture_creator: &'a TextureCreator<WindowContext>,
    // Bullet
    pub bullets: Vec<Bullet<'a>>,
    // Input
    one_shot_input: bool,
    health: U4F4,
    stun_timer: Timer,
    hit: bool,
}

impl<'a> Player<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let rect = FRect::new(0.0, 0.0, 30.0, 30.0);
        let mut texture = texture_creator
            .load_texture("sprite.png")
            .expect("Failed to Load Player Texture: ");
        texture.set_color_mod(COLOR.0, COLOR.1, COLOR.2);
        Player {
            rect,
            speed: 300.0,
            accel: 1200.0,
            velocity: (0.0, 0.0),
            direction: (0.0, 0.0),
            texture,
            texture_creator,
            bullets: Vec::new(),
            one_shot_input: true,
            health: U4F4::from_num(15),
            stun_timer: Timer::new(1.0),
            hit: false,
        }
    }
    pub fn check_hit(&mut self, enemy_rect: FRect) {
        if self.rect.has_intersection(enemy_rect) {
            self.hit = true;
        }
        self.hit = false;
    }
    pub fn handle_input(&mut self, keyboard_state: KeyboardState, mouse_state: MouseState) {
        self.direction = (0.0, 0.0);
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            self.direction.1 = -1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            self.direction.1 = 1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            self.direction.0 = 1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            self.direction.0 = -1.0;
        }

        // Mouse
        if mouse_state.is_mouse_button_pressed(MouseButton::Left) {
            if self.one_shot_input == true {
                self.bullets.push(
                    Bullet::new((self.rect.x, self.rect.y), &self.texture_creator, mouse_state)
                );
                self.one_shot_input = false;
            }
        } else {
            self.one_shot_input = true;
        }
    }
    pub fn process(&mut self, delta_time: f32) {
        if self.hit {
            self.stun_timer.start(delta_time);
            self.texture.set_color_mod(255, 255, 255);
        }
        if self.stun_timer.is_ended() {
            self.texture.set_color_mod(COLOR.0, COLOR.1, COLOR.2);
            self.health -= U4F4::from_num(1);
            self.hit = false;
        }
        self.bullets.iter_mut().for_each(|bullet| bullet.process(delta_time));
        self.velocity.0 = utils::move_toward(
            self.velocity.0,
            self.direction.0 * self.speed,
            delta_time * self.accel
        );
        self.velocity.1 = utils::move_toward(
            self.velocity.1,
            self.direction.1 * self.speed,
            delta_time * self.accel
        );
        self.velocity.normalized();
        if !self.hit {
            self.rect.x += self.velocity.0 * delta_time;
            self.rect.y += self.velocity.1 * delta_time;
        }
    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        self.bullets.retain(|bullet| !bullet.destroy);
        self.bullets.iter_mut().for_each(|bullet| bullet.render(canvas));
        canvas.copy_f(&self.texture, None, self.rect).expect("Failed to render player: ")
    }
}
