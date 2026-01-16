use colorgrad::Gradient;
use owo_colors::{OwoColorize, Rgb};

use crate::presentation::http::v1::constants::LOGO;

pub fn show_brand_logo() {
    let lines: Vec<&str> = LOGO.lines().collect();
    let gradient = colorgrad::GradientBuilder::new()
        .colors(&[
            colorgrad::Color::from_rgba8(255, 105, 180, 255),
            colorgrad::Color::from_rgba8(199, 21, 133, 255),
            colorgrad::Color::from_rgba8(138, 43, 226, 255),
            colorgrad::Color::from_rgba8(75, 0, 130, 255),
        ])
        .build::<colorgrad::LinearGradient>()
        .unwrap();

    let colorful_logo = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let t = i as f32 / lines.len().max(1) as f32;
            let color = gradient.at(t).to_rgba8();
            line.color(Rgb(color[0], color[1], color[2])).to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", colorful_logo);
}
