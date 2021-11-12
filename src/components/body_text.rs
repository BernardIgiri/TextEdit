use glib::Sender;
use gtk::prelude::*;
use super::util;

pub enum Action {
    LoadDocument(String),
    DocumentChanged(String),
    DocumentSaved,
}

pub struct Model {
    saved_document: String,
    active_document: String,
    revision: u64,
}

impl Model {
    pub fn new() -> Self {
        Self {
            saved_document: "".into(),
            active_document: "".into(),
            revision: 0,
        }
    }
    pub fn update(&mut self, action: Action) {
        match action {
            Action::LoadDocument(value) => {
                self.saved_document = value.clone();
                self.active_document = value;
                self.revision += 1;
            },
            Action::DocumentChanged(value) => {
                self.active_document = value;
            },
            Action::DocumentSaved => {
                self.saved_document = self.active_document.clone();
            },
        }
    }
    pub fn is_dirty(&self) -> bool {
        !self.saved_document.eq(&self.active_document)
    }
    pub fn get_document(&self) -> &String {
        &self.active_document
    }
}

pub struct View {
    widget: gtk::TextBuffer,
    revision: u64,
}

impl View {
    pub fn new(builder: &gtk::Builder) -> Self {
        let widget = util::get_object(&builder, "body-text");
        Self {
            widget,
            revision: 0,
        }
    }
    pub fn transmit(&self, tx: Sender<Action>) {
        let tx_local = tx.clone();
        self.widget.connect_changed(move |body_text| {
            let start = body_text.get_start_iter();
            let end = body_text.get_end_iter();
            let value = body_text.get_text(&start, &end, true).unwrap_or("".into()).to_string();
            tx_local.send(Action::DocumentChanged(value)).ok();
        });
    }
    pub fn refresh(&mut self, model: &Model) {
        if self.revision != model.revision {
            self.revision = model.revision;
            self.widget.set_text(model.saved_document.as_str());
        }
    }
}
