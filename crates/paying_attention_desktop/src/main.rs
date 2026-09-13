use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use libadwaita::{self as adw, gtk, prelude::*};
use paying_attention_core::{AttentionWorkflow, DeclaredTask, Event, WorkflowView};
use paying_attention_desktop::{
    check_in::{CheckInInput, Energy, Environment},
    drift_recovery_screen, review_screen, strings,
    tray::AttentionTray,
    tray_command::TrayCommand,
    tray_window,
};

const APPLICATION_ID: &str = "io.github.renancesarr.PayingAttention.Spike";

#[derive(Clone, Copy)]
enum PreviewScreen {
    CheckIn,
    Review,
    DriftRecovery,
}

fn main() {
    let application = adw::Application::builder()
        .application_id(APPLICATION_ID)
        .build();
    let (sender, receiver) = mpsc::channel();
    let tray = AttentionTray::start(sender).ok();
    let receiver = Rc::new(RefCell::new(Some(receiver)));

    application.connect_activate(move |application| {
        build_spike_window(application, receiver.borrow_mut().take());
    });
    let _hold = application.hold();
    let _tray = tray;
    application.run();
}

fn build_spike_window(application: &adw::Application, commands: Option<Receiver<TrayCommand>>) {
    let screen = preview_screen();
    let workflow = Rc::new(RefCell::new(preview_workflow(screen)));
    let display = gtk::gdk::Display::default().expect("a graphical display is required");
    let monitors = display.monitors();

    for index in 0..monitors.n_items() {
        let monitor = monitors
            .item(index)
            .and_downcast::<gtk::gdk::Monitor>()
            .expect("display monitor is available");
        build_monitor_window(application, &monitor, index + 1, workflow.clone());
    }
    if let Some(commands) = commands {
        install_tray_commands(application, workflow, commands);
    }
}

fn install_tray_commands(
    application: &adw::Application,
    workflow: Rc<RefCell<AttentionWorkflow>>,
    commands: Receiver<TrayCommand>,
) {
    let application = application.clone();
    gtk::glib::timeout_add_local(Duration::from_millis(100), move || {
        while let Ok(command) = commands.try_recv() {
            tray_window::show(&application, workflow.clone(), command);
        }
        gtk::glib::ControlFlow::Continue
    });
}

fn build_monitor_window(
    application: &adw::Application,
    monitor: &gtk::gdk::Monitor,
    monitor_number: u32,
    workflow: Rc<RefCell<AttentionWorkflow>>,
) {
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title("Paying Attention")
        .build();
    window.set_decorated(false);
    window.set_content(Some(&workflow_content(
        application,
        monitor_number,
        workflow,
    )));
    install_safe_exit(&window, application);
    window.fullscreen_on_monitor(monitor);
    window.present();
}

fn preview_screen() -> PreviewScreen {
    match std::env::var("PAYING_ATTENTION_SCREEN").ok().as_deref() {
        Some("review") => PreviewScreen::Review,
        Some("drift-recovery") => PreviewScreen::DriftRecovery,
        _ => PreviewScreen::CheckIn,
    }
}

fn preview_workflow(screen: PreviewScreen) -> AttentionWorkflow {
    let task = DeclaredTask::new("Implementar a tela de revisão").expect("preview task is valid");
    let mut workflow = AttentionWorkflow::boot();
    workflow
        .dispatch(Event::BootDelayElapsed)
        .expect("Check-in is valid after boot delay");

    if matches!(screen, PreviewScreen::Review | PreviewScreen::DriftRecovery) {
        workflow
            .dispatch(Event::CheckInSubmitted {
                declared_task: task,
            })
            .expect("preview Check-in is valid");
    }
    if matches!(screen, PreviewScreen::Review) {
        workflow
            .dispatch(Event::FocusElapsed)
            .expect("preview Focus Cycle can end");
    }
    if matches!(screen, PreviewScreen::DriftRecovery) {
        workflow
            .dispatch(Event::IdleThresholdElapsed)
            .expect("preview Focus Cycle can become idle");
        workflow
            .dispatch(Event::InputDetected)
            .expect("preview Nagging Mode can recover");
    }
    workflow
}

fn workflow_content(
    application: &adw::Application,
    monitor_number: u32,
    workflow: Rc<RefCell<AttentionWorkflow>>,
) -> gtk::Box {
    let view = workflow.borrow().view();
    match view {
        WorkflowView::CheckIn { .. } => check_in_content(application, monitor_number, workflow),
        WorkflowView::Review {
            declared_task,
            continuation,
        } => {
            let application = application.clone();
            review_screen::build(declared_task, continuation, move |event| {
                let _ = workflow.borrow_mut().dispatch(event);
                application.quit();
            })
        }
        WorkflowView::DriftRecovery { .. } => {
            let application = application.clone();
            drift_recovery_screen::build(move |submission| {
                let _ = workflow
                    .borrow_mut()
                    .dispatch(Event::DriftRecoverySubmitted { submission });
                application.quit();
            })
        }
        state => panic!("unexpected preview state: {state:?}"),
    }
}

fn check_in_content(
    application: &adw::Application,
    monitor_number: u32,
    workflow: Rc<RefCell<AttentionWorkflow>>,
) -> gtk::Box {
    let heading = gtk::Label::builder()
        .label(strings::APP_TITLE)
        .css_classes(["title-1"])
        .build();
    let message = gtk::Label::builder()
        .label(format!("{} - monitor {monitor_number}", strings::CHECK_IN))
        .css_classes(["title-3"])
        .build();
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(16)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    let input = Rc::new(RefCell::new(CheckInInput::default()));
    let release = gtk::Button::with_label(strings::RELEASE);
    release.set_sensitive(false);
    let task = gtk::Entry::builder()
        .placeholder_text(strings::TASK)
        .build();
    let environment = choice_row(
        strings::ENVIRONMENT,
        ["Casa", "Escritório", "Café", "Biblioteca", "Outro"],
    );
    let energy = choice_row(
        strings::ENERGY,
        ["Exausto", "Baixo", "Neutro", "Focado", "Flow"],
    );

    connect_choice(
        &environment.1,
        input.clone(),
        release.clone(),
        |index, form| {
            form.environment = Some(
                [
                    Environment::Home,
                    Environment::Office,
                    Environment::Cafe,
                    Environment::Library,
                    Environment::Other,
                ][index],
            );
        },
    );
    connect_choice(&energy.1, input.clone(), release.clone(), |index, form| {
        form.energy = Some(
            [
                Energy::Exhausted,
                Energy::Low,
                Energy::Neutral,
                Energy::Focused,
                Energy::Flow,
            ][index],
        );
    });
    let form = input.clone();
    let button = release.clone();
    task.connect_changed(move |entry| {
        form.borrow_mut().task_text = entry.text().to_string();
        button.set_sensitive(form.borrow().can_release());
    });
    let application_for_release = application.clone();
    release.connect_clicked(move |_| {
        if let Some(event) = input.borrow().release_event() {
            let _ = workflow.borrow_mut().dispatch(event);
            application_for_release.quit();
        }
    });

    content.append(&heading);
    content.append(&message);
    content.append(&environment.0);
    content.append(&energy.0);
    content.append(&task);
    content.append(&release);
    content
}

fn choice_row(title: &str, labels: [&str; 5]) -> (gtk::Box, Vec<gtk::CheckButton>) {
    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .build();
    row.append(&gtk::Label::new(Some(title)));
    let first = gtk::CheckButton::with_label(labels[0]);
    row.append(&first);
    let mut choices = vec![first.clone()];
    for label in labels.into_iter().skip(1) {
        let button = gtk::CheckButton::with_label(label);
        button.set_group(Some(&first));
        row.append(&button);
        choices.push(button);
    }
    (row, choices)
}

fn connect_choice<F>(
    choices: &[gtk::CheckButton],
    input: Rc<RefCell<CheckInInput>>,
    release: gtk::Button,
    update: F,
) where
    F: Fn(usize, &mut CheckInInput) + Copy + 'static,
{
    for (index, choice) in choices.iter().enumerate() {
        let form = input.clone();
        let button = release.clone();
        choice.connect_toggled(move |choice| {
            if choice.is_active() {
                update(index, &mut form.borrow_mut());
                button.set_sensitive(form.borrow().can_release());
            }
        });
    }
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
