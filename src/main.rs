#![windows_subsystem = "windows"]
extern crate sdl2;
use fractal_generator::generator_logic;
use fractal_generator::rend::Renderer;
use fractal_generator::WINDOW_HEIGHT;
use fractal_generator::WINDOW_WIDTH;
use std::env;
//TODO accept argument
pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let sides = match args.len() {
        1 => 3,
        2 => match args[1].parse::<i32>() {
            Ok(n) => n,
            Err(_) => panic!("Invlaid argument - enter an integer value"),
        },
        _ => panic!("Invlaid arguments - enter one integer value"),
    };
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Fractal", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let vertex_coords = fractal_generator::calculate_vertices(sides);
    let renderer = Renderer::new(window, &vertex_coords)?;
    generator_logic::generate(sdl_context, renderer, vertex_coords)?;
    Ok(())
}
