use super::{Action, Widget};

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
}

impl Widget for CenteredContainerWidget {
    fn children(&self) -> &[Box<dyn Widget>] {
        &self.widget_children
    }

    fn render(&self, ui: &mut egui::Ui) -> Option<Action> {
        let mut result = None;
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                ui.vertical_centered(|ui| {
                    result = self.render_children(ui);
                });
            },
        );
        result
    }
}
