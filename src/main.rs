use mandelbrot::{cli::Options, render_parallel, render_rgb_parallel, Image, ImageFormat};
use std::io::{self, IsTerminal, Read, Write};

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
            write!(out,"\x1b[H{}\x1b[Kiterations={}  {:?}\n\x1b[Kwasd pan | k/j zoom | [] detail | r reset | u/y undo/redo | p palette | q quit\n",if options.color { frame.ansi(&options.colors)? } else { frame.ascii() },options.render.iterations.max_iterations,options.render.iterations.fractal)?;
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
  --preset NAME          overview, seahorse, elephant, julia, ship\n\
  --julia RE,IM           Julia set with a fixed complex constant\n\
  --variant NAME          mandelbrot or ship\n\
  --power N --radius R    Multibrot power (2..=8), escape radius (>=2)\n\
  --palette NAME          ocean, fire, gray, viridis\n\
  --interior HEX         Six-digit RGB for bounded samples\n\
  --cycles N --shift N    Cyclic palette frequency and offset\n\
  --gamma N --histogram   Tone correction / histogram equalization\n\
  --no-smooth --samples N Integer bands / n×n supersampling (1..=4)\n\
  --output PATH, -o PATH  Save .png or .ppm; never overwrite existing files\n\
  --print-recipe         Print a portable render recipe, then exit\n\
  --recipe PATH          Load whitespace-separated render settings (no shell)\n\
  --stats                Batch elapsed time and settings on stderr\n\
  --color                Opt-in 24-bit terminal color cells\n\
  --interactive          Keyboard explorer (requires a TTY)\n\
  --threads N            Scoped CPU workers (1..=64)\n\
  --help, -h              Show this help\n";

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args_os()
        .skip(1)
        .map(|v| v.into_string().map_err(|_| "arguments must be UTF-8"))
        .collect::<Result<Vec<_>, _>>()?;
    let mut expanded = Vec::new();
    let mut input = args.into_iter();
    while let Some(arg) = input.next() {
        if arg == "--recipe" {
            let path = input.next().ok_or("missing path for --recipe")?;
            let mut text = String::new();
            std::fs::File::open(path)?
                .take(65_537)
                .read_to_string(&mut text)?;
            if text.len() > 65_536 {
                return Err("recipe exceeds 64 KiB".into());
            }
            let tokens: Vec<String> = text.split_whitespace().map(str::to_owned).collect();
            if tokens
                .iter()
                .any(|v| ["--recipe", "--output", "-o", "--interactive"].contains(&v.as_str()))
            {
                return Err("recipe files may not request file I/O or interactive mode".into());
            }
            expanded.extend(tokens);
        } else {
            expanded.push(arg);
        }
    }
    let options = Options::parse(expanded)?;
    if options.help {
        io::stdout().lock().write_all(HELP.as_bytes())?;
        return Ok(());
    }
    if options.print_recipe {
        writeln!(io::stdout().lock(), "{}", options.recipe())?;
        return Ok(());
    }
    let started = std::time::Instant::now();
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
            {
                let frame = render_parallel(&options.render, options.threads)?;
                if options.color {
                    frame.ansi(&options.colors)?
                } else {
                    frame.ascii()
                }
            }
            .as_bytes(),
        )?;
    }
    if options.stats {
        eprintln!(
            "elapsed_seconds={:.6} size={}x{} iterations={} samples_per_axis={} workers={}",
            started.elapsed().as_secs_f64(),
            options.render.width,
            options.render.height,
            options.render.iterations.max_iterations,
            options.samples,
            options.threads
        );
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
