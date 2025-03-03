use crate::tui::{
    con,
    emg,
};

struct Menu<'a> {
    containers: Vec<&'a con::Container>,
}

impl<'a> Menu<'a> {
    pub fn new(main_menu: &'a con::Container) -> Menu<'a> {
        Menu {
            containers: vec![main_menu],
        }
    }

    pub fn current(&self) -> &con::Container {
        self.containers.last().expect(emg::NO_MAIN_MENU_ERRMSG)
    }

    pub fn set_current(&mut self, container: &'a con::Container) {
        self.containers.push(container);
    }

    pub fn back(&mut self) {
        if self.containers.len() > 1 {
            let _ = self.containers.pop();
        }
    }
}
