use macroquad::prelude::*;
mod cars;
mod dashed;
mod draw_road;
use cars::*;
use draw_road::*;

// Count cars in each lane/direction
// fn count_cars_per_lane(cars: &Vec<Car>) -> (usize, usize, usize, usize) {
//     let mut up_count = 0;
//     let mut down_count = 0;
//     let mut left_count = 0;
//     let mut right_count = 0;

//     for car in cars {
//         match car.direction.as_str() {
//             "up" => {
//                 up_count += 1;
//             }
//             "down" => {
//                 down_count += 1;
//             }
//             "left" => {
//                 left_count += 1;
//             }
//             "right" => {
//                 right_count += 1;
//             }
//             _ => {}
//         }
//     }

//     (up_count, down_count, left_count, right_count)
// }

fn window_conf() -> Conf {
    Conf {
        window_title: "road_intersection".to_string(),
        window_width: 1000,
        window_height: 800,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

fn can_spawn(cars: &Vec<Car>, direction: &str, spawn_cord: (f32, f32)) -> bool {
    let safe_dist = 60.0;
    for car in cars {
        if car.direction == direction {
            let dist =
                ((car.cord.0 - spawn_cord.0).powi(2) + (car.cord.1 - spawn_cord.1).powi(2)).sqrt();
            if dist < safe_dist {
                return false;
            }
        }
    }
    true
}

#[macroquad::main(window_conf)]
async fn main() {
    let car_tex: Texture2D = load_texture("assets/car.png").await.unwrap();
    car_tex.set_filter(FilterMode::Nearest);
    let mut cars: Vec<Car> = Vec::new();

    let safety_gap = 50.0;

    loop {
        let dt = get_frame_time();

        clear_background(Color::from_rgba(4, 96, 85, 255));
        draw_road();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Backspace) {
            cars.clear();
        }

        if is_key_pressed(KeyCode::Up) {
            let random_case = rand::gen_range(0, 3);
            let (cord, direction) = match random_case {
                0 => (
                    (screen_width() / 2.0 + 85.0, screen_height() - 55.0),
                    "up_right",
                ),
                1 => (
                    (screen_width() / 2.0 + 45.0, screen_height() - 55.0),
                    "up_stright",
                ),
                _ => (
                    (screen_width() / 2.0 + 5.0, screen_height() - 55.0),
                    "up_left",
                ),
            };
            if can_spawn(&cars, direction, cord) {
                cars.push(Car::new(direction.to_string(), 30, 50, cord, 0, 0.0));
            }
        }

        if is_key_pressed(KeyCode::Right) {
            let random_case = rand::gen_range(0, 3);
            let (cord, direction) = match random_case {
                0 => ((10.0, screen_height() / 2.0 + 75.0), "right_right"),
                1 => ((10.0, screen_height() / 2.0 + 35.0), "right_stright"),
                _ => ((10.0, screen_height() / 2.0 - 5.0), "right_left"),
            };
            if can_spawn(&cars, direction, cord) {
                cars.push(Car::new(direction.to_string(), 30, 50, cord, 0, 90.0));
            }
        }

        if is_key_pressed(KeyCode::Down) {
            let random_case = rand::gen_range(0, 3);
            let (cord, direction) = match random_case {
                0 => ((screen_width() / 2.0 - 115.0, 5.0), "down_right"),
                1 => ((screen_width() / 2.0 - 75.0, 5.0), "down_stright"),
                _ => ((screen_width() / 2.0 - 35.0, 5.0), "down_left"),
            };
            if can_spawn(&cars, direction, cord) {
                cars.push(Car::new(direction.to_string(), 30, 50, cord, 0, 180.0));
            }
        }

        if is_key_pressed(KeyCode::Left) {
            let random_case = rand::gen_range(0, 3);
            let (cord, direction) = match random_case {
                0 => (
                    (screen_width() - 40.0, screen_height() / 2.0 - 125.0),
                    "left_right",
                ),
                1 => (
                    (screen_width() - 40.0, screen_height() / 2.0 - 85.0),
                    "left_stright",
                ),
                _ => (
                    (screen_width() - 40.0, screen_height() / 2.0 - 45.0),
                    "left_left",
                ),
            };
            if can_spawn(&cars, direction, cord) {
                cars.push(Car::new(direction.to_string(), 30, 50, cord, 0, 270.0));
            }
        }

        if is_key_pressed(KeyCode::R) {
            let random_dir = rand::gen_range(0, 12);
            let (cord, direction, rotation) = match random_dir {
                0 => (
                    (screen_width() / 2.0 + 85.0, screen_height() - 55.0),
                    "up_right",
                    0.0,
                ),
                1 => (
                    (screen_width() / 2.0 + 45.0, screen_height() - 55.0),
                    "up_stright",
                    0.0,
                ),
                2 => (
                    (screen_width() / 2.0 + 5.0, screen_height() - 55.0),
                    "up_left",
                    0.0,
                ),
                3 => ((10.0, screen_height() / 2.0 + 75.0), "right_right", 90.0),
                4 => ((10.0, screen_height() / 2.0 + 35.0), "right_stright", 90.0),
                5 => ((10.0, screen_height() / 2.0 - 5.0), "right_left", 90.0),
                6 => ((screen_width() / 2.0 - 115.0, 5.0), "down_right", 180.0),
                7 => ((screen_width() / 2.0 - 75.0, 5.0), "down_stright", 180.0),
                8 => ((screen_width() / 2.0 - 35.0, 5.0), "down_left", 180.0),
                9 => (
                    (screen_width() - 40.0, screen_height() / 2.0 - 125.0),
                    "left_right",
                    270.0,
                ),
                10 => (
                    (screen_width() - 40.0, screen_height() / 2.0 - 85.0),
                    "left_stright",
                    270.0,
                ),
                _ => (
                    (screen_width() - 40.0, screen_height() / 2.0 - 45.0),
                    "left_left",
                    270.0,
                ),
            };

            if can_spawn(&cars, direction, cord) {
                cars.push(Car::new(direction.to_string(), 30, 50, cord, 0, rotation));
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

                    let dist = ((my_cord.0 - other_cord.0).powi(2)
                        + (my_cord.1 - other_cord.1).powi(2))
                    .sqrt();

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

            cars[i].update(dt, blocked);
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
