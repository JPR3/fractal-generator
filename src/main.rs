extern crate sdl2;
use fractal_generator::generator_logic;
use fractal_generator::rend::Renderer;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 450;
//TODO accept argument
pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Fractal", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let renderer = Renderer::new(window)?;
    generator_logic::generate(sdl_context, renderer)?;
    Ok(())
}
