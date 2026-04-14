use crate::main_screen::MainScreen;
use crate::widgets::{Action, CenteredContainerWidget, Widget};
use crate::Screen;
use egui::ThemePreference;
use std::cell::RefCell;
use std::rc::Rc;

pub struct App<W: Widget> {
    _state: Rc<RefCell<AppState>>,
    current_screen: Rc<RefCell<Box<dyn Screen<W>>>>,
}

impl<W: Widget> App<W> where MainScreen: Screen<W> {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(ThemePreference::Dark);
        let state = Rc::new(RefCell::new(AppState::default()));
        Self {
            _state: Rc::<RefCell<AppState>>::clone(&state),
            current_screen: Rc::new(RefCell::new(Box::new(MainScreen::new(&state, &cc.egui_ctx)))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AppState {
    pub(crate) dark_mode: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}

impl eframe::App for App<CenteredContainerWidget> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.current_screen.borrow_mut().render(ui, _frame);
    }
}

pub(crate) struct ThemeSwitchAction {
    app_state: Rc<RefCell<AppState>>,
    ctx: egui::Context,
}

impl ThemeSwitchAction {
    pub(crate) fn new(ctx: egui::Context, app_state: Rc<RefCell<AppState>>) -> Self {
        Self { app_state, ctx }
    }
}

impl Action for ThemeSwitchAction {
    fn run_action(&mut self) {
        let new_theme = if self.app_state.borrow().dark_mode {
            ThemePreference::Light
        } else {
            ThemePreference::Dark
        };
        self.app_state.borrow_mut().dark_mode = new_theme == ThemePreference::Dark;
        self.ctx.set_theme(new_theme);
    }
}
