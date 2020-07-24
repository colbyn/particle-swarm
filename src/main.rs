#![allow(unused)]

use nannou::prelude::*;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use std::convert::From;
use std::collections::HashMap;
use std::iter::FromIterator;

///////////////////////////////////////////////////////////////////////////////
// SCENE
///////////////////////////////////////////////////////////////////////////////

fn to_new_position(x: f32, y: f32, a: f32) -> (f32, f32) {
    let new_x = (a.cos() * x) - (a.sin() * y);
    let new_y = (a.sin() * x) + (a.cos() * y);
    
    (new_x, new_y)
}

#[derive(Debug, Clone)]
pub struct V2 {
    x: f32,
    y: f32,
}

impl std::fmt::Display for V2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}


impl V2 {
    pub fn add_mut(&mut self, other: &V2) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
    pub fn sub_mut(&mut self, other: &V2) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
    pub fn mul_mut(&mut self, n: f32) {
        self.x = self.x * n;
        self.y = self.y * n;
    }
    pub fn div_mut(&mut self, n: f32) {
        self.x = self.x / n;
        self.y = self.y / n;
    }
    pub fn magnitude(&self) -> f32 {
        let x_2 = (self.x * self.x);
        let y_2 = (self.y * self.y);
        (x_2 + y_2).sqrt()
    }
    pub fn normalize_mut(&mut self){
        let mag = self.magnitude();
        if mag != 0.0 {
            self.div_mut(mag);
        }
    }
    pub fn hypt(&self) -> f32 {
        let x_2 = self.x * self.x;
        let y_2 = self.y * self.y;
        (x_2 + y_2).sqrt()
    }
}


#[derive(Debug, Clone)]
pub struct Particle {
    uid: String,
    location: V2,
    velocity: V2,
    color: (u8, u8, u8),
}

impl Particle {
    pub fn tick(&mut self, others: &[&Particle]) {
        self.location.add_mut(&self.velocity);
        if self.location.x.abs() >= 100.0 {
            self.velocity.x = self.velocity.x * -1.0;
        }
        if self.location.y.abs() > 100.0 {
            self.velocity.y = self.velocity.y * -1.0;
        }
        for other in others {
            let within_range = |v1: &V2, v2: &V2| -> bool {
                let x_diff = (v1.x - v2.x).abs();
                let y_diff = (v1.y - v2.y).abs();
                let is_within_range = {
                    x_diff <= 2.5 && y_diff <= 2.5
                };
                is_within_range
            };
            if within_range(&self.location, &other.location) {
                self.velocity.x = self.velocity.x * -1.0;
                self.velocity.y = self.velocity.y * -1.0;
                self.location.add_mut(&self.velocity);
            }
        }
    }
    pub fn position(&self) -> (f32, f32) {
        (self.location.x, self.location.y)
    }
}

///////////////////////////////////////////////////////////////////////////////
// MODEL
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Model {
    particles: HashMap<String, Particle>,
    last_tick: Instant,
}

impl Model {
    pub fn new() -> Self {
        // RANDOM
        use rand::{Rng, thread_rng};
        use rand::distributions::Uniform;
        let mut rng = thread_rng();
        let position_uni = Uniform::new(-100.0, 100.0);
        let angle_uni = Uniform::new(0.0, 2.0 * PI);
        // SETUP COLORS
        use colourado::{Color, ColorPalette, PaletteType};
        // GENERATE PARTICLES
        let length: usize = 100;
        let palette = ColorPalette::new(
            length as u32,
            PaletteType::Random,
            false
        );
        let particles = (0..length)
            .into_iter()
            .map(|ix| {
                let (x, y) = (
                    rng.sample(position_uni),
                    rng.sample(position_uni)
                );
                let angle = rng.sample(angle_uni);
                let new_color = || {
                    fn convert(x: f32) -> u8 {
                        (x * 255.0) as u8
                    }
                    // FINALIZE
                    let red = convert(palette.colors[ix].red);
                    let green = convert(palette.colors[ix].green);
                    let blue = convert(palette.colors[ix].blue);
                    (red, green, blue)
                };
                let particle = Particle {
                    uid: format!(
                        "{}-{}",
                        rand::random::<u64>(),
                        rand::random::<u64>(),
                    ),
                    location: V2 {x, y},
                    velocity: V2 {x: 1.5, y: 1.5},
                    color: new_color(),
                };
                (particle.uid.clone(), particle)
            })
            .collect::<Vec<_>>();
        // // SETUP 
        // let entries = (0 .. 360)
        //     .filter(|x| x % 15 == 0)
        //     .enumerate()
        //     .collect::<Vec<_>>();
        // // SETUP COLORS
        // let palette = ColorPalette::new(
        //     entries.len() as u32,
        //     PaletteType::Random,
        //     false
        // );
        // // CREATE
        // let mut particles = Vec::new();
        // for (ix, angle) in entries {
        //     let angle_rad = (angle as f32).to_radians();
        //     let position = to_new_position(
        //         30.0,
        //         30.0,
        //         angle_rad,
        //     );
        //     let new_color = || {
        //         fn convert(x: f32) -> u8 {
        //             (x * 255.0) as u8
        //         }
        //         // FINALIZE
        //         let red = convert(palette.colors[ix].red);
        //         let green = convert(palette.colors[ix].green);
        //         let blue = convert(palette.colors[ix].blue);
        //         (red, green, blue)
        //     };
        //     let particle = Particle {
        //         uid: format!(
        //             "{}-{}",
        //             rand::random::<u64>(),
        //             rand::random::<u64>(),
        //         ),
        //         location: V2 {x: position.0, y: position.1},
        //         velocity: V2 {x: 1.5, y: 1.5},
        //         color: new_color(),
        //     };
        //     particles.push((particle.uid.clone(), particle));
        // }
        // DONE
        let last_tick = Instant::now();
        Model {particles: HashMap::from_iter(particles), last_tick}
    }
    
    pub fn tick(&mut self) {
        // MOVE
        let keys = {
            self.particles
                .keys()
                .map(|x| x.clone()).collect::<Vec<_>>()
        };
        let others = self.particles.clone();
        for pid in keys {
            let particle = self.particles.get_mut(&pid).unwrap();
            let others = others
                .values()
                .filter(|x| x.uid != particle.uid)
                .collect::<Vec<_>>();
            particle.tick(&others);
        }
        // FINALIZE
        self.last_tick = Instant::now();
    }
}


fn update_model(_app: &App, model: &mut Model, ts: Update) {
    use std::convert::From;
    let current = std::time::Instant::now().elapsed();
    let diff = model.last_tick.elapsed() - current;
    if diff.secs() >= 0.1 {
        model.tick();
        model.last_tick = Instant::now();
    }
}



///////////////////////////////////////////////////////////////////////////////
// RENDERER
///////////////////////////////////////////////////////////////////////////////

fn new_linear_scale(
    domain: (f32, f32),
    codomain: (f32, f32)
) -> impl Fn(f32)->f32 {
    let (min_input, max_input) = domain;
    let (min_output, max_output) = codomain;
    move |value: f32| -> f32 {
        let result = (
            (max_output - min_output)
            * (value - min_input)
            / (max_input - min_input)
            + min_output
        );
        result
    }
}



fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    let win = app.window_rect();
    let width_scale = new_linear_scale(
        (-100.0, 100.0),
        (-win.w() / 2.0, win.w() / 2.0)
    );
    let height_scale = new_linear_scale(
        (-100.0, 100.0),
        (-win.h() / 2.0, win.h() / 2.0)
    );

    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    let draw_circle = |x: f32, y: f32, color: Option<(u8, u8, u8)>| {
        let shape_color = {
            color
                .map(|x| {
                    nannou::color::Rgb::new(
                        x.0,
                        x.1,
                        x.2,
                    )
                })
                .unwrap_or(nannou::color::Rgb::new(255, 0, 0))
        };
        draw.ellipse()
            .x_y(width_scale(x), height_scale(y))
            .radius(6.0)
            .color(shape_color);
    };

    for particle in model.particles.values() {
        let (x, y) = particle.position();
        draw_circle(x, y, Some(particle.color.clone())); 
    }

    // draw_circle(0.0, 0.0);

    // let new_position = |x: f32, y: f32, a: f32|{
    //     let new_x = (a.cos() * x) - (a.sin() * y);
    //     let new_y = (a.sin() * x) + (a.cos() * y);
        
    //     (new_x, new_y)
    // };

    // for angle in (0 .. 361).filter(|x| x % 30 == 0) {
    //     let angle_rad = (angle as f32).to_radians();
    //     let position = new_position(30.0, 30.0, angle_rad);
    //     draw_circle(position.0, position.1);
    // }

    // for angle in (0 .. 361).filter(|x| x % 30 == 0) {
    //     let angle_rad = (angle as f32).to_radians();
    //     let position = new_position(40.0, 40.0, angle_rad);
    //     draw_circle(position.0, position.1);
    // }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn init_model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();
    Model::new()
}

///////////////////////////////////////////////////////////////////////////////
// MAIN
///////////////////////////////////////////////////////////////////////////////

fn main() {
    nannou::app(init_model)
        .update(update_model)
        .run();
}