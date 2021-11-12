use gtk::prelude::*;
use super::util;
use super::super::config;

pub enum Action {
    Open,
}

pub struct Model {
    revision: u64,
}

impl Model {
    pub fn new() -> Self {
        Self {
            revision: 0,
        }
    }
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Open => {
                self.revision += 1;
            },
        }
    }
}

pub struct View {
    widget: gtk::AboutDialog,
    revision: u64,
}

impl View {
    pub fn new(builder: &gtk::Builder) -> Self {
        let widget: gtk::AboutDialog = util::get_object(&builder, "about-dialog");
        let version = format!("{} {}",
            gettextrs::gettext("Version"),
            config::VERSION,
        );
        widget.set_version(Some(version.as_str()));
        Self {
            widget,
            revision: 0,
        }
    }
    pub fn refresh(&mut self, model: &Model) {
        if self.revision != model.revision {
            self.revision = model.revision;
            self.widget.show();
        }
    }
}
