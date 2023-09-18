use super::MainWindow;

use glib::subclass::{
    object::ObjectImplExt,
    prelude::{ObjectImpl, ObjectSubclass},
    InitializingObject,
};

use gtk4::subclass::widget::CompositeTemplateClass;
use gtk4::subclass::widget::WidgetClassExt;
use gtk4::{
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{
            CompositeTemplateInitializingExt as InitializingWidgetExt, TemplateChild, WidgetImpl,
            WindowImpl,
        },
    },
    Button, CompositeTemplate, Image,
};
use libadwaita::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow};

#[derive(CompositeTemplate, Default, Debug)]
#[template(resource = "/main-window.ui")]
pub struct MainWindowTemplate {
    #[template_child]
    pub name: TemplateChild<gtk4::Label>,

    #[template_child]
    pub button: TemplateChild<Button>,

    #[template_child]
    pub image: TemplateChild<Image>,

    #[template_child]
    pub network_image: TemplateChild<Image>
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindowTemplate {
    const NAME: &'static str = "MainWindow";

    type Type = MainWindow;
    type ParentType = ApplicationWindow;

    fn class_init(my_class: &mut Self::Class) {
        my_class.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindowTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}
