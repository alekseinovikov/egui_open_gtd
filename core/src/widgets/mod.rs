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
    fn render(&self, ui: &mut egui::Ui) -> Vec<Action>;
}
