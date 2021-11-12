use glib::{Sender, Continue, MainContext, PRIORITY_DEFAULT};
use gtk::prelude::*;
use super::{body_text, file_dialog, menu, about_dialog, status_bar, util};
use gettextrs::*;

pub enum Action {
    Exit,
    OpenFile(Option<std::path::PathBuf>),
    SaveFile(Option<std::path::PathBuf>),

    BodyText(body_text::Action),
    FileDialog(file_dialog::Action),
    Menu(menu::Action),
    AboutDialog(about_dialog::Action),
    StatusBar(status_bar::Action),
}

struct ModelChildren {
    body_text: body_text::Model,
    file_dialog: file_dialog::Model,
    menu: menu::Model,
    about_dialog: about_dialog::Model,
    status_bar: status_bar::Model,
}

pub struct Model {
    file_path: Option<std::path::PathBuf>,
    children: Option<ModelChildren>,
    tx: Option<Sender<Action>>,
}

impl Model {
    pub fn new() -> Self {
        let mut model = Self {
            file_path: None,
            children: None,
            tx: None,
        };
        model.children = Some(
            ModelChildren {
                body_text: body_text::Model::new(),
                file_dialog: file_dialog::Model::new(),
                menu: menu::Model::new(),
                about_dialog: about_dialog::Model::new(),
                status_bar: status_bar::Model::new(),
            }
        );
        model
    }

    fn load_file(path: std::path::PathBuf, tx: Sender<Action>) {
        tokio::spawn(async move {
            match tokio::fs::read(path).await {
                Ok(data) => {
                    match String::from_utf8(data) {
                        Ok(document) => {
                            tx.send(
                                Action::StatusBar(status_bar::Action::ClearStatus)
                            ).ok();
                            tx.send(
                                Action::BodyText(
                                    body_text::Action::LoadDocument(document)
                                )
                            ).ok();
                        },
                        Err(e) => {
                            eprintln!("Data Error! {}", e);
                        },
                    };
                },
                Err(e) => {
                    eprintln!("IO Error! {}", e);
                },
            };
        });
    }

    fn save_file(path: std::path::PathBuf, tx: Sender<Action>, document: String) {
        tokio::spawn(async move {
            match tokio::fs::write(path.clone(), document.into_bytes()).await {
                Ok(_) => {
                    let message = format!("{}: {}",
                        gettext("Saved file to"),
                        path.to_str().unwrap()
                    );
                    tx.send(Action::StatusBar(status_bar::Action::Toast(message))).ok();
                    tx.send(Action::BodyText(body_text::Action::DocumentSaved)).ok();
                },
                Err(e) => {
                    eprintln!("IO Error! {}", e);
                },
            };
        });
    }

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Exit => {
                std::process::exit(0);
            },
            Action::OpenFile(path) => {
                self.file_path = path;
                let tx = self.tx.as_ref().unwrap();
                match self.file_path.clone() {
                    Some(p) => {
                        let message = gettext("Loading file...");
                        tx.send(Action::StatusBar(status_bar::Action::SetStatus(message))).ok();
                        Self::load_file(p, tx.clone());
                    },
                    None => {
                        tx.send(Action::BodyText(body_text::Action::LoadDocument("".into()))).ok();
                    },
                };
            },
            Action::SaveFile(path) => {
                let tx = self.tx.as_ref().unwrap();
                match path {
                    Some(p) => {
                        self.file_path = Some(p);
                    },
                    None => (),
                };
                match self.file_path.clone() {
                    Some(p) => {
                        let message = gettext("Saving file...");
                        tx.send(Action::StatusBar(status_bar::Action::SetStatus(message))).ok();
                        let document = self.children.as_ref().unwrap().body_text.get_document().clone();
                        Self::save_file(p, tx.clone(), document);
                    },
                    None => {
                        tx.send(Action::FileDialog(file_dialog::Action::Save)).ok();
                    },
                };
            },

            Action::BodyText(a) => {
                self.children.as_mut().unwrap().body_text.update(a);
            },
            Action::FileDialog(a) => {
                self.children.as_mut().unwrap().file_dialog.update(a);
            },
            Action::Menu(a) => {
                self.children.as_mut().unwrap().menu.update(a);
            },
            Action::AboutDialog(a) => {
                self.children.as_mut().unwrap().about_dialog.update(a);
            },
            Action::StatusBar(a) => {
                self.children.as_mut().unwrap().status_bar.update(a);
            },
        };
    }
    pub fn transmit(&mut self, tx: Sender<Action>) {
        self.tx = Some(tx.clone());
        {
            let tx_local = tx.clone();
            let (child_tx, child_rx) = MainContext::channel(PRIORITY_DEFAULT);
            self.children.as_mut().unwrap().file_dialog.transmit(child_tx);
            child_rx.attach(None, move |action| {
                tx_local.send(action).unwrap();
                Continue(true)
            });
        }
        {
            let tx_local = tx.clone();
            let (child_tx, child_rx) = MainContext::channel(PRIORITY_DEFAULT);
            self.children.as_mut().unwrap().menu.transmit(child_tx);
            child_rx.attach(None, move |action| {
                tx_local.send(action).unwrap();
                Continue(true)
            });
        }
        {
            let tx_local = tx.clone();
            let (child_tx, child_rx) = MainContext::channel(PRIORITY_DEFAULT);
            self.children.as_mut().unwrap().status_bar.transmit(child_tx);
            child_rx.attach(None, move |action| {
                tx_local.send(action).unwrap();
                Continue(true)
            });
        }
    }
}

pub struct View {
    widget: gtk::ApplicationWindow,
    menu: menu::View,
    body_text: body_text::View,
    file_dialog: file_dialog::View,
    about_dialog: about_dialog::View,
    status_bar: status_bar::View,
}

impl View {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/com/bernardigiri/TextEditor/window.ui");
        let body_text = body_text::View::new(&builder);
        let widget: gtk::ApplicationWindow =
            util::get_object(&builder, "window");
        let file_dialog = file_dialog::View::new(&widget);
        let menu = menu::View::new(&builder);
        let about_dialog = about_dialog::View::new(&builder);
        let status_bar = status_bar::View::new(&builder);
        Self {
            widget,
            body_text,
            file_dialog,
            menu,
            about_dialog,
            status_bar,
        }
    }
    pub fn present(&self, app: &gtk::Application) {
        self.widget.set_application(Some(app));
        app.add_window(&self.widget);
        self.widget.present();
    }
    pub fn transmit(&self, tx: Sender<Action>) {
        {
            let tx_local = tx.clone();
            let (child_tx, child_rx) = MainContext::channel(PRIORITY_DEFAULT);
            self.body_text.transmit(child_tx);
            child_rx.attach(None, move |action| {
                tx_local.send(Action::BodyText(action)).unwrap();
                Continue(true)
            });
        }
        {
            let tx_local = tx.clone();
            let (child_tx, child_rx) = MainContext::channel(PRIORITY_DEFAULT);
            self.file_dialog.transmit(child_tx);
            child_rx.attach(None, move |action| {
                tx_local.send(Action::FileDialog(action)).unwrap();
                Continue(true)
            });
        }
        {
            let tx_local = tx.clone();
            let (child_tx, child_rx) = MainContext::channel(PRIORITY_DEFAULT);
            self.menu.transmit(child_tx);
            child_rx.attach(None, move |action| {
                tx_local.send(Action::Menu(action)).unwrap();
                Continue(true)
            });
        }
    }
    pub fn refresh(&mut self, model: &Model) {
        let title = format!("{}: {}{}",
            gettext("TextEdit"),
            match &model.file_path {
                None => gettext("Untitled"),
                Some(path) => path.file_name().unwrap().to_str().unwrap().into(),
            },
            match model.children.as_ref().unwrap().body_text.is_dirty() {
                true => " *",
                false => "",
            }
        );
        self.widget.set_title(&title);
        let c = model.children.as_ref().unwrap();
        self.body_text.refresh(&c.body_text);
        self.file_dialog.refresh(&c.file_dialog);
        self.menu.refresh(&c.menu);
        self.about_dialog.refresh(&c.about_dialog);
        self.status_bar.refresh(&c.status_bar);
    }
}
