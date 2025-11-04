use gtk::gio;
use gtk::glib;
use gtk::prelude::{ApplicationExt, ApplicationWindowExt, GtkWindowExt, OrientableExt, WidgetExt};

use relm4::{
    actions::{RelmAction, RelmActionGroup},
    adw, gtk, main_application, Component, ComponentParts, ComponentSender, SimpleComponent,
};

use crate::config::{APP_ID, PROFILE};
use crate::modals::about::AboutDialog;
use crate::modals::pref::{PrefSettings, Preferences};

pub(super) struct App {}

#[derive(Debug)]
pub(super) enum AppMsg {
    Quit,
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About My Rust App" => AboutAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,
            set_default_height: 500,
            set_default_width: 600,



            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/org/mydomain/MyRustApp/gtk/help-overlay.ui"
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap() -> gtk::ShortcutsWindow {
                    set_transient_for: Some(&main_window),
                    set_application: Some(&main_application()),
            },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
                } else {
                    None
                },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },

                gtk::Label {
                    set_label: "My Rust Application",
                    add_css_class: "title-header",
                    set_vexpand: true,
                }
            }

        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = view_output!();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            RelmAction::<AboutAction>::new_stateless(move |_| {
                AboutDialog::builder().launch(()).detach();
            })
        };

        let pref_action = {
            RelmAction::<PreferencesAction>::new_stateless(move |_| {
                let settings = PrefSettings {
                    dark_mode: true,
                    verbose_logging: true,
                };
                Preferences::builder().launch(settings).detach();
            })
        };

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.add_action(pref_action);
        actions.register_for_widget(&widgets.main_window);

        widgets.load_window_size();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Quit => main_application().quit(),
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size();
    }
}

impl AppWidgets {
    fn get_settings(&self) -> Option<gio::Settings> {
        let source = gio::SettingsSchemaSource::default();
        match source {
            None => {
                println!("No default schema source found");
                return None;
            }
            Some(s) => {
                if s.lookup(APP_ID, true).is_none() {
                    println!("Schema {} not found", APP_ID);
                    return None;
                }
            }
        }
        Some(gio::Settings::new(APP_ID))
    }
    fn save_window_size(&self) {
        let settings = match self.get_settings() {
            None => return,
            Some(s) => s,
        };
        println!("{:?}", settings)
        // let (width, height) = self.main_window.default_size();
        // settings.set_int("windows", width);
        // s.set_int("window-width", width)?;
        // s.set_int("window-height", height)?;
        // s.set_boolean("is-maximized", self.main_window.is_maximized())?;
    }

    fn load_window_size(&self) {
        let settings = match self.get_settings() {
            None => return,
            Some(s) => s,
        };
        println!("{:?}", settings)
        // let width = settings.int("window-width");
        // let height = settings.int("window-height");
        // let is_maximized = settings.boolean("is-maximized");

        // self.main_window.set_default_size(width, height);

        // if is_maximized {
        //     self.main_window.maximize();
        // }
    }
}
