extern crate sdl2;
use fractal_generator::generator_logic;
use fractal_generator::rend::Renderer;
use fractal_generator::WINDOW_HEIGHT;
use fractal_generator::WINDOW_WIDTH;

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

    let vertex_coords = fractal_generator::calculate_vertices(3);
    let renderer = Renderer::new(window, &vertex_coords)?;
    generator_logic::generate(sdl_context, renderer, vertex_coords)?;
    Ok(())
}
