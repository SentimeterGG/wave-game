use core::{ engine, text::Text, utils::SCREEN_SIZE };
use std::time::Instant;

use object::{ enemy::Enemy, player::Player };
use rand::Rng;
use sdl2::pixels::Color;
mod core {
    pub mod engine;
    pub mod utils;
    pub mod text;
}
mod object {
    pub mod bullet;
    pub mod player;
    pub mod enemy;
}

fn main() {
    let (
        mut canvas,
        mut event_pump,
        texture_creator,
        mut enemies,
        mut enemy_timer,
        mut running,
        mut last_time,
        mut rng,
        font_context,
    ) = engine::setup().expect("Failed To Setup System:");

    let mut player = Player::new(&texture_creator);
    let mut text = Text::new(
        &texture_creator,
        &font_context,
        "0",
        "vhs-gothic.ttf",
        32,
        ((SCREEN_SIZE.0 / 2.0) as i32, 50),
        None
    );
    let mut point = 0;
    while running {
        // Delta Time
        let now = Instant::now();
        let delta_time = last_time.elapsed().as_secs_f32();
        last_time = now;

        // Handle Input
        let (keyboard_state, mouse_state) = engine
            ::handle_input(&mut running, &mut event_pump)
            .expect("Failed Handle Input:");
        player.handle_input(keyboard_state, mouse_state);

        // Process
        enemy_timer.start(delta_time);
        if enemy_timer.is_ended() {
            let mut enemy_spawn_position: (f32, f32) = (
                rng.gen_range(-45.0..655.0),
                rng.gen_range(-45.0..405.0),
            );
            while
                enemy_spawn_position.0 < SCREEN_SIZE.0 &&
                enemy_spawn_position.0 > -30.0 &&
                enemy_spawn_position.1 < SCREEN_SIZE.1 &&
                enemy_spawn_position.1 > -30.0
            {
                enemy_spawn_position = (rng.gen_range(-45.0..655.0), rng.gen_range(-45.0..405.0));
            }
            enemies.push(
                Enemy::new((enemy_spawn_position.0, enemy_spawn_position.1), &texture_creator)
            );
        }
        player.process(delta_time);
        for enemy in &mut enemies {
            enemy.process(delta_time, &mut player);
            player.check_hit(enemy.rect);
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        //Render Stuff
        enemies.retain(|enemy| {
            if enemy.destroy {
                point += 1;
                text.set_text(point.to_string().as_str());
                false
            } else {
                true
            }
        });
        player.render(&mut canvas);
        enemies.iter_mut().for_each(|enemy| enemy.render(&mut canvas));
        text.render(&mut canvas);
        canvas.present();
    }
}
