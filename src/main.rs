use raylib::{color::Color, prelude::RaylibDraw};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Crusty")
        .build();
    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::PURPLE);
        draw.draw_text("Hello Crusty", 350, 300, 20, Color::WHITE);
    }
}
