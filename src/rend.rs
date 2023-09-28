extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
pub struct Renderer {
    canvas: WindowCanvas,
}
impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }
    pub fn draw(&mut self) -> Result<(), String> {
        self.draw_bgnd()?;
        self.canvas.present();
        Ok(())
    }
    fn draw_bgnd(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(19, 19, 19));
        self.canvas.clear();
        // self.canvas.set_draw_color(Color::RGB(245, 245, 245));
        // self.canvas.draw_point(Point::new(10, 10))?;
        Ok(())
    }
}
