use mandelbrot::{cli::Options, render_parallel, render_rgb_parallel, Image, ImageFormat};
use std::io::{self, IsTerminal, Write};

struct TerminalSession;
impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = io::stdout().write_all(b"\x1b[?25h\x1b[?1049l");
    }
}
fn interactive(options: Options) -> Result<(), Box<dyn std::error::Error>> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("interactive mode requires an input and output terminal".into());
    }
    io::stdout().write_all(b"\x1b[?1049h\x1b[?25l")?;
    let _session = TerminalSession;
    let input = getch::Getch::new();
    let mut explorer = mandelbrot::Explorer::new(options)?;
    loop {
        let options = explorer.options();
        let frame = render_parallel(&options.render, options.threads)?;
        {
            let mut out = io::stdout().lock();
            write!(out,"\x1b[H{}\x1b[Kiterations={}  {:?}\n\x1b[Kwasd pan | k/j zoom | [] detail | r reset | u/y undo/redo | p palette | q quit\n",frame.ascii(),options.render.iterations.max_iterations,options.render.iterations.fractal)?;
            out.flush()?;
        }
        match explorer.key(input.getch()? as char) {
            Ok(mandelbrot::controls::Action::Quit) => break,
            Ok(_) => {}
            Err(error) => {
                eprintln!("{error}");
            }
        }
    }
    Ok(())
}

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
  --interactive          Keyboard explorer (requires a TTY)\n\
  --threads N            Scoped CPU workers (1..=64)\n\
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
    if options.interactive {
        return interactive(options);
    }
    if let Some(path) = &options.output {
        let pixels = render_rgb_parallel(
            &options.render,
            &options.colors,
            options.samples,
            options.threads,
        )?;
        let image = Image::new(options.render.width, options.render.height, pixels)?;
        image.save(path, ImageFormat::from_path(path)?)?;
    } else {
        io::stdout().lock().write_all(
            render_parallel(&options.render, options.threads)?
                .ascii()
                .as_bytes(),
        )?;
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
