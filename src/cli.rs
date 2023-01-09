//! CLI parsing is pure and testable without launching a terminal.
use crate::{ColorSettings, Error, Fractal, Palette, RenderSettings, Viewport};

#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    pub render: RenderSettings,
    pub help: bool,
    pub colors: ColorSettings,
    pub samples: u8,
    pub threads: usize,
    pub output: Option<std::path::PathBuf>,
}
impl Options {
    pub fn parse<I, S>(args: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut out = Self {
            render: RenderSettings {
                width: 100,
                height: 30,
                ..Default::default()
            },
            help: false,
            colors: ColorSettings::default(),
            samples: 1,
            threads: 1,
            output: None,
        };
        let mut args = args.into_iter().map(Into::into);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => out.help = true,
                "--threads" => {
                    out.threads = value(&mut args, &arg)?
                        .parse()
                        .map_err(|_| Error("threads must be an integer".into()))?
                }
                "--output" | "-o" => out.output = Some(value(&mut args, &arg)?.into()),
                "--palette" => out.colors.palette = Palette::parse(&value(&mut args, &arg)?)?,
                "--cycles" => out.colors.cycles = number(&value(&mut args, &arg)?)?,
                "--shift" => out.colors.shift = number(&value(&mut args, &arg)?)?,
                "--gamma" => out.colors.gamma = number(&value(&mut args, &arg)?)?,
                "--histogram" => out.colors.histogram = true,
                "--no-smooth" => out.colors.smooth = false,
                "--samples" => {
                    out.samples = value(&mut args, &arg)?
                        .parse()
                        .map_err(|_| Error("samples must be an integer".into()))?
                }
                "--variant" => out.render.iterations.fractal = match value(&mut args, &arg)?
                    .as_str()
                {
                    "mandelbrot" => Fractal::Mandelbrot,
                    "ship" => Fractal::BurningShip,
                    _ => return Err(Error(
                        "variant must be mandelbrot or ship; use --julia or --power for other sets"
                            .into(),
                    )),
                },
                "--julia" => {
                    let c = numbers::<2>(&value(&mut args, &arg)?)?;
                    out.render.iterations.fractal = Fractal::Julia { re: c[0], im: c[1] };
                }
                "--power" => {
                    out.render.iterations.fractal = Fractal::Multibrot {
                        power: value(&mut args, &arg)?
                            .parse()
                            .map_err(|_| Error("power must be an integer".into()))?,
                    }
                }
                "--radius" => {
                    out.render.iterations.escape_radius = number(&value(&mut args, &arg)?)?
                }
                "--bounds" => {
                    let b = numbers::<4>(&value(&mut args, &arg)?)?;
                    out.render.viewport = Viewport::new(b[0], b[1], b[2], b[3])?;
                }
                "--zoom" => {
                    let factor = number(&value(&mut args, &arg)?)?;
                    out.render.viewport = out.render.viewport.zoom(factor)?;
                }
                "--pan" => {
                    let p = numbers::<2>(&value(&mut args, &arg)?)?;
                    out.render.viewport = out.render.viewport.pan(p[0], p[1])?;
                }
                "--width" => {
                    out.render.width = value(&mut args, &arg)?
                        .parse()
                        .map_err(|_| Error("width must be an integer".into()))?
                }
                "--height" => {
                    out.render.height = value(&mut args, &arg)?
                        .parse()
                        .map_err(|_| Error("height must be an integer".into()))?
                }
                "--iterations" => {
                    out.render.iterations.max_iterations = value(&mut args, &arg)?
                        .parse()
                        .map_err(|_| Error("iterations must be an integer".into()))?
                }
                _ => return Err(Error(format!("unknown option: {arg}; use --help"))),
            }
        }
        if let Some(path) = &out.output {
            crate::ImageFormat::from_path(path)?;
        }
        out.render.validate()?;
        out.colors.validate()?;
        if !(1..=64).contains(&out.threads) {
            return Err(Error("threads must be in 1..=64".into()));
        }
        if !(1..=4).contains(&out.samples) {
            return Err(Error("samples must be in 1..=4".into()));
        }
        Ok(out)
    }
}
pub(crate) fn number(s: &str) -> Result<f64, Error> {
    s.parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
        .ok_or_else(|| Error(format!("expected a finite number: {s}")))
}
fn numbers<const N: usize>(s: &str) -> Result<[f64; N], Error> {
    let values: Vec<f64> = s.split(',').map(number).collect::<Result<_, _>>()?;
    values
        .try_into()
        .map_err(|_| Error(format!("expected {N} comma-separated numbers")))
}

fn value(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, Error> {
    args.next()
        .ok_or_else(|| Error(format!("missing value for {flag}")))
}
