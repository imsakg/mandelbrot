use crate::{
    cli::Options,
    controls::{self, Action},
    Error, Navigation, Palette,
};

pub struct Explorer {
    options: Options,
    history: Navigation,
}
impl Explorer {
    pub fn new(options: Options) -> Result<Self, Error> {
        options.render.validate()?;
        let history = Navigation::new(options.render.viewport, 100)?;
        Ok(Self { options, history })
    }
    pub fn options(&self) -> &Options {
        &self.options
    }
    pub fn key(&mut self, key: char) -> Result<Action, Error> {
        match key {
            'u' => {
                if let Some(view) = self.history.undo() {
                    self.options.render.viewport = view;
                }
                return Ok(Action::Redraw);
            }
            'y' => {
                if let Some(view) = self.history.redo() {
                    self.options.render.viewport = view;
                }
                return Ok(Action::Redraw);
            }
            'p' => {
                let current = Palette::ALL
                    .iter()
                    .position(|p| *p == self.options.colors.palette)
                    .unwrap_or(0);
                self.options.colors.palette = Palette::ALL[(current + 1) % Palette::ALL.len()];
                return Ok(Action::Redraw);
            }
            _ => {}
        }
        let action = controls::apply(&mut self.options.render, key)?;
        if action == Action::Redraw {
            self.history.push(self.options.render.viewport);
        }
        Ok(action)
    }
}
