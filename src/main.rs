// Example 1: The Square
// Open a window, and draw a colored square in it
use quicksilver::{
    geom::{Rectangle, Circle, Vector},
    graphics::{Color, VectorFont},
    input::{Event, Key},
    run, Graphics, Input, Result, Settings, Window, Timer
};
use rand::Rng;

/**
 * Allows controlling an equilateral triangle from the center of its position
 * (not equilateral but close enough who cares)
 */
struct ETriangle {
    pos: Vector, // center of triangle
    hheight: f32  // half the base/height of equilateral triangle
}

impl ETriangle {
    fn new(pos: Vector, hheight: f32) -> ETriangle {
        ETriangle {
            pos,
            hheight
        }
    }

    fn get_vertices(&self) -> [Vector; 3] {
        [
            Vector::new(self.pos.x, self.pos.y-self.hheight),
            Vector::new(self.pos.x-self.hheight, self.pos.y+self.hheight),
            Vector::new(self.pos.x+self.hheight, self.pos.y+self.hheight)
        ]
    }
}

/**
 * Struct to handle a diamond as an object that can be moved by the center.
 */
struct Diamond {
    pos: Vector,
    hheight: f32 // half the total diamond height
}

impl Diamond {
    fn new(pos: Vector, hheight: f32) -> Diamond {
        Diamond {
            pos,
            hheight
        }
    }

    fn get_vertices(&self) -> [Vector; 4] { // these need to stay clockwise or counterclockwise
        [
            Vector::new(self.pos.x, self.pos.y-self.hheight),
            Vector::new(self.pos.x - self.hheight/2.0, self.pos.y),
            Vector::new(self.pos.x, self.pos.y+self.hheight),
            Vector::new(self.pos.x + self.hheight/2.0, self.pos.y)
        ]
    }
}

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

    let mut update_timer = Timer::time_per_second(60.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let mut rand = rand::thread_rng();

    let ttf = VectorFont::load("font.ttf").await?; // stolen from examples
    let mut font = ttf.to_renderer(&gfx, 30.0)?;

    let mut score = 0.0;

    let mut rect = Rectangle::new(Vector::new(80.0, 100.0), Vector::new(100.0, 100.0)); // first vector is pos, second is size
    let mut circle = Circle::new(Vector::new(240.0,150.0), 50.0);
    let mut triangle = ETriangle::new(Vector::new(350.0,150.0), 50.0);
    let mut diamond = Diamond::new(Vector::new(460.0,150.0), 50.0);

    let target_rect = Rectangle::new(Vector::new(80.0, 325.0), Vector::new(100.0, 100.0));
    let target_circle = Circle::new(Vector::new(240.0,375.0), 50.0); // hah not the rewards kind
    let target_triangle = ETriangle::new(Vector::new(350.0,375.0), 50.0);
    let target_diamond = Diamond::new(Vector::new(460.0,375.0), 50.0);

    let barrier_rect = Rectangle::new(Vector::new(515.0, 0.0), Vector::new(4.0, SCREEN_HEIGHT));

    // strange way to do mut but compiler said i had to
    let (mut rect_delay, mut circle_delay, mut triangle_delay, mut diamond_delay) = (
        rand.gen_range(0..=25), rand.gen_range(0..=25), rand.gen_range(0..=25), rand.gen_range(0..=25)
    );

    // i could make speed changing, but it would break all the mathematical expressions i'm using for position
    let speed = 5.0;
    let mut max_score = 50.0;
    let mut selected = 0;

    // println!("{target_circle:?}");

    loop {

        // start up menu selection
        loop {
            while let Some(_) = input.next_event().await {}

            if input.key_down(Key::W) || input.key_down(Key::Up) {
                selected = 0;
            } else if input.key_down(Key::S) || input.key_down(Key::Down) {
                selected = 1;
            } else if input.key_down(Key::Space) || input.key_down(Key::Return) {
                if selected == 1 {
                    max_score = f32::MAX;
                }
                break;
            }

            gfx.clear(Color::from_rgba(32,44,89,1.0));

            gfx.fill_rect(&Rectangle::new(Vector::new(250.0, 175.0), Vector::new(300.0, 135.0)), Color::from_rgba(156,13,56,1.0));
            font.draw(
                &mut gfx,
                &format!("Choose your mode:"),
                Color::BLACK,
                Vector::new(267.5, 210.0),
            )?;

            gfx.fill_rect(&Rectangle::new(Vector::new(329.5, 263.0), Vector::new(141.0, -36.0)), Color::from_rgba(111,255,233,1.0));
            font.draw(
                &mut gfx,
                &format!("Regular"),
                Color::BLACK,
                Vector::new(344.5, 255.0),
            )?;

            gfx.fill_rect(&Rectangle::new(Vector::new(329.5, 303.0), Vector::new(141.0, -36.0)), Color::from_rgba(91,192,190,1.0));
            font.draw(
                &mut gfx,
                &format!("Endless"),
                Color::BLACK,
                Vector::new(344.5, 295.0),
            )?;

            gfx.fill_circle(&Circle::new(Vector::new(318.0,244.0+(selected as f32)*40.0), 7.5), Color::BLACK);

            gfx.present(&window)?;
        }

        // actual game
        while score < max_score {

            while let Some(event) = input.next_event().await {
                // format!("{event:?}");
                match event {
                    Event::KeyboardInput(key) if key.is_down() => {
                        if key.key() == Key::D { // rect
                            let distance = rect.pos.y - target_rect.pos.y;
                            if (0.0..15.0).contains(&distance.abs()) { // cool way to check ranges
                                score += 2.5;
                                // println!("score: {score}");
                            } else {
                                score -= 0.5;
                            }
                        } else if key.key() == Key::F { // circle
                            let distance = circle.pos.y - target_circle.pos.y;
                            if (0.0..15.0).contains(&distance.abs()) { // cool way to check ranges
                                score += 2.5;
                                // println!("score: {score}");
                            } else {
                                score -= 0.5;
                            }
                        } else if key.key() == Key::J { // triangle
                            let distance = triangle.pos.y - target_triangle.pos.y;
                            if (0.0..15.0).contains(&distance.abs()) { // cool way to check ranges
                                score += 2.5;
                                // println!("score: {score}");
                            } else {
                                score -= 0.5;
                            }
                        } else if key.key() == Key::K { // diamond
                            let distance = diamond.pos.y - target_diamond.pos.y;
                            if (0.0..15.0).contains(&distance.abs()) { // cool way to check ranges
                                score += 2.5;
                                // println!("score: {score}");
                            } else {
                                score -= 0.5;
                            }
                        }
                    }
                    _ => (),
                }
            }

            while update_timer.tick() {
                // println!("{}", circle.pos.y);
                //to scroll the box top to bottom, runs it from a y position of -y to the screen height
                // also makes it wait a random amount of time
                if rect.pos.y == -100.0 {
                    rect_delay -= 1;
                    if rect_delay <= 0 {
                        rect_delay = rand.gen_range(5..=30);
                        rect.pos.y += speed;
                    }
                } else {
                    rect.pos.y = ((speed+rect.pos.y+rect.size.y) % (SCREEN_HEIGHT+rect.size.y)) - rect.size.y;
                }

                if circle.pos.y == -50.0 {
                    circle_delay -= 1;
                    if circle_delay <= 0 {
                        circle_delay = rand.gen_range(5..=30);
                        circle.pos.y += speed;
                    }
                } else {
                    circle.pos.y = ((speed+circle.pos.y+circle.radius) % (SCREEN_HEIGHT+circle.radius*2.0)) - circle.radius;
                }

                if triangle.pos.y == -50.0 {
                    triangle_delay -= 1;
                    if triangle_delay <= 0 {
                        triangle_delay = rand.gen_range(5..=30);
                        triangle.pos.y += speed;
                    }
                } else {
                    triangle.pos.y = ((speed+triangle.pos.y+triangle.hheight) % (SCREEN_HEIGHT+triangle.hheight*2.0)) - triangle.hheight;
                }

                if diamond.pos.y == -50.0 {
                    diamond_delay -= 1;
                    if diamond_delay <= 0 {
                        diamond_delay = rand.gen_range(5..=30);
                        diamond.pos.y += speed;
                    }
                } else {
                    diamond.pos.y = ((speed+diamond.pos.y+diamond.hheight) % (SCREEN_HEIGHT+diamond.hheight*2.0)) - diamond.hheight;
                }

                // circle.pos.y = ((speed+circle.pos.y+circle.radius) % (SCREEN_HEIGHT+circle.radius*2.0)) - circle.radius;
                // triangle.pos.y = ((speed+triangle.pos.y+triangle.hheight) % (SCREEN_HEIGHT+triangle.hheight*2.0)) - triangle.hheight;
                // diamond.pos.y = ((speed+diamond.pos.y+diamond.hheight) % (SCREEN_HEIGHT+diamond.hheight*2.0)) - diamond.hheight;
            }

            if draw_timer.exhaust().is_some() {

                gfx.clear(Color::WHITE);

                gfx.fill_rect(&barrier_rect, Color::BLACK);

                font.draw(
                    &mut gfx,
                    &format!("Score: {score}"),
                    Color::BLACK,
                    Vector::new(530.0, 100.0),
                )?;

                font.draw_wrapping(
                    &mut gfx,
                    "Press the d, f, j, and k keys when the shapes enter their outlines. Keys match their respective shapes in order from left to right.",
                    Some(250.0),
                    Color::BLACK,
                    Vector::new(530.0, 150.0),
                )?;

                gfx.stroke_rect(&target_rect, Color::BLACK);
                gfx.stroke_circle(&target_circle, Color::BLACK);
                gfx.stroke_polygon(&target_triangle.get_vertices(), Color::BLACK);
                gfx.stroke_polygon(&target_diamond.get_vertices(), Color::BLACK);

                gfx.fill_rect(&rect, Color::BLUE);
                // gfx.stroke_rect(&rect, Color::RED);

                gfx.fill_circle(&circle, Color::RED);
                gfx.fill_polygon(&triangle.get_vertices(), Color::INDIGO);
                gfx.fill_polygon(&diamond.get_vertices(), Color::ORANGE);

                gfx.present(&window)?;
            }

        }

        // for winning screen after 50 points

        let mut exit_timer = Timer::time_per_second(0.2);

        score = 0.0;
        gfx.clear(Color::WHITE);
        font.draw(
            &mut gfx,
            &format!("You win, hooray!"),
            Color::BLACK,
            Vector::new(285.0, 215.0),
        )?;
        gfx.present(&window)?;

        //trying to wait 5 seconds before going back to selection screen
        loop { if exit_timer.tick() {break;} }
            
    }

}