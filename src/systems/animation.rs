pub struct Animation {
    direction: Direction,
    duration: f32,
    elapsed: f32,
    start_x: f32,
    start_y: f32,
    distance: f32,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Animation {
    pub fn new(
        direction: Direction,
        duration: f32,
        start_x: f32,
        start_y: f32,
        distance: f32,
    ) -> Self {
        Self {
            direction,
            duration,
            elapsed: 0.0,
            start_x,
            start_y,
            distance,
        }
    }

    pub fn position(&self) -> (f32, f32) {
        let progress = if self.duration > 0.0 {
            (self.elapsed / self.duration).clamp(0.0, 1.0)
        } else {
            1.0
        };

        let offset = self.distance * progress;

        match self.direction {
            Direction::Right => (self.start_x + offset, self.start_y),
            Direction::Left => (self.start_x - offset, self.start_y),
            Direction::Down => (self.start_x, self.start_y + offset),
            Direction::Up => (self.start_x, self.start_y - offset),
        }
    }

    pub fn update(&mut self) {
        self.update_with_delta(macroquad::prelude::get_frame_time());
    }

    pub fn update_with_delta(&mut self, delta_time: f32) {
        if self.elapsed < self.duration {
            self.elapsed = (self.elapsed + delta_time.max(0.0)).min(self.duration);
        }
    }

    pub fn restart(
        &mut self,
        direction: Direction,
        duration: f32,
        start_x: f32,
        start_y: f32,
        distance: f32,
    ) {
        self.direction = direction;
        self.duration = duration;
        self.elapsed = 0.0;
        self.start_x = start_x;
        self.start_y = start_y;
        self.distance = distance;
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_animation_moves_to_the_right() {
        let mut animation = Animation::new(Direction::Right, 0.5, 100.0, 200.0, 280.0);

        assert_eq!(animation.position(), (100.0, 200.0));

        animation.update_with_delta(0.25);

        let (x, y) = animation.position();

        assert_eq!(x, 240.0);
        assert_eq!(y, 200.0);

        let (x, y) = animation.position();

        assert!(x > 100.0);
        assert_eq!(y, 200.0);
    }

    #[test]
    fn zero_duration_animation_finishes_immediately() {
        let animation = Animation::new(Direction::Right, 0.0, 100.0, 200.0, 280.0);

        assert_eq!(animation.position(), (380.0, 200.0));
    }
}
