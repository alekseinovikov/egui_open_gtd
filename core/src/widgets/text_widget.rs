use super::{Action, Widget};
use std::cell::RefCell;
use std::rc::Rc;

pub struct TextWidget {
    label_provider: Box<dyn Fn() -> String>,
}

impl TextWidget {
    pub fn new(label_provider: Box<dyn Fn() -> String>) -> Self {
        Self { label_provider }
    }
}

impl Widget for TextWidget {
    fn render(&self, ui: &mut egui::Ui, _: &mut Vec<Rc<RefCell<dyn Action>>>) {
        ui.heading((self.label_provider)());
    }
}
