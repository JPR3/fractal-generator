extern crate sdl2;
use crate::calc_new_point;
use crate::rend::Renderer;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;
use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn generate(
    sdl_context: sdl2::Sdl,
    mut renderer: Renderer,
    vertex_coords: Vec<(i32, i32)>,
) -> Result<(), String> {
    let mut event_pump = sdl_context.event_pump()?;
    let mut previous_point: (i32, i32) = (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
    let mut rng = thread_rng();
    let mut previous_vertex_ind = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let current_vertex_ind = match vertex_coords.len() > 3 {
            true => exclusive_rand(&mut rng, vertex_coords.len() - 1, previous_vertex_ind),
            false => rng.gen_range(0..vertex_coords.len()),
        };

        let new_point = calc_new_point(previous_point, vertex_coords[current_vertex_ind]);
        previous_vertex_ind = current_vertex_ind;
        previous_point = new_point;
        renderer.draw(new_point)?;
        std::thread::sleep(Duration::from_secs_f32(1.0 / 240.0));
    }
    Ok(())
}

fn exclusive_rand(rng: &mut ThreadRng, bound: usize, exclude: usize) -> usize {
    let mut num = rng.gen_range(0..bound);
    if num >= exclude {
        num += 1;
    }
    num as usize
}
