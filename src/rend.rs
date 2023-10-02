extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
// use std::collections::HashSet;
pub struct Renderer {
    canvas: WindowCanvas,
    border_points: Vec<Point>,
}
impl Renderer {
    pub fn new(window: Window, border_coords: Vec<(i32, i32)>) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let mut border_points: Vec<Point> = vec![];
        for coords in border_coords {
            border_points.push(Point::new(coords.0, coords.1));
        }

        Ok(Renderer {
            canvas,
            border_points,
        })
    }
    pub fn draw(&mut self) -> Result<(), String> {
        self.draw_bgnd()?;
        self.canvas.present();
        Ok(())
    }
    fn draw_bgnd(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(19, 19, 19));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(245, 245, 245));
        for i in 0..self.border_points.len() - 1 {
            self.canvas.draw_line(
                self.border_points.get(i).unwrap().clone(),
                self.border_points.get(i + 1).unwrap().clone(),
            )?;
        }
        self.canvas.draw_line(
            self.border_points.get(0).unwrap().clone(),
            self.border_points
                .get(self.border_points.len() - 1)
                .unwrap()
                .clone(),
        )?;
        Ok(())
    }
}
