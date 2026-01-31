//! Classic Matrix rain effect: the iconic green falling characters.

use super::Effect;
use crate::buffer::ScreenBuffer;
use crate::rain::RainField;

/// The classic Matrix digital rain effect.
pub struct ClassicRain {
    rain: RainField,
}

impl ClassicRain {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            rain: RainField::new(width, height),
        }
    }
}

impl Effect for ClassicRain {
    fn name(&self) -> &str {
        "classic"
    }

    fn update(&mut self, delta_time: f64) {
        self.rain.update(delta_time);
    }

    fn render(&self, buffer: &mut ScreenBuffer) {
        self.rain.render(buffer);
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.rain.resize(width, height);
    }
}
