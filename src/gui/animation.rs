// animation.rs — простая анимация fade/scale.
pub struct Animation {
    duration: f32,
    elapsed: f32,
    value: f32,
}

impl Animation {
    pub fn new(duration: f32) -> Self {
        Self { duration, elapsed: 0.0, value: 0.0 }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed = (self.elapsed + dt).min(self.duration);
        self.value = self.elapsed / self.duration;
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}