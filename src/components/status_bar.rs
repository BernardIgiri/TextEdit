use glib::Sender;
use gtk::prelude::*;
use super::util;
use super::application::Action as AppAction;
use std::{thread, time};

const TOAST_DURATION_SECONDS: u64 = 3;

pub enum Action {
    SetStatus(String),
    Toast(String),
    ClearStatus,
}

pub struct Model {
    message: Option<String>,
    revision: u64,
    tx: Option<Sender<AppAction>>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            message: None,
            revision: 0,
            tx: None,
        }
    }
    pub fn update(&mut self, action: Action) {
        match action {
            Action::SetStatus(message) => {
                self.message = Some(message);
                self.revision += 1;
            },
            Action::Toast(message) => {
                self.message = Some(message);
                self.revision += 1;
                let tx_local = self.tx.as_ref().unwrap().clone();
                tokio::spawn(async move {
                    thread::sleep(time::Duration::from_secs(TOAST_DURATION_SECONDS));
                    tx_local.send(AppAction::StatusBar(Action::ClearStatus)).ok();
                });
            },
            Action::ClearStatus => {
                self.message = None;
                self.revision += 1;
            },
        }
    }
    pub fn transmit(&mut self, tx: Sender<AppAction>) {
        self.tx = Some(tx.clone());
    }
}

pub struct View {
    widget: gtk::Label,
    revision: u64,
}

impl View {
    pub fn new(builder: &gtk::Builder) -> Self {
        let widget = util::get_object(&builder, "status-bar");
        Self {
            widget,
            revision: 0,
        }
    }
    pub fn refresh(&mut self, model: &Model) {
        if self.revision != model.revision {
            self.revision = model.revision;
            match &model.message {
                Some(message) => {
                    self.widget.set_text(message.as_str());
                },
                None => {
                    self.widget.set_text("");
                },
            }
        }
    }
}
