use super::{Action, Widget};

pub struct TextWidget {
    text: String,
}

impl TextWidget {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for TextWidget {
    fn render(&self, ui: &mut egui::Ui) -> Vec<Action> {
        ui.heading(&self.text);
        vec![]
    }
}
