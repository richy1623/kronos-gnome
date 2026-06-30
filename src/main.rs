mod application;
mod config;
mod window;

use std::env;
use std::path::PathBuf;

use self::application::KronosApplication;
use self::window::KronosWindow;

use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::gdk::Display;
use gtk::{CssProvider, prelude::*};
use gtk::{gio, glib};

fn main() -> glib::ExitCode {
    // Set up gettext translations
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    // Load resources
    let resource_path = if env::var("MESON_DEVENV").is_ok() {
        // Development mode: Look in the same directory as the running binary
        let mut exe_dir = env::current_exe().expect("Failed to get current exe path");
        exe_dir.pop();
        exe_dir.join("kronos.gresource")
    } else {
        // Production mode: Use the Meson-configured installation directory
        PathBuf::from(PKGDATADIR).join("kronos.gresource")
    };
    let resources = gio::Resource::load(&resource_path).expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = KronosApplication::new("io.richard.kronos", &gio::ApplicationFlags::empty());

    app.connect_startup(|_| load_css());

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
