use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{CLAW_W, CLAW_H, CLAW_S};

pub enum Directions {
    Up,
    Down,
    Left,
    Right
}

pub struct Claw {
    pub location: Point2,
    body_anchor: Vector2,
    joint_anchor: Vector2,
    w: f32,
    h: f32,
    s: f32
}

impl Claw {
    pub fn new(location: Point2,
               body_anchor: Vector2,
               joint_anchor: Vector2) -> GameResult<Claw> {
        let c = Claw {
            location,
            body_anchor,
            joint_anchor,
            w: CLAW_W,
            h: CLAW_H,
            s: CLAW_S
        };
        Ok(c)
    }

    pub fn update(&mut self, parent_loc: Point2) -> GameResult<&Self> {
        self.location = parent_loc;
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        let position = graphics::DrawParam::new()
            .dest(self.get_origin())
            .scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, img, position)?;

        let body_location = self.location + self.body_anchor;
        let self_location = self.location + self.joint_anchor;
        let arm = graphics::Mesh::new_line(
            ctx,
            &[body_location, self_location],
            10.0,
            graphics::Color::new(1.0, 0.0, 0.0, 1.0)
        )?;
        graphics::draw(ctx, &arm, graphics::DrawParam::default())?;

        Ok(self)
    }

    pub fn get_origin(&self) -> Point2 {
        self.location
            + self.joint_anchor
            - Vector2::new(self.w / 2.0, self.h)
    }

    pub fn movedir(&mut self, dir:Directions) {
        use Directions::*;
        self.joint_anchor += match dir {
            Right => Vector2::new(self.s, 0.),
            Left => Vector2::new(-self.s, 0.),
            Down => Vector2::new(0., self.s),
            Up => Vector2::new(0., -self.s),
        };
    }
}
