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
    fn children(&self) -> &[Box<dyn Widget>] {
        &[]
    }

    fn render(&self, ui: &mut egui::Ui) -> Option<Action> {
        ui.heading(&self.text);
        None
    }
}
