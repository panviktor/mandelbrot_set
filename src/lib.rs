use std::process;
use num::complex::Complex;

pub struct Mandelbrot {
    pub max_iters: usize,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub width: usize,
    pub height: usize
}

impl Mandelbrot {
    pub fn new_from_args(max_iters: &str,
                             x_min: &str,
                             x_max: &str,
                             y_min: &str,
                             y_max: &str,
                             width: &str,
                             height: &str) -> Result<Mandelbrot, &'static str> {

        let max_iters: usize = max_iters.parse().unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        let x_min: f64 = x_min.parse().unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        let x_max: f64 = x_max.parse().unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        let y_min: f64 = y_min.parse().unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        let y_max: f64 = y_max.parse().unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        let width: usize = width.parse().unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
        let height: usize = height.parse().unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        Ok(Mandelbrot { max_iters, x_min, x_max, y_min, y_max, width, height})
    }

    pub fn calculate_mandelbrot(self) -> Vec<Vec<usize>> {
        let mut all_rows: Vec<Vec<usize>> = Vec::with_capacity(self.width);
        for img_y in 0..self.height {
            let mut row: Vec<usize> = Vec::with_capacity(self.height);
            for img_x in 0..self.width {
                let cx = self.x_min + (self.x_max - self.x_min) * (img_x as f64 / self.width as f64);
                let cy = self.y_min + (self.y_max - self.y_min) * (img_y as f64 / self.height as f64);
                let escaped_at = Mandelbrot::mandelbrot_at_point(cx, cy, self.max_iters);
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
         max_iters
    }

    pub fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
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
}

