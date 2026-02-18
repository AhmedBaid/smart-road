use macroquad::prelude::*;
#[derive(Clone)]
pub struct Car {
    pub direction: String,
    pub width: i32,
    pub height: i32,
    pub cord: (f32, f32),
    pub speed: f32,
    pub rotation: f32,
}

impl Car {
    pub fn new(
        direction: String,
        width: i32,
        height: i32,
        cord: (f32, f32),
        rotation: f32,
    ) -> Self {
        Self {
            direction,
            width,
            height,
            cord,
            speed: 360.0,
            rotation,
        }
    }

    pub fn update(&mut self, dt: f32, blocked: bool) {
        if blocked {
            return;
        }
        let (mut x, mut y) = self.cord;

        match self.direction.as_str() {
            "up_right" => {
                if y < screen_height() / 2.0 + 80.0 {
                    x += self.speed * dt;
                    self.rotation = 90.0;
                } else {
                    y -= self.speed * dt;
                    self.rotation = 0.0;
                }
            }
            "up_stright" => {
                y -= self.speed * dt;
            }
            "up_left" => {
                if y < screen_height() / 2.0 - 40.0 {
                    x -= self.speed * dt;
                    self.rotation = 270.0;
                } else {
                    y -= self.speed * dt;
                }
            }
            "right_right" => {
                if x < screen_width() / 2.0 - 115.0 {
                    x += self.speed * dt;
                } else {
                    y += self.speed * dt;
                    self.rotation = 180.0;
                }
            }
            "right_stright" => {
                x += self.speed * dt;
            }
            "right_left" => {
                if x < screen_width() / 2.0 + 5.0 {
                    x += self.speed * dt;
                } else {
                    y -= self.speed * dt;
                    self.rotation = 0.0;
                }
            }
            "down_right" => {
                if y < screen_height() / 2.0 - 125.0 {
                    y += self.speed * dt;
                } else {
                    x -= self.speed * dt;
                    self.rotation = 270.0;
                }
            }
            "down_stright" => {
                y += self.speed * dt;
            }
            "down_left" => {
                if y < screen_height() / 2.0 - 5.0 {
                    y += self.speed * dt;
                } else {
                    x += self.speed * dt;
                    self.rotation = 90.0;
                }
            }
            "left_right" => {
                if x > screen_width() / 2.0 + 85.0 {
                    x -= self.speed * dt;
                } else {
                    y -= self.speed * dt;
                    self.rotation = 0.0;
                }
            }
            "left_stright" => {
                x -= self.speed * dt;
            }
            "left_left" => {
                if x > screen_width() / 2.0 - 35.0 {
                    x -= self.speed * dt;
                } else {
                    y += self.speed * dt;
                    self.rotation = 180.0;
                }
            }
            _ => {}
        }

        self.cord = (x, y);
    }
}
