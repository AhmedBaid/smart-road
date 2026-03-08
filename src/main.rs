use macroquad::prelude::*;
mod cars;
mod dashed;
mod draw_road;
use cars::*;
use draw_road::*;

#[derive(Clone, Debug)]
pub struct Stats {
    pub total_cars: u32,
    pub nbr_passed: u32,
    pub max_velocity: f32,
    pub min_velocity: f32,
    pub average_velocity: f32,
    pub max_time: f32,
    pub min_time: f32,
    pub total_velocity_sum: f64,
    pub velocity_samples: u64,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            total_cars: 0,
            nbr_passed: 0,
            max_velocity: f32::MIN,
            min_velocity: f32::MAX,
            average_velocity: 0.0,
            max_time: f32::MIN,
            min_time: f32::MAX,
            total_velocity_sum: 0.0,
            velocity_samples: 0,
        }
    }

    pub fn sample_velocity(&mut self, velocity: f32) {
        if velocity > self.max_velocity { self.max_velocity = velocity; }
        if velocity < self.min_velocity { self.min_velocity = velocity; }
        self.total_velocity_sum += velocity as f64;
        self.velocity_samples += 1;
        self.average_velocity = (self.total_velocity_sum / self.velocity_samples as f64) as f32;
    }

    pub fn register_passed_car(&mut self, time_lived: f32) {
        self.nbr_passed += 1;
        if time_lived > self.max_time { self.max_time = time_lived; }
        if time_lived < self.min_time { self.min_time = time_lived; }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "road_intersection".to_string(),
        window_width: 1100,
        window_height: 800,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

// Fixed: Checks ALL cars, regardless of direction, to prevent spawning on top of cross-traffic
fn can_spawn(cars: &Vec<Car>, spawn_cord: (f32, f32)) -> bool {
    let safe_dist = 85.0; 
    for car in cars {
        let dist = ((car.cord.0 - spawn_cord.0).powi(2) + (car.cord.1 - spawn_cord.1).powi(2)).sqrt();
        if dist < safe_dist {
            return false;
        }
    }
    true
}

#[macroquad::main(window_conf)]
async fn main() {
    let car_tex: Texture2D = load_texture("assets/car2.png").await.unwrap();
    car_tex.set_filter(FilterMode::Nearest);
    let mut cars: Vec<Car> = Vec::new();

    let mut stats = Stats::new();
    let mut show_stats = false;

    loop {
        let dt = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            if show_stats {
                break; 
            } else {
                show_stats = true; 
            }
        }

        if show_stats {
            clear_background(Color::from_rgba(4, 96, 85, 255));
            draw_road();
            
            for car in &cars {
                draw_texture_ex(
                    &car_tex, car.cord.0, car.cord.1, WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(car.width as f32, car.height as f32)),
                        rotation: car.rotation.to_radians(),
                        ..Default::default()
                    },
                );
            }

            let cx = screen_width() / 2.0;
            let cy = screen_height() / 2.0;
            draw_rectangle(cx - 200.0, cy - 220.0, 400.0, 440.0, Color::from_rgba(0, 0, 0, 220));
            draw_text("STATISTICS", cx - 85.0, cy - 160.0, 35.0, WHITE);

            let min_v = if stats.min_velocity == f32::MAX { 0.0 } else { stats.min_velocity };
            let max_v = if stats.max_velocity == f32::MIN { 0.0 } else { stats.max_velocity };
            let min_t = if stats.min_time == f32::MAX { 0.0 } else { stats.min_time };
            let max_t = if stats.max_time == f32::MIN { 0.0 } else { stats.max_time };

            let text_x = cx - 160.0;
            draw_text(&format!("Total Cars: {}", stats.total_cars), text_x, cy - 100.0, 24.0, WHITE);
            draw_text(&format!("Passed Cars: {}", stats.nbr_passed), text_x, cy - 60.0, 24.0, WHITE);
            draw_text(&format!("Average Velocity: {:.2} px/s", stats.average_velocity), text_x, cy - 20.0, 24.0, WHITE);
            draw_text(&format!("Max Velocity: {:.2} px/s", max_v), text_x, cy + 20.0, 24.0, WHITE);
            draw_text(&format!("Min Velocity: {:.2} px/s", min_v), text_x, cy + 60.0, 24.0, WHITE);
            draw_text(&format!("Max Time Lived: {:.2} s", max_t), text_x, cy + 100.0, 24.0, WHITE);
            draw_text(&format!("Min Time Lived: {:.2} s", min_t), text_x, cy + 140.0, 24.0, WHITE);
            
            draw_text("Press ESC again to exit", cx - 100.0, cy + 195.0, 20.0, YELLOW);

            next_frame().await;
            continue;
        }

        clear_background(Color::from_rgba(4, 96, 85, 255));
        draw_road();

        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Backspace) {
            cars.clear();
        }

        if is_key_pressed(KeyCode::Up) {
            let random_case = rand::gen_range(0, 3);
            let (cord, direction) = match random_case {
                0 => ((screen_width() / 2.0 + 85.0, screen_height() - 55.0), "up_right"),
                1 => ((screen_width() / 2.0 + 45.0, screen_height() - 55.0), "up_stright"),
                _ => ((screen_width() / 2.0 + 5.0, screen_height() - 55.0), "up_left"),
            };
            if can_spawn(&cars, cord) {
                cars.push(Car::new(direction.to_string(), 30, 50, cord, 0.0));
                stats.total_cars += 1;
            }
        }

        if is_key_pressed(KeyCode::Right) {
            let random_case = rand::gen_range(0, 3);
            let (cord, direction) = match random_case {
                0 => ((10.0, screen_height() / 2.0 + 75.0), "right_right"),
                1 => ((10.0, screen_height() / 2.0 + 35.0), "right_stright"),
                _ => ((10.0, screen_height() / 2.0 - 5.0), "right_left"),
            };
            if can_spawn(&cars, cord) {
                cars.push(Car::new(direction.to_string(), 30, 50, cord, 90.0));
                stats.total_cars += 1;
            }
        }

        if is_key_pressed(KeyCode::Down) {
            let random_case = rand::gen_range(0, 3);
            let (cord, direction) = match random_case {
                0 => ((screen_width() / 2.0 - 115.0, 5.0), "down_right"),
                1 => ((screen_width() / 2.0 - 75.0, 5.0), "down_stright"),
                _ => ((screen_width() / 2.0 - 35.0, 5.0), "down_left"),
            };
            if can_spawn(&cars, cord) {
                cars.push(Car::new(direction.to_string(), 30, 50, cord, 180.0));
                stats.total_cars += 1;
            }
        }

        if is_key_pressed(KeyCode::Left) {
            let random_case = rand::gen_range(0, 3);
            let (cord, direction) = match random_case {
                0 => ((screen_width() - 40.0, screen_height() / 2.0 - 125.0), "left_right"),
                1 => ((screen_width() - 40.0, screen_height() / 2.0 - 85.0), "left_stright"),
                _ => ((screen_width() - 40.0, screen_height() / 2.0 - 45.0), "left_left"),
            };
            if can_spawn(&cars, cord) {
                cars.push(Car::new(direction.to_string(), 30, 50, cord, 270.0));
                stats.total_cars += 1;
            }
        }

        if is_key_pressed(KeyCode::R) {
            let random_dir = rand::gen_range(0, 12);
            let (cord, direction, rotation) = match random_dir {
                0 => ((screen_width() / 2.0 + 85.0, screen_height() - 55.0), "up_right", 0.0),
                1 => ((screen_width() / 2.0 + 45.0, screen_height() - 55.0), "up_stright", 0.0),
                2 => ((screen_width() / 2.0 + 5.0, screen_height() - 55.0), "up_left", 0.0),
                3 => ((10.0, screen_height() / 2.0 + 75.0), "right_right", 90.0),
                4 => ((10.0, screen_height() / 2.0 + 35.0), "right_stright", 90.0),
                5 => ((10.0, screen_height() / 2.0 - 5.0), "right_left", 90.0),
                6 => ((screen_width() / 2.0 - 115.0, 5.0), "down_right", 180.0),
                7 => ((screen_width() / 2.0 - 75.0, 5.0), "down_stright", 180.0),
                8 => ((screen_width() / 2.0 - 35.0, 5.0), "down_left", 180.0),
                9 => ((screen_width() - 40.0, screen_height() / 2.0 - 125.0), "left_right", 270.0),
                10 => ((screen_width() - 40.0, screen_height() / 2.0 - 85.0), "left_stright", 270.0),
                _ => ((screen_width() - 40.0, screen_height() / 2.0 - 45.0), "left_left", 270.0),
            };

            if can_spawn(&cars, cord) {
                cars.push(Car::new(direction.to_string(), 30, 50, cord, rotation));
                stats.total_cars += 1;
            }
        }
        
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        // Minimum crawl speed (px/s) used when yielding instead of full stop
        let min_crawl = 30.0;
        // Distance under which we consider an imminent collision and allow a full stop
        let stop_threshold = 35.0;

        for i in 0..cars.len() {
            let mut requested_speed = cars[i].max_speed;
            let my_radar = cars[i].get_radar();
            let my_rect = cars[i].get_rect();
            let my_dist_center = ((cars[i].cord.0 - cx).powi(2) + (cars[i].cord.1 - cy).powi(2)).sqrt();

            for j in 0..cars.len() {
                if i == j { continue; }
                let other_rect = cars[j].get_rect();
                let other_radar = cars[j].get_radar();
                let other_dist_center = ((cars[j].cord.0 - cx).powi(2) + (cars[j].cord.1 - cy).powi(2)).sqrt();
                let dist_between = ((cars[i].cord.0 - cars[j].cord.0).powi(2) + (cars[i].cord.1 - cars[j].cord.1).powi(2)).sqrt();

                // 1. Avoid Rear-ending (Car physically inside my radar)
                if my_radar.intersect(other_rect).is_some() {
                    if dist_between < stop_threshold {
                        // Imminent collision: allow hard stop
                        requested_speed = 0.0;
                    } else if dist_between < 150.0 {
                        // Close: slow to a careful crawl
                        requested_speed = requested_speed.min(cars[i].max_speed * 0.20);
                    } else {
                        // Approaching: slow down but keep flow
                        requested_speed = requested_speed.min(cars[i].max_speed * 0.45);
                    }
                }

                // 2. Intersection Right-of-Way (Paths cross but haven't hit yet)
                if my_radar.intersect(other_radar).is_some() && my_rect.intersect(other_rect).is_none() {
                    let dist_diff = my_dist_center - other_dist_center;

                    // Cars already deep inside the intersection get de facto priority; we yield but don't fully stop
                    if other_dist_center < 130.0 && my_dist_center >= 130.0 {
                        requested_speed = requested_speed.min(cars[i].max_speed * 0.18);
                    }
                    // Whoever is closer to the center goes first; yield by crawling
                    else if dist_diff > 15.0 {
                        requested_speed = requested_speed.min(cars[i].max_speed * 0.18);
                    }
                    // Tie-breaker: prefer lower-index car but only slow to avoid deadlocks
                    else if dist_diff.abs() <= 15.0 && i > j {
                        requested_speed = requested_speed.min(cars[i].max_speed * 0.15);
                    }
                }
            }
            // Ensure we don't force a tiny nonzero speed; either full stop or a meaningful crawl
            if requested_speed > 0.0 && requested_speed < min_crawl {
                requested_speed = min_crawl.min(cars[i].max_speed);
            }

            cars[i].target_speed = requested_speed;
        }

        for car in cars.iter_mut() {
            car.update(dt);
            stats.sample_velocity(car.speed);
        }

        let current_time = get_time();
        cars.retain(|car| {
            let (x, y) = car.cord;
            let keep = x > -30.0 && x < screen_width() + 30.0 && y > -30.0 && y < screen_height() + 30.0;
            if !keep {
                stats.register_passed_car((current_time - car.spawn_time) as f32);
            }
            keep
        });

        for car in &cars {
            draw_texture_ex(
                &car_tex,
                car.cord.0,
                car.cord.1,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(car.width as f32, car.height as f32)),
                    rotation: car.rotation.to_radians(),
                    ..Default::default()
                },
            );
        }

        next_frame().await;
    }
}