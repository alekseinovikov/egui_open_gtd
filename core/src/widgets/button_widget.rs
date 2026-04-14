use super::{Action, Widget};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ButtonWidget {
    label_provider: Box<dyn Fn() -> String>,
    action: Rc<RefCell<dyn Action>>,
}

impl ButtonWidget {
    pub fn new(label_provider: Box<dyn Fn() -> String>, action: Rc<RefCell<dyn Action>>) -> Self {
        Self {
            label_provider,
            action,
        }
    }
}

impl Widget for ButtonWidget {
    fn render(&self, ui: &mut egui::Ui, actions: &mut Vec<Rc<RefCell<dyn Action>>>) {
        if ui.button((self.label_provider)()).clicked() {
            actions.push(Rc::clone(&self.action));
        }
    }
}
