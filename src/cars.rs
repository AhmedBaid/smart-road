use macroquad::prelude::*;
#[derive(Clone)]
pub struct Car {
    pub direction: String,
    pub width: i32,
    pub height: i32,
    pub cord: (f32, f32),
    pub color: Color,
    pub speed: f32,
}

impl Car {
    pub fn new(direction: String, width: i32, height: i32, cord: (f32, f32), color: i32) -> Self {
        let color = match color {
            1 => YELLOW,
            2 => BLUE,
            _ => RED,
        };
        Self {
            direction,
            width,
            height,
            cord,
            color,
            speed: 360.0,
        }
    }

    // Added 'blocked' parameter
    pub fn update(&mut self, dt: f32, state: String, blocked: bool) {
        if blocked {
            return;
        }

        let (mut x, mut y) = self.cord;

        match self.direction.as_str() {
            "up" => {
                if
                    y >= screen_height() / 2.0 + 12.0 &&
                    y <= screen_height() / 2.0 + 18.0 &&
                    self.color == RED
                {
                    y = screen_height() / 2.0 + 15.0;
                    x += self.speed * dt;
                } else if
                    y >= screen_height() / 2.0 - 48.0 &&
                    y <= screen_height() / 2.0 - 43.0 &&
                    self.color == YELLOW
                {
                    y = screen_height() / 2.0 - 45.0;
                    x -= self.speed * dt;
                } else {
                    if state == self.direction {
                        y -= self.speed * dt;
                    } else if y >= screen_height() / 2.0 + 65.0 {
                        y -= self.speed * dt;
                    } else if y < screen_height() / 2.0 + 60.0 {
                        y -= self.speed * dt;
                    }
                }
            }
            "down" => {
                if
                    y >= screen_height() / 2.0 - 48.0 &&
                    y <= screen_height() / 2.0 - 43.0 &&
                    self.color == RED
                {
                    y = screen_height() / 2.0 - 45.0;
                    x -= self.speed * dt;
                } else if
                    y >= screen_height() / 2.0 + 12.0 &&
                    y <= screen_height() / 2.0 + 18.0 &&
                    self.color == YELLOW
                {
                    y = screen_height() / 2.0 + 15.0;
                    x += self.speed * dt;
                } else {
                    if state == self.direction {
                        y += self.speed * dt;
                    } else if y <= screen_height() / 2.0 - 95.0 {
                        y += self.speed * dt;
                    } else if y > screen_height() / 2.0 - 90.0 {
                        y += self.speed * dt;
                    }
                }
            }
            "left" => {
                if
                    x >= screen_width() / 2.0 + 12.0 &&
                    x <= screen_width() / 2.0 + 18.0 &&
                    self.color == RED
                {
                    x = screen_width() / 2.0 + 15.0;
                    y -= self.speed * dt;
                } else if
                    x >= screen_width() / 2.0 - 48.0 &&
                    x <= screen_width() / 2.0 - 42.0 &&
                    self.color == YELLOW
                {
                    x = screen_width() / 2.0 - 45.0;
                    y += self.speed * dt;
                } else {
                    if state == self.direction {
                        x -= self.speed * dt;
                    } else if x >= screen_width() / 2.0 + 65.0 {
                        x -= self.speed * dt;
                    } else if x < screen_width() / 2.0 + 60.0 {
                        x -= self.speed * dt;
                    }
                }
            }
            "right" => {
                if
                    x >= screen_width() / 2.0 - 48.0 &&
                    x <= screen_width() / 2.0 - 42.0 &&
                    self.color == RED
                {
                    x = screen_width() / 2.0 - 45.0;
                    y += self.speed * dt;
                } else if
                    x >= screen_width() / 2.0 + 12.0 &&
                    x <= screen_width() / 2.0 + 18.0 &&
                    self.color == YELLOW
                {
                    x = screen_width() / 2.0 + 15.0;
                    y -= self.speed * dt;
                } else {
                    if state == self.direction {
                        x += self.speed * dt;
                    } else if x <= screen_width() / 2.0 - 95.0 {
                        x += self.speed * dt;
                    } else if x > screen_width() / 2.0 - 90.0 {
                        x += self.speed * dt;
                    }
                }
            }
            _ => {}
        }

        self.cord = (x, y);
    }
}
