#![allow(unused)]


use raylib::prelude::*;

fn main() {
    let mut width = 640;
    let mut height = 480;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Closure plots")
        .build();

    let mut camera = Camera2D {
        offset: Vector2 { x: width as f32/2.0, y: height as f32/2.0 },
        target: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        width = d.get_screen_width();
        height = d.get_screen_height();
        // camera.offset = Vector2 { x: width as f32/2.0, y: height as f32/2.0 };

        d.draw_rectangle(10, 10, 50, 40, Color::BLACK);
        
        d.clear_background(Color::WHITE);
    }

    todo!()
}
