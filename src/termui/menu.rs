use crate::tui::con;
use crate::tui::emg;

pub struct Menu<'a> {
    containers: Vec<&'a mut con::Container>,
}

impl<'a> Menu<'a> {
    pub fn new(main_menu: &'a mut con::Container) -> Self {
        Menu { containers: vec![main_menu], }
    }

    pub fn current_mut(&mut self) -> &mut con::Container {
        self.containers.last_mut().expect(emg::NO_MAIN_MENU_ERRMSG)
    }

    pub fn current(&self) -> &con::Container {
        self.containers.last().expect(emg::NO_MAIN_MENU_ERRMSG)
    }

    pub fn back(&mut self) {
        if self.containers.len() > 1 {
            let _ = self.containers.pop();
        }
    }
}
