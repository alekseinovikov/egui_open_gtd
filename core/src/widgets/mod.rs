mod button_widget;
mod centered_container_widget;
mod text_widget;

pub use button_widget::ButtonWidget;
pub use centered_container_widget::CenteredContainerWidget;
use std::cell::RefCell;
use std::rc::Rc;
pub use text_widget::TextWidget;

pub trait Action {
    fn run_action(&mut self);
}

pub trait Widget {
    fn render(&self, ui: &mut egui::Ui, actions: &mut Vec<Rc<RefCell<dyn Action>>>);
}
