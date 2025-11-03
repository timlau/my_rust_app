use adw::prelude::AdwDialogExt;
use gtk::prelude::GtkApplicationExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct PrefDialog {}

impl SimpleComponent for PrefDialog {
    type Init = ();
    type Widgets = adw::PreferencesDialog;
    type Input = ();
    type Output = ();
    type Root = adw::PreferencesDialog;

    fn init_root() -> Self::Root {
        adw::PreferencesDialog::builder().build()
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
