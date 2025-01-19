pub static SCREEN_SIZE: (f32, f32) = (640.0, 360.0);
pub fn move_toward(from: f32, to: f32, delta: f32) -> f32 {
    if (to - from).abs() <= delta {
        return to;
    }
    return from + (to - from).signum() * delta;
}

pub fn look_at(from: (f32, f32), to: (f32, f32)) -> f32 {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    return dy.atan2(dx);
}
pub fn is_off_screen(object_position: (f32, f32)) -> bool {
    object_position.0 < 0.0 ||
        object_position.0 > SCREEN_SIZE.0 ||
        object_position.1 < 0.0 ||
        object_position.1 > SCREEN_SIZE.1
}

pub trait Vector2 {
    fn rotated(self, p_by: f32) -> Self;
    fn normalized(self) -> Self;
    fn direction_to(self, to: (f32, f32)) -> Self;
}
impl Vector2 for (f32, f32) {
    fn rotated(self, p_by: f32) -> Self {
        let sine = p_by.sin();
        let cosi = p_by.cos();

        (self.0 * cosi - self.1 * sine, self.0 * sine + self.1 * cosi)
    }
    fn normalized(mut self) -> Self {
        let mut magnitude = self.0.powi(2) * self.1.powi(2);
        if magnitude != 0.0 {
            magnitude = magnitude.sqrt();
            self.0 /= magnitude;
            self.1 /= magnitude;
        }
        self
    }
    fn direction_to(self, to: (f32, f32)) -> Self {
        let ret = (to.0 - self.0, to.1 - self.1);
        ret.normalized();
        ret
    }
    
}
// Timer
pub struct Timer {
    duration: f32,
    pub elapsed: f32,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Timer {
            duration,
            elapsed: 0.0,
        }
    }

    pub fn start(&mut self, delta_time: f32) {
        self.elapsed += delta_time;
    }
    pub fn is_ended(&mut self) -> bool {
        if self.elapsed >= self.duration {
            self.elapsed = 0.0;
            return true;
        }
        false
    }
}
