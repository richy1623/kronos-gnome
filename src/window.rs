use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/richard/kronos/window.ui")]
    pub struct KronosWindow {
        // This variable matches the name "label" from your blueprint file perfectly!
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for KronosWindow {
        const NAME: &'static str = "KronosWindow";
        type Type = super::KronosWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for KronosWindow {
        // This lifecycle hook triggers as soon as the window and its UI are fully constructed
        fn constructed(&self) {
            self.parent_constructed();

            // Fire up our custom UI logic
            self.obj().setup_internal_logic();
        }
    }

    impl WidgetImpl for KronosWindow {}
    impl WindowImpl for KronosWindow {}
    impl ApplicationWindowImpl for KronosWindow {}
    impl AdwApplicationWindowImpl for KronosWindow {}
}

glib::wrapper! {
    pub struct KronosWindow(ObjectSubclass<imp::KronosWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl KronosWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    // This is where you write your interactive features for the window!
    fn setup_internal_logic(&self) {
        // 1. Get access to the private UI struct layer
        let imp = self.imp();

        // 2. Fetch the actual native GtkLabel instance
        let main_label = imp.label.get();

        // 3. You can now modify it dynamically from Rust!
        main_label.set_label("Hello from the updated Rust window!");
    }
}
