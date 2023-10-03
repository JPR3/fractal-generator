extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::collections::HashSet;
pub struct Renderer {
    canvas: WindowCanvas,
    vertex_points: Vec<Point>,
    fractal_points: HashSet<Point>,
}
impl Renderer {
    pub fn new(window: Window, vertex_coords: &Vec<(i32, i32)>) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let mut vertex_points: Vec<Point> = vec![];
        for coords in vertex_coords {
            vertex_points.push(Point::from(*coords));
        }

        Ok(Renderer {
            canvas,
            vertex_points,
            fractal_points: HashSet::new(),
        })
    }
    pub fn draw(&mut self, point: (i32, i32)) -> Result<(), String> {
        self.draw_bgnd()?;
        self.canvas.set_draw_color(Color::RGB(245, 245, 245));
        self.fractal_points.insert(Point::from(point));

        for p in self.fractal_points.iter() {
            self.canvas.draw_point(*p)?;
        }
        self.canvas.present();
        Ok(())
    }
    fn draw_bgnd(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(19, 19, 19));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(245, 245, 245));
        for i in 0..self.vertex_points.len() - 1 {
            self.canvas
                .draw_line(self.vertex_points[i], self.vertex_points[i + 1])?;
        }
        self.canvas.draw_line(
            self.vertex_points[0],
            self.vertex_points[self.vertex_points.len() - 1],
        )?;
        Ok(())
    }
}
