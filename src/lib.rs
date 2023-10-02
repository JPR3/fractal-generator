use std::f32::consts::PI;
pub mod generator_logic;
pub mod rend;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 450;
const RADIUS: i32 = 200;

pub fn calculate_points(n: i32) -> Vec<(i32, i32)> {
    let interval: f32 = 2.0 * PI / n as f32;
    let mut points: Vec<(i32, i32)> = Vec::new();
    for angle in 0..n {
        let radian_angle = match n % 2 {
            0 => angle as f32 * interval + (PI * 3.0 / 2.0) + PI / (n as f32),
            _ => angle as f32 * interval + (PI * 3.0 / 2.0),
        };
        points.push((
            ((f32::cos(radian_angle)) * RADIUS as f32) as i32 + WINDOW_WIDTH as i32 / 2,
            ((f32::sin(radian_angle)) * RADIUS as f32) as i32 + WINDOW_HEIGHT as i32 / 2,
        ));
    }
    points
}
