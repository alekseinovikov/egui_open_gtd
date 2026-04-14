mod button_widget;
mod centered_container_widget;
mod text_widget;

pub use button_widget::ButtonWidget;
pub use centered_container_widget::CenteredContainerWidget;
pub use text_widget::TextWidget;

#[derive(Clone, Copy)]
pub enum Action {
    ToggleTheme,
}

pub trait Widget {
    fn children(&self) -> &[Box<dyn Widget>];
    fn render(&self, ui: &mut egui::Ui) -> Option<Action>;

    fn render_children(&self, ui: &mut egui::Ui) -> Option<Action> {
        let children = self.children();
        let mut result = None;
        for (i, child) in children.iter().enumerate() {
            if let Some(action) = child.render(ui) {
                result = Some(action);
            }
            if i < children.len() - 1 {
                ui.add_space(8.0);
            }
        }
        result
    }
}
