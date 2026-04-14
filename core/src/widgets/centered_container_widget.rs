use super::{Action, Widget};
use std::cell::RefCell;
use std::rc::Rc;

pub struct CenteredContainerWidget {
    widget_children: Vec<Box<dyn Widget>>,
}

impl Default for CenteredContainerWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl CenteredContainerWidget {
    pub fn new() -> Self {
        Self {
            widget_children: Vec::new(),
        }
    }

    pub fn child(mut self, widget: impl Widget + 'static) -> Self {
        self.widget_children.push(Box::new(widget));
        self
    }

    fn children(&self) -> &[Box<dyn Widget>] {
        &self.widget_children
    }

    fn render_children(&self, ui: &mut egui::Ui, actions: &mut Vec<Rc<RefCell<dyn Action>>>) {
        let children = self.children();
        for (i, child) in children.iter().enumerate() {
            child.render(ui, actions);

            if i < children.len() - 1 {
                ui.add_space(8.0);
            }
        }
    }
}

impl Widget for CenteredContainerWidget {
    fn render(&self, ui: &mut egui::Ui, actions: &mut Vec<Rc<RefCell<dyn Action>>>) {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                ui.vertical_centered(|ui| {
                    self.render_children(ui, actions);
                });
            },
        );
    }
}
