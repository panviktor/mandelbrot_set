use num::complex::Complex;
use clap::{App, Arg};

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize
) -> Vec<Vec<usize>> {
    let mut all_rows: Vec<Vec<usize>> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let cx = x_min + (x_max - x_min) * (img_x as f64 / width as f64);
            let cy = y_min + (y_max - y_min) * (img_y as f64 / height as f64);
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }
        all_rows.push(row);
    }
    all_rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    return  max_iters;
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                2..=5 => '.',
                5..=10 => '•',
                11..=30 => '*',
                30..=100 => '+',
                100..=200 => 'x',
                200..=400 => '$',
                400..=700 => '#',
                _ => '%'
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let args = App::new("mandelbrot_set")
        .version("0.2.0")
        .author("SwiftViktor <panafviktor@gmail.com>")
        .about(
            "A simple console application written in Rust to display Mandelbrot set.")

        .arg(Arg::new("max_iters")
            .about("If a value has not 'escaped' before reaching the maximum number of iterations,\n
             it is considered to be within the Mandelbrot set")
            .takes_value(true)
            .required(true))

        .arg(Arg::new("x_min")
            .about("These four parameters specify the space we’re searching for to look for members of the set")
            .takes_value(true)
            .required(true))

        .arg(Arg::new("x_max")
            .about("TThese four parameters specify the space we’re searching for to look for members of the set")
            .takes_value(true)
            .required(true))

        .arg(Arg::new("y_min")
            .about("These four parameters specify the space we’re searching for to look for members of the set")
            .takes_value(true)
            .required(true))


        .arg(Arg::new("y_max")
            .about("These four parameters specify the space we’re searching for to look for members of the set")
            .takes_value(true)
            .required(true))

        .arg(Arg::new("width")
            //.help("These two parameters represent the width of the output in 'pixels'")
            .takes_value(true)
            .required(true))

        .arg(Arg::new("height")
            .about("These two parameters represent the height of the output in 'pixels'")
            .takes_value(true)
            .required(true))

        .get_matches();


    let max_iters = args.value_of("max_iters").unwrap();
    let max_iters_usize: usize = max_iters.parse().unwrap();

    let x_min = args.value_of("x_min").unwrap();
    let x_min_f64: f64 = x_min.parse().unwrap();

    let x_max = args.value_of("x_max").unwrap();
    let x_max_f64: f64 = x_max.parse().unwrap();

    let y_min = args.value_of("y_min").unwrap();
    let y_min_f64: f64 = y_min.parse().unwrap();

    let y_max = args.value_of("y_max").unwrap();
    let y_max_f64: f64 = y_max.parse().unwrap();

    let width = args.value_of("width").unwrap();
    let width_usize: usize = width.parse().unwrap();

    let height = args.value_of("height").unwrap();
    let height_usize: usize = height.parse().unwrap();

    let mandelbrot = calculate_mandelbrot(max_iters_usize,
                                                        x_min_f64, x_max_f64,
                                                        y_min_f64, y_max_f64,
                                                         width_usize, height_usize,
    );
    render_mandelbrot(mandelbrot);
}