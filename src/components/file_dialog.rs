use glib::Sender;
use gtk::prelude::*;
use super::application::Action as AppAction;

pub enum Action {
    Open,
    Save,
    Accept(std::path::PathBuf),
    Cancel,
}

pub enum State {
    ShowOpen,
    ShowSave,
    Hide,
}

pub struct Model {
    state: State,
    revision: u64,
    tx: Option<Sender<AppAction>>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            state: State::Hide,
            revision: 0,
            tx: None,
        }
    }
    pub fn transmit(&mut self, tx: Sender<AppAction>) {
        self.tx = Some(tx);
    }
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Open => {
                self.state = State::ShowOpen;
                self.revision += 1;
            },
            Action::Save => {
                self.state = State::ShowSave;
                self.revision += 1;
            },
            Action::Accept(path) => {
                let tx = self.tx.as_ref().unwrap();
                match self.state {
                    State::ShowOpen => {
                        tx.send(AppAction::OpenFile(Some(path))).ok();
                    },
                    State::ShowSave => {
                        tx.send(AppAction::SaveFile(Some(path))).ok();
                    },
                    State::Hide => {
                        // TODO find better solution
                        panic!("This should not happen.");
                    }
                };
            },
            Action::Cancel => {
                self.state = State::Hide;
            }
        }
    }
}

pub struct View {
    widget: gtk::FileChooserNative,
    revision: u64,
}

impl View {
    pub fn new(app_window: &gtk::ApplicationWindow) -> Self {
        let widget: gtk::FileChooserNative = gtk::FileChooserNative::new(None, Some(app_window), gtk::FileChooserAction::Open, None, Some("Close"));
        let filter = gtk::FileFilter::new();
        filter.add_mime_type("text/plain");
        filter.add_pattern("*.txt");
        filter.add_mime_type("application/octet-stream");
        filter.add_pattern("*");
        widget.set_filter(&filter);
        Self {
            widget,
            revision: 0,
        }
    }
    pub fn transmit(&self, tx: Sender<Action>) {
        self.widget.connect_response(move |chooser, response_type| {
            if let gtk::ResponseType::Accept = response_type {
                let path = chooser.get_filename().unwrap();
                tx.send(Action::Accept(path)).ok();
            } else {
                tx.send(Action::Cancel).ok();
            }
        });
    }
    pub fn refresh(&mut self, model: &Model) {
        if self.revision != model.revision {
            self.revision = model.revision;
            match model.state {
                State::ShowOpen => {
                    self.widget.set_action(gtk::FileChooserAction::Open);
                    self.widget.set_title("Open File");
                    self.widget.set_accept_label(Some("Open"));
                    self.widget.show();
                },
                State::ShowSave => {
                    self.widget.set_action(gtk::FileChooserAction::Save);
                    self.widget.set_title("Save File");
                    self.widget.set_accept_label(Some("Save"));
                    self.widget.show();
                },
                State::Hide => {
                    self.widget.hide();
                },
            }
        }
    }
}
