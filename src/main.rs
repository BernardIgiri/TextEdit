use gettextrs::*;
use gio::prelude::*;
use std::env;
use glib::{Continue, MainContext, PRIORITY_DEFAULT};

mod config;
mod components;

use components::application;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1].eq("-v") {
        println!("{}", config::VERSION);
        return;
    }
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("my-gtk-text-editor", config::LOCALEDIR);
    textdomain("my-gtk-text-editor");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/my-gtk-text-editor.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app = gtk::Application::new(Some("com.bernardigiri.TextEditor"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        let mut model = application::Model::new();
        let mut view = application::View::new();

        let (tx, rx) = MainContext::channel(PRIORITY_DEFAULT);

        view.refresh(&model);
        model.transmit(tx.clone());
        view.transmit(tx);
        view.present(app);

        rx.attach(None, move |action| {
            model.update(action);
            view.refresh(&model);
            Continue(true)
        });
    });

    let ret = app.run(&args);
    std::process::exit(ret);
}
