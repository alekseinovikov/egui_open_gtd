use super::{Action, Widget};

pub struct ButtonWidget {
    label: String,
    action: Action,
}

impl ButtonWidget {
    pub fn new(label: impl Into<String>, action: Action) -> Self {
        Self {
            label: label.into(),
            action,
        }
    }
}

impl Widget for ButtonWidget {
    fn render(&self, ui: &mut egui::Ui) -> Vec<Action> {
        if ui.button(&self.label).clicked() {
            vec![self.action]
        } else {
            vec![]
        }
    }
}
