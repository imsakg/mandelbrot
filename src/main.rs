use mandelbrot::{cli::Options, render, render_rgb, Image, ImageFormat};
use std::io::{self, Write};

const HELP: &str = "mandelbrot — terminal explorer and RGB image renderer\n\n\
Usage: mandelbrot [OPTIONS]\n\
  --width N --height N     Pixel or character dimensions (default 100 × 30)\n\
  --iterations N          Updates per orbit (1..=10000000)\n\
  --bounds X0,X1,Y0,Y1     Ordered complex-plane rectangle\n\
  --zoom FACTOR --pan X,Y  Ordered viewport transforms\n\
  --julia RE,IM           Julia set with a fixed complex constant\n\
  --variant NAME          mandelbrot or ship\n\
  --power N --radius R    Multibrot power (2..=8), escape radius (>=2)\n\
  --palette NAME          ocean, fire, gray, viridis\n\
  --cycles N --shift N    Cyclic palette frequency and offset\n\
  --gamma N --histogram   Tone correction / histogram equalization\n\
  --no-smooth --samples N Integer bands / n×n supersampling (1..=4)\n\
  --output PATH, -o PATH  Save .png or .ppm; never overwrite existing files\n\
  --help, -h              Show this help\n";

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args_os()
        .skip(1)
        .map(|v| v.into_string().map_err(|_| "arguments must be UTF-8"))
        .collect::<Result<Vec<_>, _>>()?;
    let options = Options::parse(args)?;
    if options.help {
        io::stdout().lock().write_all(HELP.as_bytes())?;
        return Ok(());
    }
    if let Some(path) = &options.output {
        let pixels = render_rgb(&options.render, &options.colors, options.samples)?;
        let image = Image::new(options.render.width, options.render.height, pixels)?;
        image.save(path, ImageFormat::from_path(path)?)?;
    } else {
        io::stdout()
            .lock()
            .write_all(render(&options.render)?.ascii().as_bytes())?;
    }
    Ok(())
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error)
            if error.downcast_ref::<io::Error>().map(|e| e.kind())
                == Some(io::ErrorKind::BrokenPipe) =>
        {
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("mandelbrot: {error}");
            std::process::ExitCode::from(2)
        }
    }
}
