use std::time::Instant;

use rand::rngs::ThreadRng;
use sdl2::{
    event::Event,
    keyboard::KeyboardState,
    mouse::MouseState,
    render::{ Canvas, TextureCreator },
    ttf::{ self, Sdl2TtfContext },
    video::{ Window, WindowContext },
    EventPump,
};

use crate::object::enemy::Enemy;

use super::utils::{ Timer, SCREEN_SIZE };

pub fn setup<'a>() -> Result<
    (
        Canvas<Window>,
        EventPump,
        TextureCreator<WindowContext>,
        Vec<Enemy<'a>>,
        Timer,
        bool,
        Instant,
        ThreadRng,
        Sdl2TtfContext,
    ),
    String
> {
    let sdl2_context = sdl2::init().expect("Failed Initialized SDL2");
    let video_subsystem = sdl2_context.video().expect("Failed to create video subsystem");
    let window = video_subsystem
        .window("Wave Game ", SCREEN_SIZE.0 as u32, SCREEN_SIZE.1 as u32)
        .build()
        .expect("Failed Create Window");
    let canvas = window.into_canvas().build().expect("Failed Create Canvas");
    let event_pump = sdl2_context.event_pump().expect("Failed create event pump");
    let texture_creator = canvas.texture_creator();
    let enemies = Vec::<Enemy>::new();
    let enemy_timer = Timer::new(3.0);
    let rng = rand::thread_rng();
    let font_context = ttf::init().expect("Failed Initialized Font API");
    let running = true;
    let last_time = Instant::now();
    Ok((
        canvas,
        event_pump,
        texture_creator,
        enemies,
        enemy_timer,
        running,
        last_time,
        rng,
        font_context,
    ))
}

pub fn handle_input<'a>(
    running: &mut bool,
    event_pump: &'a mut EventPump
) -> Result<(KeyboardState<'a>, MouseState), String> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                *running = false;
            }
            _ => (),
        }
    }
    let keyboard_state = event_pump.keyboard_state();
    let mouse_state = event_pump.mouse_state();

    Ok((keyboard_state, mouse_state))
}
