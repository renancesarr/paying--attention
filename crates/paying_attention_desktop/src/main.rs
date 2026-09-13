use libadwaita::{self as adw, gtk, prelude::*};

const APPLICATION_ID: &str = "io.github.renancesarr.PayingAttention.Spike";

fn main() {
    let application = adw::Application::builder()
        .application_id(APPLICATION_ID)
        .flags(gtk::gio::ApplicationFlags::NON_UNIQUE)
        .build();

    application.connect_activate(build_spike_window);
    let _hold = application.hold();
    application.run();
}

fn build_spike_window(application: &adw::Application) {
    let display = gtk::gdk::Display::default().expect("a graphical display is required");
    let monitors = display.monitors();

    for index in 0..monitors.n_items() {
        let monitor = monitors
            .item(index)
            .and_downcast::<gtk::gdk::Monitor>()
            .expect("display monitor is available");
        build_monitor_window(application, &monitor, index + 1);
    }
}

fn build_monitor_window(
    application: &adw::Application,
    monitor: &gtk::gdk::Monitor,
    monitor_number: u32,
) {
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title("Paying Attention")
        .build();
    window.set_decorated(false);
    window.set_content(Some(&spike_content(monitor_number)));
    install_safe_exit(&window, application);
    window.fullscreen_on_monitor(monitor);
    window.present();
}

fn spike_content(monitor_number: u32) -> gtk::Box {
    let heading = gtk::Label::builder()
        .label("PAYING ATTENTION")
        .css_classes(["title-1"])
        .build();
    let message = gtk::Label::builder()
        .label(format!(
            "Spike de tela cheia em execução - monitor {monitor_number}"
        ))
        .css_classes(["title-3"])
        .build();
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(16)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    content.append(&heading);
    content.append(&message);
    content
}

fn install_safe_exit(window: &adw::ApplicationWindow, application: &adw::Application) {
    let controller = gtk::EventControllerKey::new();
    let application_for_exit = application.clone();
    controller.connect_key_pressed(move |_, key, _, modifiers| {
        let safe_exit = key == gtk::gdk::Key::F
            && modifiers.contains(gtk::gdk::ModifierType::CONTROL_MASK)
            && modifiers.contains(gtk::gdk::ModifierType::SHIFT_MASK);

        if safe_exit {
            application_for_exit.quit();
            gtk::glib::Propagation::Stop
        } else {
            gtk::glib::Propagation::Proceed
        }
    });
    window.add_controller(controller);
}
