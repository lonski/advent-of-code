use std::collections::HashSet;

use sfml::graphics::{Color, FloatRect, RectangleShape, RenderTarget, RenderWindow, Shape};
use sfml::system::Clock;
use sfml::window::{Event, Key, Style};

const SEGMENT_SIZE: f32 = 4.0;
const FRAME_RATE_CAP: i32 = 120;

pub fn run(snapshots: Vec<Vec<(i32, i32)>>) {
    let (min_x, max_x, min_y, max_y) = extract_boundaries(&snapshots);
    let frames = create_frames(&snapshots, min_x, min_y);

    let mut window = RenderWindow::new(
        (
            (max_x + min_x.abs() + 1) as u32 * SEGMENT_SIZE as u32,
            (max_y + min_y.abs() + 1) as u32 * SEGMENT_SIZE as u32,
        ),
        "AoC Rope",
        Style::CLOSE,
        &Default::default(),
    );

    let mut clock = Clock::start();
    let mut current_frame = 0;
    let mut time_buf = 0;
    let delta_time_cap = 1000 / FRAME_RATE_CAP;
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

fn extract_boundaries(snapshots: &Vec<Vec<(i32, i32)>>) -> (i32, i32, i32, i32) {
    let maxes = snapshots
        .iter()
        .map(|c| {
            (
                c.iter().map(|&(x, _)| x).max().unwrap(),
                c.iter().map(|&(x, _)| x).min().unwrap(),
                c.iter().map(|&(_, y)| y).max().unwrap(),
                c.iter().map(|&(_, y)| y).min().unwrap(),
            )
        })
        .collect::<HashSet<(i32, i32, i32, i32)>>();

    let max_x = maxes.iter().map(|&(x, _, _, _)| x).max().unwrap();
    let min_x = maxes.iter().map(|&(_, x, _, _)| x).min().unwrap();
    let max_y = maxes.iter().map(|&(_, _, y, _)| y).max().unwrap();
    let min_y = maxes.iter().map(|&(_, _, _, y)| y).min().unwrap();

    (min_x, max_x, min_y, max_y)
}

fn create_frames(
    snapshots: &Vec<Vec<(i32, i32)>>,
    min_x: i32,
    min_y: i32,
) -> Vec<Vec<RectangleShape>> {
    snapshots
        .iter()
        .map(|state| {
            state
                .iter()
                .map(|&(x, y)| {
                    let mut rect = RectangleShape::from_rect(FloatRect::new(
                        (min_x.abs() as f32 + x as f32) * SEGMENT_SIZE,
                        (min_y.abs() as f32 + y as f32) * SEGMENT_SIZE,
                        SEGMENT_SIZE,
                        SEGMENT_SIZE,
                    ));
                    rect.set_fill_color(Color::GREEN);
                    rect
                })
                .collect::<Vec<RectangleShape>>()
        })
        .collect::<Vec<Vec<RectangleShape>>>()
}
