use crate::Screen;
use crate::app::AppState;
use crate::widgets::{ButtonWidget, CenteredContainerWidget, TextWidget};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MainScreen {
    current_screen: Rc<RefCell<CenteredContainerWidget>>,
}

impl MainScreen {
    pub fn new(app_state: &Rc<RefCell<AppState>>, ctx: &egui::Context) -> Self {
        let app_state_for_label = Rc::<RefCell<AppState>>::clone(app_state);
        let label = Box::new(move || {
            if app_state_for_label.borrow().dark_mode {
                "Switch to Light".to_owned()
            } else {
                "Switch to Dark".to_owned()
            }
        });

        let app_state_for_action = Rc::<RefCell<AppState>>::clone(app_state);
        let layout = CenteredContainerWidget::new()
            .child(TextWidget::new(Box::new(|| "Hello World!".to_owned())))
            .child(ButtonWidget::new(
                label,
                Rc::new(RefCell::new(crate::app::ThemeSwitchAction::new(
                    ctx.clone(),
                    app_state_for_action,
                ))),
            ));

        Self {
            current_screen: Rc::new(RefCell::new(layout)),
        }
    }
}

impl Screen<CenteredContainerWidget> for MainScreen {
    fn root(&self) -> Rc<RefCell<CenteredContainerWidget>> {
        Rc::<RefCell<CenteredContainerWidget>>::clone(&self.current_screen)
    }
}
