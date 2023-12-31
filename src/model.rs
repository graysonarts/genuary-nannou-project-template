use crate::framework::*;

#[derive(Default, PartialEq)]
pub enum RunMode {
    #[default]
    Production,
    Debug,
}
pub struct Model {
    pub run_mode: RunMode,
    pub label: String,
}

impl Model {
    pub fn new(debug: RunMode) -> Model {
        Model {
            run_mode: debug,
            ..Default::default()
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            run_mode: Default::default(),
            label: projname().unwrap_or_else(|| "unlabeled".to_owned()),
        }
    }
}
