use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::unsync::OnceCell;

use crate::app::MyApp;

#[derive(Debug)]
pub struct WindowWidgets {
    headerbar: gtk::HeaderBar,
    pub statusbar: gtk::Statusbar,
    notebook: gtk::Notebook,
}

#[derive(Debug, Default)]
pub struct Window {
    app: OnceCell<gtk::Application>,
    super_: OnceCell<MainWindow>,
    pub widgets: OnceCell<WindowWidgets>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = MainWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for Window {
    // Here we are overriding the glib::Object::contructed
    // method. Its what gets called when we create our Object
    // and where we can initialize things.
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        self.super_.set(obj.clone()).unwrap();

        let headerbar = gtk::HeaderBar::new();

        headerbar.set_title(Some("hello world"));
        headerbar.set_show_close_button(true);

        let notebook = gtk::Notebook::builder()
            .expand(true)
            .visible(true)
            .can_focus(true)
            .name("main-window-notebook")
            .show_tabs(true)
            .scrollable(true)
            .enable_popup(true)
            .show_border(true)
            .build();

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .expand(true)
            .spacing(5)
            .visible(true)
            .can_focus(true)
            .build();
        vbox.pack_start(&notebook, true, true, 0);

        let statusbar = gtk::Statusbar::builder()
            .vexpand(false)
            .hexpand(true)
            .visible(true)
            .can_focus(true)
            .margin(0)
            .build();
        vbox.pack_start(&statusbar, false, false, 0);

        obj.set_child(Some(&vbox));
        obj.set_titlebar(Some(&headerbar));
        obj.set_default_size(640, 480);
        obj.set_events(
            gtk::gdk::EventMask::POINTER_MOTION_MASK
                | gtk::gdk::EventMask::ENTER_NOTIFY_MASK
                | gtk::gdk::EventMask::LEAVE_NOTIFY_MASK,
        );

        self.widgets
            .set(WindowWidgets {
                headerbar,
                statusbar,
                notebook,
            })
            .expect("Failed to initialize window state");
    }
}

impl WidgetImpl for Window {}
impl ContainerImpl for Window {}
impl BinImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<Window>)
        @extends gtk::Widget, gtk::Container, gtk::Bin, gtk::Window, gtk::ApplicationWindow;
}

impl MainWindow {
    pub fn new(app: &MyApp) -> Self {
        let ret: Self =
            glib::Object::new(&[("application", app)]).expect("Failed to create Main Window");
        ret.imp()
            .app
            .set(app.upcast_ref::<gtk::Application>().clone())
            .unwrap();
        ret
    }
}
