use adw::prelude::AdwDialogExt;
use gtk::prelude::GtkApplicationExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::config::{APP_ID, APP_NAME, DEVELOPER_NAME, GITHUB_ID, VERSION};

pub struct AboutDialog {}

impl SimpleComponent for AboutDialog {
    type Init = ();
    type Widgets = adw::AboutDialog;
    type Input = ();
    type Output = ();
    type Root = adw::AboutDialog;

    fn init_root() -> Self::Root {
        adw::AboutDialog::builder()
            .application_icon(APP_ID)
            // Insert your license of choice here
            .license_type(gtk::License::MitX11)
            // Insert your website here
            .website(format!("https://github.com/{GITHUB_ID}/{APP_NAME}"))
            // Insert your Issues page
            .issue_url(format!("https://github.com/{GITHUB_ID}/{APP_NAME}/issues"))
            // Insert your application name here
            .application_name("My Rust Application")
            .version(VERSION)
            .translator_credits("translator-credits")
            .copyright(format!("© 2025 {DEVELOPER_NAME}"))
            .developers(vec![DEVELOPER_NAME])
            .designers(vec![DEVELOPER_NAME])
            .build()
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = root.clone();
        widgets.present(Some(&relm4::main_application().windows()[0]));

        ComponentParts { model, widgets }
    }

    fn update_view(&self, _dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
