use getch::Getch;
use num::complex::Complex;

struct Params {
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
    scale: f64,
}

fn calculate_mandelbrot(params: &Params) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(params.width);
    for img_y in 0..params.height {
        let mut row: Vec<usize> = Vec::with_capacity(params.height);
        for img_x in 0..params.width {
            let x_percent = (img_x as f64 / params.width as f64) * params.scale;
            let y_percent = (img_y as f64 / params.height as f64) * params.scale;
            let cx = params.x_min + (params.x_max - params.x_min) * x_percent;
            let cy = params.y_min + (params.y_max - params.y_min) * y_percent;

            let escaped_at = mandelbrot_at_point(cx, cy, params.max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(escape_values: Vec<Vec<usize>>) {
    for row in escape_values {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let input = Getch::new();
    let mut params = Params {
        max_iters: 1000,
        x_min: -2.,
        x_max: 1.,
        y_min: -1.,
        y_max: 1.,
        width: 100,
        height: 24,
        scale: 1.,
    };
    loop {
        print!("\x1B[2J\x1B[1;1H"); // clear screen
        let mandelbrot = calculate_mandelbrot(&params);
        render_mandelbrot(mandelbrot);

        match input.getch().unwrap() as char {
            'q' => break,
            'w' => {
                params.y_max -= 0.1;
                params.y_min -= 0.1;
            }
            's' => {
                params.y_min += 0.1;
                params.y_max += 0.1;
            }
            'a' => {
                params.x_max -= 0.1;
                params.x_min -= 0.1;
            }
            'd' => {
                params.x_min += 0.1;
                params.x_max += 0.1;
            }
            'j' => {
                params.scale += 0.1;
            }
            'k' => {
                params.scale -= 0.1;
            }
            _ => {}
        }
    }
}
