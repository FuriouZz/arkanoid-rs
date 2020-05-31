use crate::pipelines::Sprite;
use fine::math::Vector2;

pub struct Ball {
    pub sprite: Sprite,
    acc: Vector2<f32>,
    released: bool,
}

impl Ball {
    pub fn new(sprite: Sprite) -> Self {
        Self {
            sprite,
            acc: Vector2::new(0.0, 0.0),
            released: false,
        }
    }

    pub fn transform_mut(&mut self) -> &mut fine::Transform {
        &mut self.sprite.transform
    }

    pub fn update_with_paddle(&mut self, paddle: &super::Paddle) {
        let sprite = &mut self.sprite;
        if !self.released {
            let t_paddle = paddle.transform().translation();
            let t_ball = sprite.transform.translation_mut();
            t_ball[0] = t_paddle[0];
            t_ball[1] = t_paddle[1] + 10.0;
        } else {
            // let px = paddle.sprite.x();
            // let py = paddle.sprite.y();
            // let pw = paddle.sprite.width();
            // let ph = paddle.sprite.height();

            // let bx = sprite.x();
            // let by = sprite.y();
            // let bw = sprite.width();
            // let bh = sprite.height();

            // if bx+bw*0.5 >= px || bx-bw*0.5 <= px+pw*0.5 {
            //     self.acc[0] *= -1.0;
            // }
            // if by+bh*0.5 >= py || by-bh*0.5 <= py+ph*0.5 {
            //     self.acc[1] *= -1.0;
            // }

            let t_ball = sprite.transform.translation_mut();
            t_ball[0] = t_ball[0] + self.acc[0];
            t_ball[1] = t_ball[1] - self.acc[1];
        }
    }

    pub fn update_with_brick(&mut self, brick: &mut super::Brick) {
        let (x, y, width, height) = self.sprite.rect();
        let (bx, by, bwidth, bheight) = brick.sprite.rect();

        if x >= bx || x+width*0.5 <= bx+bwidth*0.5 || y >= by || y+height*0.5 <= by+bheight {
            brick.alive = false;
        }
    }

    pub fn release(&mut self) {
        if !self.released {
            self.released = true;
            let range = std::f32::consts::PI * 0.25; // 45deg
            let angle = fine::rand() as f32 * range;
            let offset = std::f32::consts::PI * -0.5 - range * 0.5;
            self.acc[0] = 1. * f32::cos(offset + angle);
            self.acc[1] = 1. * f32::sin(offset + angle);
        }
    }
}