mod imp;

use glib::subclass::types::ObjectSubclassIsExt;
use imp::MainWindowTemplate;

use gdk_pixbuf::Pixbuf;
use gtk4::{
    gio::{ActionGroup, ActionMap},
    Accessible, ApplicationWindow, Buildable, ConstraintTarget, Native, Root,
    ShortcutManager, Widget, Window,
};
use libadwaita::glib::{wrapper, Object};
use libadwaita::Application;

use std::thread;

use glib::ControlFlow::Continue;
use glib::MainContext;

wrapper! {
    pub struct MainWindow(ObjectSubclass<MainWindowTemplate>)
        @extends ApplicationWindow, Window, Widget,
        @implements ActionGroup, ActionMap, Accessible, Buildable,
                    ConstraintTarget, Native, Root, ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn set_image(&self) {
        let imp = self.imp();
        imp.image.set_from_resource(Some("/test.png"));

        let (tx, rx) = MainContext::channel(glib::source::Priority::DEFAULT);
        thread::spawn(move || {
            println!("开始下载图片");
            let result = reqwest::blocking::get("https://picsum.photos/200")
                .unwrap()
                .bytes()
                .unwrap();
            let bytes = result.to_vec();
            let _ = tx.send(bytes);
        });

        let network_image = imp.network_image.clone();
        network_image.set_from_resource(Some("/loading.gif"));
        rx.attach(None, move |msg| {
            // println!("rx: {}", msg);
            let bytes = glib::Bytes::from(&msg.to_vec());
            let stream = gio::MemoryInputStream::from_bytes(&bytes);
            let pixbuf = Pixbuf::from_stream(&stream, gio::Cancellable::NONE).unwrap();
            network_image.set_from_pixbuf(Some(&pixbuf));
            Continue
        });
    }
}
