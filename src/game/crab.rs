use ggez::{Context, GameResult};
use ggez::graphics;
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{CRAB_W,
                          CRAB_S};

pub struct Crab {
    pub location: Point2,
    velocity: Vector2,
    w: f32,
}

impl Crab {
    pub fn new(location: Point2) -> GameResult<Crab> {
        let c = Crab {
            location,
            velocity: Vector2::new(CRAB_S, 0.0),
            w: CRAB_W,
        };
        Ok(c)
    }

    pub fn update(&mut self, max_screen: f32) -> GameResult<&Self> {
        self.location += self.velocity;

        if self.is_outside_screen(max_screen) {
            self.velocity = -self.velocity;
        }

        Ok(self)
    }

    fn is_outside_screen(&self, max_screen: f32) -> bool {
        (self.location.coords.x + 1.5 * self.w) > max_screen ||
            self.location.coords.x < 0.0
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        let location = graphics::DrawParam::new()
            .dest(self.location)
            .scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, img, location)?;
        Ok(self)
    }
}
