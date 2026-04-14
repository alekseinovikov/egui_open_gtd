use crate::widgets::{Action, ButtonWidget, CenteredContainerWidget, TextWidget, Widget};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    dark_mode: bool,
    #[serde(skip)]
    root: Option<Box<dyn Widget>>,
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: Self = cc
            .storage
            .and_then(|s| eframe::get_value(s, eframe::APP_KEY))
            .unwrap_or_default();

        let theme = if app.dark_mode {
            egui::ThemePreference::Dark
        } else {
            egui::ThemePreference::Light
        };
        cc.egui_ctx.set_theme(theme);

        app.rebuild_ui();
        app
    }

    fn rebuild_ui(&mut self) {
        let label = if self.dark_mode {
            "Switch to Light"
        } else {
            "Switch to Dark"
        };

        self.root = Some(Box::new(
            CenteredContainerWidget::new()
                .child(TextWidget::new("Hello World!"))
                .child(ButtonWidget::new(label, Action::ToggleTheme)),
        ));
    }

    fn handle_action(&mut self, action: Action, ctx: &egui::Context) {
        match action {
            Action::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                let theme = if self.dark_mode {
                    egui::ThemePreference::Dark
                } else {
                    egui::ThemePreference::Light
                };
                ctx.set_theme(theme);
            }
        }
        self.rebuild_ui();
    }
}

impl eframe::App for TemplateApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Take the root out so `self` is free for handle_action
        let root = self.root.take().expect("root widget not initialized");
        let mut triggered_actions: Vec<Action> = vec![];
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let actions = root.render(ui);
            triggered_actions.extend(actions);
        });
        self.root = Some(root);

        for action in triggered_actions {
            self.handle_action(action, ui.ctx());
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
