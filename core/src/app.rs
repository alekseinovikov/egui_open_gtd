use crate::Screen as _;
use crate::main_screen::MainScreen;
use crate::widgets::Action;
use egui::ThemePreference;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct App {
    state: Rc<RefCell<AppState>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(ThemePreference::Dark);
        Self {
            state: Rc::new(RefCell::new(AppState::default())),
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

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        MainScreen::new(&self.state, ui).render(ui, _frame);
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
