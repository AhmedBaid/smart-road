use macroquad::prelude::*;

#[derive(Clone)]
pub struct Car {
    pub direction: String,
    pub width: i32,
    pub height: i32,
    pub cord: (f32, f32),
    pub speed: f32,
    pub target_speed: f32, // السرعة الذكية المطلوبة
    pub max_speed: f32,    // السرعة القصوى
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
        let initial_speed = macroquad::rand::gen_range(150.0, 250.0);
        Self {
            direction,
            width,
            height,
            cord,
            speed: initial_speed,
            target_speed: initial_speed,
            max_speed: initial_speed,
            rotation,
        }
    }

    // دالة لاستنتاج اتجاه حركة سيارتك (لتركيب الرادار في الأمام)
    pub fn current_heading(&self) -> (f32, f32) {
        let (x, y) = self.cord;
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        let mut dx = 0.0;
        let mut dy = 0.0;

        // نفس منطق الانعطاف الخاص بك
        match self.direction.as_str() {
            "up_right" => { if y < cy + 80.0 { dx = 1.0; } else { dy = -1.0; } }
            "up_stright" => { dy = -1.0; }
            "up_left" => { if y < cy - 40.0 { dx = -1.0; } else { dy = -1.0; } }
            "right_right" => { if x < cx - 115.0 { dx = 1.0; } else { dy = 1.0; } }
            "right_stright" => { dx = 1.0; }
            "right_left" => { if x < cx + 5.0 { dx = 1.0; } else { dy = -1.0; } }
            "down_right" => { if y < cy - 125.0 { dy = 1.0; } else { dx = -1.0; } }
            "down_stright" => { dy = 1.0; }
            "down_left" => { if y < cy - 5.0 { dy = 1.0; } else { dx = 1.0; } }
            "left_right" => { if x > cx + 85.0 { dx = -1.0; } else { dy = -1.0; } }
            "left_stright" => { dx = -1.0; }
            "left_left" => { if x > cx - 35.0 { dx = -1.0; } else { dy = 1.0; } }
            _ => {}
        }
        (dx, dy)
    }

    // صندوق التصادم
    pub fn get_rect(&self) -> Rect {
        Rect::new(self.cord.0, self.cord.1, self.width as f32, self.height as f32)
    }

    // إنشاء مستطيل الرادار أمام السيارة
    pub fn get_radar(&self) -> Rect {
        let (dx, dy) = self.current_heading();
        let (x, y) = self.cord;
        let w = self.width as f32;
        let h = self.height as f32;
        let radar_len = 70.0; // طول الرادار

        if dx > 0.1 { Rect::new(x + w, y, radar_len, h) }
        else if dx < -0.1 { Rect::new(x - radar_len, y, radar_len, h) }
        else if dy > 0.1 { Rect::new(x, y + h, w, radar_len) }
        else if dy < -0.1 { Rect::new(x, y - radar_len, w, radar_len) }
        else { Rect::new(x, y, 0.0, 0.0) }
    }

    pub fn update(&mut self, dt: f32) {
        // تسارع وتباطؤ سلس (Smooth Velocity)
        let accel = 400.0;
        if self.speed < self.target_speed {
            self.speed += accel * dt;
            if self.speed > self.target_speed { self.speed = self.target_speed; }
        } else if self.speed > self.target_speed {
            self.speed -= accel * 2.0 * dt;
            if self.speed < self.target_speed { self.speed = self.target_speed; }
        }

        let step = self.speed * dt;
        let (mut x, mut y) = self.cord;

        // الكود الأصلي الخاص بك للحركة تماماً 100%
        match self.direction.as_str() {
            "up_right" => {
                if y < screen_height() / 2.0 + 80.0 {
                    x += step;
                    self.rotation = 90.0;
                } else {
                    y -= step;
                    self.rotation = 0.0;
                }
            }
            "up_stright" => { y -= step; }
            "up_left" => {
                if y < screen_height() / 2.0 - 40.0 {
                    x -= step;
                    self.rotation = 270.0;
                } else {
                    y -= step;
                }
            }
            "right_right" => {
                if x < screen_width() / 2.0 - 115.0 {
                    x += step;
                } else {
                    y += step;
                    self.rotation = 180.0;
                }
            }
            "right_stright" => { x += step; }
            "right_left" => {
                if x < screen_width() / 2.0 + 5.0 {
                    x += step;
                } else {
                    y -= step;
                    self.rotation = 0.0;
                }
            }
            "down_right" => {
                if y < screen_height() / 2.0 - 125.0 {
                    y += step;
                } else {
                    x -= step;
                    self.rotation = 270.0;
                }
            }
            "down_stright" => { y += step; }
            "down_left" => {
                if y < screen_height() / 2.0 - 5.0 {
                    y += step;
                } else {
                    x += step;
                    self.rotation = 90.0;
                }
            }
            "left_right" => {
                if x > screen_width() / 2.0 + 85.0 {
                    x -= step;
                } else {
                    y -= step;
                    self.rotation = 0.0;
                }
            }
            "left_stright" => { x -= step; }
            "left_left" => {
                if x > screen_width() / 2.0 - 35.0 {
                    x -= step;
                } else {
                    y += step;
                    self.rotation = 180.0;
                }
            }
            _ => {}
        }

        self.cord = (x, y);
    }
}