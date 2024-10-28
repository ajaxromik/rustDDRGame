// Example 1: The Square
// Open a window, and draw a colored square in it
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Image},
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Canvas Painter",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Clear the screen to a blank, white color
    gfx.clear(Color::WHITE);
    // Paint a blue square with a red outline in the center of our screen
    // It should have a top-left of (350, 100) and a size of (150, 100)
    let image = Image::load(&gfx, "thwomp.png").await?;
    let image_rect = Rectangle::new(Vector::new(80.0,120.0), image.size());
    gfx.draw_image(&image, image_rect);
    gfx.present(&window)?;

    let rect = Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0));
    gfx.fill_rect(&rect, Color::BLUE);
    gfx.stroke_rect(&rect, Color::RED);
    // Send the data to be drawn
    gfx.present(&window)?;

    loop {
        while let Some(_) = input.next_event().await {}
    }
}