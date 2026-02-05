use macroquad::prelude::*;
mod cars;
mod dashed;
mod draw_road;
mod lights;
use cars::*;
use draw_road::*;
use lights::*;

// Count cars in each lane/direction
fn count_cars_per_lane(cars: &Vec<Car>) -> (usize, usize, usize, usize) {
    let mut up_count = 0;
    let mut down_count = 0;
    let mut left_count = 0;
    let mut right_count = 0;

    for car in cars {
        match car.direction.as_str() {
            "up" => {
                up_count += 1;
            }
            "down" => {
                down_count += 1;
            }
            "left" => {
                left_count += 1;
            }
            "right" => {
                right_count += 1;
            }
            _ => {}
        }
    }

    (up_count, down_count, left_count, right_count)
}

fn is_intersection_occupied(cars: &Vec<Car>) -> bool {
    let w = screen_width();
    let h = screen_height();
    let cx = w / 2.0;
    let cy = h / 2.0;
    let gap = 60.0;

    let left = cx - gap;
    let right = cx + gap;
    let top = cy - gap;
    let bottom = cy + gap;

    for car in cars {
        let (x, y) = car.cord;
        // Check if car center is inside the box
        if x > left && x < right && y > top && y < bottom {
            return true;
        }
    }
    false
}
// Calculate lane capacity based on project requirements
fn calculate_lane_capacity() -> usize {
    let lane_length = 400.0_f32;
    let vehicle_length = 30.0_f32;
    let safety_gap = 50.0_f32;

    (lane_length / (vehicle_length + safety_gap)).floor() as usize
}

fn window_conf() -> Conf {
    Conf {
        window_title: "road_intersection".to_string(),
        window_width: 900,
        window_height: 800,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

// Helper to check if it's safe to spawn a new car
fn can_spawn(cars: &Vec<Car>, direction: &str, spawn_cord: (f32, f32)) -> bool {
    let safe_dist = 60.0;
    for car in cars {
        if car.direction == direction {
            // Simple distance check
            let dist = (
                (car.cord.0 - spawn_cord.0).powi(2) + (car.cord.1 - spawn_cord.1).powi(2)
            ).sqrt();
            if dist < safe_dist {
                return false;
            }
        }
    }
    true
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cars: Vec<Car> = Vec::new();
    let mut traffic_light = TrafficLight::new();

    let safety_gap = 50.0;

    let lane_capacity = calculate_lane_capacity();

    loop {
        let dt = get_frame_time();

        let (up_count, down_count, left_count, right_count) = count_cars_per_lane(&cars);

        let intersection_clear = !is_intersection_occupied(&cars);

        traffic_light.update_with_congestion(
            dt,
            up_count,
            down_count,
            left_count,
            right_count,
            lane_capacity,
            intersection_clear
        );

        clear_background(Color::from_rgba(4, 96, 85, 255));
        draw_road();
        traffic_light.draw_lights();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Backspace) {
            cars.clear();
        }

        if is_key_pressed(KeyCode::Up) {
            let cord = (screen_width() / 2.0 + 15.0, screen_height() - 35.0);
            if can_spawn(&cars, "up", cord) {
                cars.push(Car::new("up".to_string(), 30, 30, cord, rand::gen_range(1, 4)));
            }
        }

        if is_key_pressed(KeyCode::Right) {
            let cord = (10.0, screen_height() / 2.0 + 15.0);
            if can_spawn(&cars, "right", cord) {
                cars.push(Car::new("right".to_string(), 30, 30, cord, rand::gen_range(1, 4)));
            }
        }

        if is_key_pressed(KeyCode::Down) {
            let cord = (screen_width() / 2.0 - 45.0, 10.0);
            if can_spawn(&cars, "down", cord) {
                cars.push(Car::new("down".to_string(), 30, 30, cord, rand::gen_range(1, 4)));
            }
        }

        if is_key_pressed(KeyCode::Left) {
            let cord = (screen_width() - 35.0, screen_height() / 2.0 - 45.0);
            if can_spawn(&cars, "left", cord) {
                cars.push(Car::new("left".to_string(), 30, 30, cord, rand::gen_range(1, 4)));
            }
        }

        if is_key_pressed(KeyCode::R) {
            let random_dir = rand::gen_range(0, 4);
            let (direction, cord) = match random_dir {
                0 => ("up", (screen_width() / 2.0 + 15.0, screen_height() - 35.0)),
                1 => ("down", (screen_width() / 2.0 - 45.0, 10.0)),
                2 => ("left", (screen_width() - 35.0, screen_height() / 2.0 - 45.0)),
                _ => ("right", (10.0, screen_height() / 2.0 + 15.0)),
            };

            if can_spawn(&cars, direction, cord) {
                cars.push(Car::new(direction.to_string(), 30, 30, cord, rand::gen_range(1, 4)));
            }
        }

        for i in 0..cars.len() {
            let mut blocked = false;
            let my_cord = cars[i].cord;
            let my_dir = cars[i].direction.clone();

            for j in 0..cars.len() {
                if i == j {
                    continue;
                }

                let other = &cars[j];

                if my_dir == other.direction {
                    let other_cord = other.cord;

                    // Calculate distance
                    let dist = (
                        (my_cord.0 - other_cord.0).powi(2) + (my_cord.1 - other_cord.1).powi(2)
                    ).sqrt();

                    if dist < safety_gap {
                        let is_ahead = match my_dir.as_str() {
                            "up" => other_cord.1 < my_cord.1,
                            "down" => other_cord.1 > my_cord.1,
                            "left" => other_cord.0 < my_cord.0,
                            "right" => other_cord.0 > my_cord.0,
                            _ => false,
                        };

                        if is_ahead {
                            blocked = true;
                            break;
                        }
                    }
                }
            }

            cars[i].update(dt, traffic_light.get_state(), blocked);
        }

        cars = cars
            .iter()
            .filter(|car| {
                let (x, y) = car.cord;
                x > -30.0 && x < screen_width() + 30.0 && y > -30.0 && y < screen_height() + 30.0
            })
            .cloned()
            .collect();

        for car in &cars {
            draw_rectangle(car.cord.0, car.cord.1, car.width as f32, car.height as f32, car.color);
        }

        next_frame().await;
    }
}
