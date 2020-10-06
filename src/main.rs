use clap::{App, Arg};
use mandelbrot::Mandelbrot;
use std::process;

fn main() {
    let args = App::new("mandelbrot_set")
        .version("0.3.0")
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
    let x_min = args.value_of("x_min").unwrap();
    let x_max = args.value_of("x_max").unwrap();
    let y_min = args.value_of("y_min").unwrap();
    let y_max = args.value_of("y_max").unwrap();
    let width = args.value_of("width").unwrap();
    let height = args.value_of("height").unwrap();

    let mandelbrot = Mandelbrot::new_from_args(max_iters, x_min, x_max, y_min, y_max, width, height).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    Mandelbrot::render_mandelbrot(Mandelbrot::calculate_mandelbrot(mandelbrot));
}