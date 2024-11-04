// Example 1: The Square
// Open a window, and draw a colored square in it
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Element, Mesh, Vertex, Image, PixelFormat, Surface},
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Canvas Painter",
            size: Vector::new(800.0, 500.0),
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Clear the screen to a blank, white color
    gfx.clear(Color::WHITE);

    let image = Image::load(&gfx, "thwomp.png").await?;
    let image_rect = Rectangle::new(Vector::new(80.0,120.0), image.size());
    gfx.draw_image(&image, image_rect);

    // Paint a blue square with a red outline in the center of our screen
    // It should have a top-left of (350, 100) and a size of (150, 100)

    /*let vertices = {
        let top = Vertex {
            pos: Vector::new(400.0, 200.0),
            uv: None,
            color: Color::RED,
        };
        let left = Vertex {
            pos: Vector::new(200.0, 400.0),
            uv: None,
            color: Color::GREEN,
        };
        let right = Vertex {
            pos: Vector::new(600.0, 400.0),
            uv: None,
            color: Color::BLUE,
        };
        vec![top, left, right]
    };
    // A triangle is simply a pointer to indices of the vertices
    let elements = vec![Element::Triangle([0, 1, 2])];
    // Bring the vertices and the triangle elements together to define a mesh
    let mesh = Mesh {
        vertices,
        elements,
        image: None,
    };
    // Pass a reference to the mesh to the graphics object to draw
    gfx.draw_mesh(&mesh);*/

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

    let rect = Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0));
    gfx.fill_rect(&rect, Color::BLUE);
    gfx.stroke_rect(&rect, Color::RED);
    // Send the data to be drawn
    gfx.present(&window)?;

    loop {
        while let Some(_) = input.next_event().await {}
    }
}