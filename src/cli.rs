//! CLI parsing is pure and testable without launching a terminal.
use crate::{Error, RenderSettings};

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
fn value(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, Error> {
    args.next()
        .ok_or_else(|| Error(format!("missing value for {flag}")))
}
