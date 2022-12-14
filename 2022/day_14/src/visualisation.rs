use sfml::graphics::{Color, FloatRect, RectangleShape, RenderTarget, RenderWindow, Shape};
use sfml::system::Clock;
use sfml::window::{Event, Key, Style};

use crate::{Map, TileType};

const SEGMENT_SIZE: f32 = 4.0;
const FRAME_RATE_CAP: i32 = 120;

const CAMERA_MIN_Y: usize = 0;
const CAMERA_MAX_Y: usize = 160;
const CAMERA_MIN_X: usize = 330;
const CAMERA_MAX_X: usize = 670;

pub fn run(map: &Map) {
    let frames = create_frames(&map);

    println!("Creating window..");
    let mut window = RenderWindow::new(
        (
            (CAMERA_MAX_X - CAMERA_MIN_X + 1) as u32 * SEGMENT_SIZE as u32,
            (CAMERA_MAX_Y - CAMERA_MIN_Y + 1) as u32 * SEGMENT_SIZE as u32,
        ),
        "AoC Sand",
        Style::CLOSE,
        &Default::default(),
    );

    let mut clock = Clock::start();
    let mut current_frame = 0;
    let mut time_buf = 0;
    let delta_time_cap = 1000 / FRAME_RATE_CAP;
    println!("Starting animation..");
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        window.clear(Color::BLACK);

        let delta_time = clock.restart().as_milliseconds();
        time_buf += delta_time;
        if time_buf > delta_time_cap {
            time_buf = 0;
            current_frame = (current_frame + 1) % frames.len();
        }

        for shape in frames[current_frame].iter() {
            window.draw(shape);
        }

        window.display();
    }
}

fn create_frames(map: &Map) -> Vec<Vec<RectangleShape>> {
    println!("Creating frames..");
    map.snapshots
        .iter()
        .map(|tiles| {
            let mut rects: Vec<RectangleShape> = Vec::new();
            for y in CAMERA_MIN_Y..CAMERA_MAX_Y {
                for x in (CAMERA_MIN_X + map.x_shift)..(CAMERA_MAX_X + map.x_shift) {
                    let idx = map.idx(x as i32, y as i32);
                    match tiles[idx] {
                        TileType::Air => {}
                        TileType::Wall => {
                            let mut rect = RectangleShape::from_rect(FloatRect::new(
                                (x as f32 - CAMERA_MIN_X as f32 - map.x_shift as f32)
                                    * SEGMENT_SIZE,
                                (y as f32 - CAMERA_MIN_Y as f32) * SEGMENT_SIZE,
                                SEGMENT_SIZE,
                                SEGMENT_SIZE,
                            ));
                            rect.set_fill_color(Color::GREEN);
                            rects.push(rect);
                        }
                        TileType::Sand => {
                            let mut rect = RectangleShape::from_rect(FloatRect::new(
                                (x as f32 - CAMERA_MIN_X as f32 - map.x_shift as f32)
                                    * SEGMENT_SIZE,
                                (y as f32 - CAMERA_MIN_Y as f32) * SEGMENT_SIZE,
                                SEGMENT_SIZE,
                                SEGMENT_SIZE,
                            ));
                            rect.set_fill_color(Color::YELLOW);
                            rects.push(rect);
                        }
                    }
                }
            }
            rects
        })
        .collect::<Vec<Vec<RectangleShape>>>()
}
