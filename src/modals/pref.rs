use adw::prelude::*;
use relm4::{adw, ComponentParts, ComponentSender, RelmContainerExt, SimpleComponent};

pub struct Preferences {
    dark_mode: bool,
    verbose_logging: bool,
    // settings: PrefSettings,
}

pub struct PrefSettings {
    pub dark_mode: bool,
    pub verbose_logging: bool,
}

impl Preferences {
    fn page_apperance() -> adw::PreferencesPage {
        let page = adw::PreferencesPage::builder().title("Appearance").build();
        let group = adw::PreferencesGroup::builder().title("Theme").build();
        page.container_add(&group);
        let switch = adw::SwitchRow::builder()
            .title("Dark Mode")
            .name("dark_mode")
            .build();
        group.container_add(&switch);
        page
    }

    fn page_general() -> adw::PreferencesPage {
        let page = adw::PreferencesPage::builder().title("General").build();
        let group = adw::PreferencesGroup::builder().title("Debugging").build();
        page.container_add(&group);
        let switch = adw::SwitchRow::builder()
            .title("Verbose Logging")
            .name("verbose")
            .build();
        group.container_add(&switch);
        page
    }
}

impl SimpleComponent for Preferences {
    type Init = PrefSettings;
    type Widgets = adw::PreferencesDialog;
    type Output = ();
    type Input = ();
    type Root = adw::PreferencesDialog;

    fn init_root() -> Self::Root {
        let pref = adw::PreferencesDialog::builder().build();
        let page_general = Self::page_general();
        let page_appearance = Self::page_apperance();

        pref.add(&page_appearance);
        pref.add(&page_general);
        pref
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            dark_mode: init.dark_mode,
            verbose_logging: init.verbose_logging,
            // settings: init,
        };
        let widgets = root.clone();
        widgets.present(Some(&relm4::main_application().windows()[0]));
        println!("{:?}", model.dark_mode);
        println!("{:?}", model.verbose_logging);

        ComponentParts { model, widgets }
    }

    fn update_view(&self, _dialog: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
