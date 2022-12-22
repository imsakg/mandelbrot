//! CLI parsing is pure and testable without launching a terminal.
use crate::{Error, RenderSettings, Viewport};

#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    pub render: RenderSettings,
    pub help: bool,
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
        };
        let mut args = args.into_iter().map(Into::into);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => out.help = true,
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
        out.render.validate()?;
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
