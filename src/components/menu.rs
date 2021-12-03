use glib::Sender;
use gtk::prelude::*;
use super::util;
use super::application::Action as AppAction;
use super::file_dialog::Action as FileDialogAction;
use super::about_dialog::Action as AboutDialogAction;

pub enum Action {
    New,
    Open,
    Save,
    SaveAs,
    Quit,
    About,
}

pub struct Model {
    tx: Option<Sender<AppAction>>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            tx: None,
        }
    }
    pub fn transmit(&mut self, tx: Sender<AppAction>) {
        self.tx = Some(tx);
    }
    pub fn update(&mut self, action: Action) {
        let tx = self.tx.as_ref().unwrap();
        match action {
            Action::New => {
                tx.send(AppAction::OpenFile(None)).ok();
            },
            Action::Open => {
                tx.send(AppAction::FileDialog(FileDialogAction::Open)).ok();
            },
            Action::Save => {
                tx.send(AppAction::SaveFile(None)).ok();
            },
            Action::SaveAs => {
                tx.send(AppAction::FileDialog(FileDialogAction::Save)).ok();
            },
            Action::Quit => {
                tx.send(AppAction::Exit).ok();
            },
            Action::About => {
                tx.send(AppAction::AboutDialog(AboutDialogAction::Open)).ok();
            },
        };
    }
}

pub struct View {
    menu_new: gtk::MenuItem,
    menu_open: gtk::MenuItem,
    menu_save: gtk::MenuItem,
    menu_save_as: gtk::MenuItem,
    menu_quit: gtk::MenuItem,
    menu_about: gtk::MenuItem,
}

impl View {
    pub fn new(builder: &gtk::Builder) -> Self {
        let menu_new = util::get_object(builder, "menu-new");
        let menu_open = util::get_object(builder, "menu-open");
        let menu_save = util::get_object(builder, "menu-save");
        let menu_save_as = util::get_object(builder, "menu-save-as");
        let menu_quit = util::get_object(builder, "menu-quit");
        let menu_about = util::get_object(builder, "menu-about");
        Self {
            menu_new,
            menu_open,
            menu_save,
            menu_save_as,
            menu_quit,
            menu_about,
        }
    }
    pub fn transmit(&self, tx: Sender<Action>) {
        {
            let tx_local = tx.clone();
            self.menu_new.connect_activate(move |_| {
                tx_local.send(Action::New).ok();
            });
        }
        {
            let tx_local = tx.clone();
            self.menu_open.connect_activate(move |_| {
                tx_local.send(Action::Open).ok();
            });
        }
        {
            let tx_local = tx.clone();
            self.menu_save.connect_activate(move |_| {
                tx_local.send(Action::Save).ok();
            });
        }
        {
            let tx_local = tx.clone();
            self.menu_save_as.connect_activate(move |_| {
                tx_local.send(Action::SaveAs).ok();
            });
        }
        {
            let tx_local = tx.clone();
            self.menu_quit.connect_activate(move |_| {
                tx_local.send(Action::Quit).ok();
            });
        }
        {
            let tx_local = tx;
            self.menu_about.connect_activate(move |_| {
                tx_local.send(Action::About).ok();
            });
        }
    }
    pub fn refresh(&mut self, _model: &Model) {
    }
}
