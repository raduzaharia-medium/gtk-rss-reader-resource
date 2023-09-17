mod imp;

use glib::subclass::types::ObjectSubclassIsExt;
use imp::MainWindowTemplate;

use libadwaita::glib::{wrapper, Object};
use gtk4::{
    gio::{ActionGroup, ActionMap},
    Accessible, ApplicationWindow, Buildable, ConstraintTarget, Native, Root, ShortcutManager,
    Widget, Window,
};
use libadwaita::Application;

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
    }
}
