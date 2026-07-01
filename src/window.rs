use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::window::imp::{HOUR_HEIGHT, MINUTES_15_HEIGHT, TIME_OFFSET};

mod imp {
    use super::*;

    pub(crate) const HOUR_HEIGHT: f64 = 60.0;
    pub(crate) const MINUTES_15_HEIGHT: i32 = 15;
    pub(crate) const TIME_OFFSET: i32 = 20;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/richard/kronos/window.ui")]
    pub struct KronosWindow {
        #[template_child]
        pub window: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub calendar_times_grid: TemplateChild<gtk::Box>,
        #[template_child]
        pub transparent_dimmer: TemplateChild<adw::Bin>,
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
            self.obj().build_calendar_time_slots();
            self.obj().setup_drag_area();
            // self.obj().build_drag_and_drop_panel();
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
    fn setup_drag_area(&self) {
        // 1. Get access to the private UI struct layer
        let imp = self.imp();

        // 2. Fetch the actual native GtkLabel instance
        let window = imp.window.get();
        let transparent_dimmer = imp.transparent_dimmer.get();

        let drag_gesture = gtk::GestureDrag::new();
        drag_gesture.set_propagation_phase(gtk::PropagationPhase::Capture);

        drag_gesture.connect_drag_begin(glib::clone!(
            #[weak]
            transparent_dimmer,
            #[weak]
            window,
            move |_gesture, _x_start, y_start| {
                let y_start = y_start as i32 - TIME_OFFSET + window.vadjustment().value() as i32;
                // Snap to earlier 15 minutes
                let start = y_start - y_start % (MINUTES_15_HEIGHT);
                transparent_dimmer.set_margin_top(start + TIME_OFFSET);
                transparent_dimmer.set_height_request(MINUTES_15_HEIGHT);
                transparent_dimmer.set_visible(true);
            }
        ));

        drag_gesture.connect_drag_update(glib::clone!(
            #[weak]
            transparent_dimmer,
            move |_gesture, _offset_x, y_offset| {
                let y_offset = y_offset as i32;
                if y_offset < 0 {
                    // Snap height to prior 15 minutes
                    let height = -(y_offset - MINUTES_15_HEIGHT - y_offset % (MINUTES_15_HEIGHT));
                    // Snap top to use new height
                    let current_end =
                        transparent_dimmer.margin_top() + transparent_dimmer.height_request();
                    // Perform updates
                    transparent_dimmer.set_margin_top(current_end - height);
                    transparent_dimmer.set_height_request(height);
                } else {
                    // Snap to next 15 minutes
                    let height = y_offset + MINUTES_15_HEIGHT - y_offset % (MINUTES_15_HEIGHT);
                    transparent_dimmer.set_height_request(height);
                }
            }
        ));

        drag_gesture.connect_drag_end(move |_gesture, _offset_x, _offset_y| {
            let current_start_hours =
                (transparent_dimmer.margin_top() - TIME_OFFSET) as f64 / HOUR_HEIGHT;
            let current_end_hours =
                current_start_hours + transparent_dimmer.height_request() as f64 / HOUR_HEIGHT;
            println!(
                "Publish new event: [{:02.02}-{:02.02}]",
                current_start_hours, current_end_hours
            );
        });

        window.add_controller(drag_gesture);

        // // 3. You can now modify it dynamically from Rust!
        // main_label.set_label("Hello from the updated Rust window!");
    }

    fn build_calendar_time_slots(&self) {
        // 1. Get access to the private UI struct layer
        let imp = self.imp();

        // 2. Fetch the actual native GtkLabel instance
        let calendar_times_grid = imp.calendar_times_grid.get();

        for hour in 0..24 {
            let row_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .height_request(60)
                .spacing(0)
                .build();

            let label_text = format!("{:02}:00", hour);
            let time_label = gtk::Label::builder()
                .label(&label_text)
                .width_request(60)
                .halign(gtk::Align::End)
                .valign(gtk::Align::Start)
                .margin_top(0)
                .build();
            time_label.add_css_class("caption");
            time_label.add_css_class("dim-label");

            let separator = gtk::Separator::builder()
                .orientation(gtk::Orientation::Horizontal)
                .hexpand(true)
                .valign(gtk::Align::Start)
                .margin_top(7)
                .build();
            separator.add_css_class("sidebar-separator");

            row_box.append(&time_label);
            row_box.append(&separator);
            calendar_times_grid.append(&row_box);
        }
    }

    // fn build_drag_and_drop_panel(&self) {
    //     // 1. Get access to the private UI struct layer
    //     let imp = self.imp();

    //     // 2. Fetch the actual native GtkLabel instance
    //     let calendar_times_grid = imp.calendar_times_grid.get();
    //     let overlay = gtk::Overlay::builder()
    //         .can_focus(false)
    //         .height_request(200)
    //         .build();

    //     overlay.add_overlay(&calendar_times_grid);
    //     // calendar_times_grid.append(&overlay);
    // }
}
