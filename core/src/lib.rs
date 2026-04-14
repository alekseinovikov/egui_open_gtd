#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod main_screen;
pub mod widgets;

use crate::widgets::{Action, Widget};
pub use app::App;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Screen<W: Widget> {
    fn render(&self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut triggered_actions: Vec<Rc<RefCell<dyn Action>>> = vec![];
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.root().borrow().render(ui, &mut triggered_actions);
        });

        for action in triggered_actions {
            action.borrow_mut().run_action();
        }
    }

    fn root(&self) -> Rc<RefCell<W>>;
}
