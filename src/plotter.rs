//Utility for plotting R^2 -> C functions (represented as Vec<Vec<Complex64>>)
//using plotters under the hood

use std::fs;
use std::path::Path;
use crate::complex::*;
use plotters::prelude::*;

#[derive(Copy, Clone)]
pub enum Interpolation{
    Nearest, Linear
}

pub struct PlotConfig<'a>{
    pub filepath: &'a str,
    pub width: usize,
    pub height: usize,
    pub interpolation: Interpolation,
}

pub fn plot(data: &Vec<Vec<Complex64>>, config: &PlotConfig) {
    let path = Path::new(config.filepath);
    
    match path.parent() {
        None => (),
        Some(directory) => {
            fs::create_dir_all(directory).unwrap();
        },
    }

    let root = BitMapBackend::new(
        config.filepath, 
        (config.width as u32, config.height as u32)
    ).into_drawing_area();
    
    let width = config.width as f64;
    let height = config.height as f64;

    let chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0.0..width, 0.0..height)
        .unwrap();

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();

    let (pw, ph) = (
        (range.0.end - range.0.start) as usize,
        (range.1.end - range.1.start) as usize
    );

    for idx in 0..pw {
        for idy in 0..ph {
            let interp_config = InteprpolationConfig{
                from_x: config.width,
                from_y: config.height,
                to_x: data.len(),
                to_y: data[0].len(),
                interpolation: config.interpolation,
            };

            let value = interpolated_value(idx, idy, &interp_config, &data);

            plotting_area.draw_pixel((idx as f64, idy as f64), &complex64_to_color(value)).unwrap();
        }
    }
}

struct InteprpolationConfig{
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
    interpolation: Interpolation,
}

fn interpolated_value(idx: usize, idy: usize, 
        config: &InteprpolationConfig, data: &Vec<Vec<Complex64>>) -> Complex64 {

    let x = (config.to_x as f64) * (idx as f64)/(config.from_x as f64);
    let y = (config.to_y as f64) * (idy as f64)/(config.from_y as f64);

    let i = f64::floor(x) as usize;
    let j = f64::floor(y) as usize;

    let right = if i+1 < data.len()    {i+1} else {i};
    let up    = if j+1 < data[0].len() {j+1} else {j}; 

    let a = data[i][j];
    let b = data[right][j];
    let c = data[i][up];
    let d = data[right][up];

    let lerp = |a: Complex64, b: Complex64, t: f64| -> Complex64 {
        if t < 0.0 || t > 1.0 {
            panic!("Interpolation parameter = {}, it should be from [0,1]", t);
        }

        a*(1.0-t) + t*b
    };

    let nearest = |(tx, ty): (f64, f64)| -> (f64, f64){
        if tx < 0.0 || tx > 1.0 {
            panic!("Interpolation parameters = ({},{}), should be from [0,1]", tx, ty);
        }

        (f64::floor(tx + 0.5), f64::floor(tx + 0.5))
    };

    let (u_x, u_y) = match config.interpolation {
        Interpolation::Nearest => {
            nearest((f64::fract(x), f64::fract(y)))
        },
        Interpolation::Linear => {
            (f64::fract(x), f64::fract(y))
        },
    };

    lerp(lerp(a, b, u_x), lerp(c, d, u_x), u_y)
}

fn complex64_to_color(value: Complex64) -> RGBColor {
    let hue = rad_to_deg(value.arg() + std::f64::consts::PI);
    let saturation = 0.5;
    let value = value.r();

    //Normalzied float rgb
    let (r, g, b) = hsv_to_rgb(hue, saturation, value);

    //Conv to u8 rgb
    let r = (256.0 * r) as u8;
    let g = (256.0 * g) as u8;
    let b = (256.0 * b) as u8;

    RGBColor(r,g,b)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let c = v * s;
    let h1 = if h != 360.0 {h / 60.0} else {0.0};
    let x = c * (1.0 - f64::abs((h1 % 2.0) - 1.0));

    let sixth_num = f64::floor(h1) as u32;

    let (r, g, b) = match sixth_num {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    let m = v - c;

    (r + m, g + m, b + m)
}

fn rad_to_deg(val: f64) -> f64 {
    360.0*val/(2.0 * std::f64::consts::PI)
}