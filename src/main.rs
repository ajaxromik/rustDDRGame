// Example 1: The Square
// Open a window, and draw a colored square in it
use quicksilver::{
    geom::{Rectangle, Circle, Vector},
    graphics::{Color, Element, Mesh, Vertex, Image, PixelFormat, Surface},
    run, Graphics, Input, Result, Settings, Window, Timer
};

const SCREEN_HEIGHT: f32 = 500.0;
const SCREEN_WIDTH: f32 = 800.0;

fn main() {
    run(
        Settings {
            title: "Canvas Painter",
            size: Vector::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {

    let mut update_timer = Timer::time_per_second(30.0);
    let mut draw_timer = Timer::time_per_second(60.0);

     // Create a surface, which allows rendering to an image
    let mut surface = Surface::new(
        &gfx,
        Image::from_raw(&gfx, None, 512, 512, PixelFormat::RGBA)?,
    )?;
    // Draw a circle inside a rectangle
    gfx.fill_rect(
        &Rectangle::new(Vector::new(0.0, 0.0), Vector::new(100.0, 100.0)),
        Color::RED,
    );
    gfx.fill_circle(&Circle::new(Vector::new(400.0, 150.0), 50.0), Color::BLACK);
    // Flush to the surface, which draws to the image
    gfx.flush_surface(&surface)?;

    let image = surface.detach().expect("The image failed to detach");
    

    // Clear the screen to a blank, white color
    gfx.clear(Color::WHITE);

    // Draw that image to the screen twice
    gfx.draw_image(&image, Rectangle::new_sized(Vector::new(400.0, 300.0)));
    gfx.draw_image(
        &image,
        Rectangle::new(Vector::new(400.0, 300.0), Vector::new(400.0, 300.0)),
    );

    // let image = Image::load(&gfx, "thwomp.png").await?; // load and draw image
    // let image_rect = Rectangle::new(Vector::new(80.0,120.0), image.size());
    // gfx.draw_image(&image, image_rect);

    let vertices = {
        let top = Vertex {
            pos: Vector::new(110.0, 300.0),
            uv: None,
            color: Color::BLACK,
        };
        let left = Vertex { 
            pos: Vector::new(480.0, 460.0),
            uv: None,
            color: Color::INDIGO,
        };
        let right = Vertex {
            pos: Vector::new(650.0, 250.0),
            uv: None,
            color: Color::BLUE,
        };
        vec![top, left, right]
    };

    let elements = vec![Element::Triangle([0,1,2])]; // not really sure what the point of the element is..

    let mesh = Mesh {
        vertices,
        elements,
        image: None,
        // image: Some(image), // no idea why the image wouldnt show but it has something to do with the vertex code above
    };

    gfx.draw_mesh(&mesh);

    let mut rect = Rectangle::new(Vector::new(250.0, 100.0), Vector::new(100.0, 100.0)); // first vector is pos, second is size
    let mut circle = Circle::new(Vector::new(400.0,150.0), 50.0);
    // gfx.fill_rect(&rect, Color::BLUE); // fill inside
    // gfx.stroke_rect(&rect, Color::RED); // outline
    // Send the data to be drawn
    // gfx.present(&window)?;

    loop {
        while let Some(_) = input.next_event().await {}

        while update_timer.tick() {
            //to scroll the box top to bottom, runs it from a y position of -y to the screen height
            rect.pos.y = ((5.0+rect.pos.y+rect.size.y) % (SCREEN_HEIGHT+rect.size.y)) - rect.size.y;
            // println!("{}", rect.pos.y);
            circle.pos.y = ((5.0+circle.pos.y+circle.radius) % (SCREEN_HEIGHT+circle.radius*2.0)) - circle.radius;
        }

        if draw_timer.exhaust().is_some() {
            gfx.clear(Color::WHITE);

            gfx.fill_rect(&rect, Color::BLUE);
            // gfx.stroke_rect(&rect, Color::RED);

            gfx.fill_circle(&circle, Color::RED);

            gfx.present(&window)?;
        }

    }
}