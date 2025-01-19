use fixed::types::U4F4;
use sdl2::{
    image::LoadTexture,
    rect::FRect,
    render::{ Canvas, Texture, TextureCreator },
    video::{ Window, WindowContext },
};
use crate::core::utils::{ Timer, Vector2 };
use super::{ bullet::Bullet, player::Player };

pub static COLOR: (u8, u8, u8) = (239, 19, 62);
pub struct Enemy<'a> {
    pub rect: FRect,
    speed: f32,
    velocity: (f32, f32),
    texture: Texture<'a>,
    stun_timer: Timer,
    hit: bool,
    health: U4F4,
    pub destroy: bool,
    knockback: (f32, f32),
}

impl<'a> Enemy<'a> {
    pub fn new(
        spawn_position: (f32, f32),
        texture_creator: &'a TextureCreator<WindowContext>
    ) -> Self {
        let rect = FRect::new(spawn_position.0, spawn_position.1, 30.0, 30.0);
        let mut texture = texture_creator
            .load_texture("sprite.png")
            .expect("Failed to Load Enemy Texture: ");
        texture.set_color_mod(COLOR.0, COLOR.1, COLOR.2);
        Enemy {
            rect,
            speed: 1.0,
            velocity: (0.0, 0.0),
            texture,
            stun_timer: Timer::new(0.1),
            hit: false,
            health: U4F4::from_num(2),
            destroy: false,
            knockback: (0.0, 0.0),
        }
    }

    fn hit(&mut self, bullet: &mut Bullet) {
        self.texture.set_color_mod(255, 255, 255);
        self.hit = true;
        bullet.destroy = true;

        //Knockback
        let direction = (self.rect.x, self.rect.y).direction_to((bullet.rect.x, bullet.rect.y));
        self.knockback = (-direction.0 * 5.0, -direction.1 * 5.0);
    }

    fn return_normal(&mut self) {
        if self.health != 0 {
            self.health -= U4F4::from_num(1);
        }
        self.texture.set_color_mod(COLOR.0, COLOR.1, COLOR.2);
        self.hit = false;
    }

    pub fn process(&mut self, delta_time: f32, player: &mut Player<'a>) {
        for bullet in player.bullets.iter_mut() {
            if self.hit == false {
                if self.rect.has_intersection(bullet.rect) {
                    self.hit(bullet); //Hit Function
                }
            }
        }
        //If Enemy Get Hit
        if self.hit == true {
            self.stun_timer.start(delta_time);
        }
        //If Stun Timer Ended
        if self.stun_timer.is_ended() {
            self.return_normal();
        }
        if self.health == 0 {
            self.destroy = true;
        }
        //Apply Knockback
        if self.hit {
            self.rect.x += self.knockback.0 * delta_time;
            self.rect.y += self.knockback.1 * delta_time;
        }
        // Default Movement
        if !self.hit {
            self.velocity = (self.rect.x, self.rect.y).direction_to((player.rect.x, player.rect.y));
            self.rect.x += self.velocity.0 * self.speed * delta_time;
            self.rect.y += self.velocity.1 * self.speed * delta_time;
        }
    }
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.copy_f(&self.texture, None, self.rect).expect("Failed to render enemy: ")
    }
}
