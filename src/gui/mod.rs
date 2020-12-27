use gtk::prelude::*;
use std::{sync::Arc};

use crate::netctl;

pub struct App {
    pub window: gtk::ApplicationWindow,
    pub header: Header,
    pub content: Content,
}

pub struct Header {
    pub headerbar: gtk::HeaderBar,
    pub add_button: gtk::Button,
}

pub struct Content {
    pub root_container: gtk::Box,
    pub profile_list: ProfileList,
    pub bottombar: BottomBar,
}

pub struct ProfileList {
    pub container: gtk::ScrolledWindow,
    pub list: gtk::TreeView,
    pub model: Arc<gtk::ListStore>,
}

pub struct BottomBar {
    pub actionbar: gtk::ActionBar,
    pub switch_button: gtk::Button,
    pub enable_button: gtk::Button,
    pub disable_button: gtk::Button,
    pub edit_button: gtk::Button,
    pub delete_button: gtk::Button,
}

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    Connection = 0,
    Interface,
    ESSID,
    Status,
}

impl App {
    fn new(application: &gtk::Application) -> App {
        // create
        let window = gtk::ApplicationWindow::new(application);
        let header = Header::new();
        let content = Content::new();

        // configure window
        window.set_title("Netti");
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(100, 500);
        window.set_wmclass("netti", "Netti");
        gtk::Window::set_default_icon_name("preferences-system-network");

        // set headerbar
        window.set_titlebar(Some(&header.headerbar));

        // add the root container to the window
        window.add(&content.root_container);

        // return main application state
        App { window, header , content }
    }
}

impl Header {
    fn new() -> Header {
        // Creates the main header bar container widget.
        let headerbar = gtk::HeaderBar::new();
        headerbar.set_title(Some("Netti"));
        headerbar.set_has_subtitle(false);
        headerbar.set_show_close_button(true);

        let add_button = gtk::Button::with_label("New profile");
        add_button.get_style_context().add_class("suggested-action");

        headerbar.pack_start(&add_button);

        // Returns the header and all of it's state
        Header { headerbar, add_button }
    }
}

impl Content {
    fn new() -> Content {
        // root container
        let root_container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // its contents
        let profile_list = ProfileList::new();
        let bottombar = BottomBar::new();

        root_container.pack_start(&profile_list.container, true, true, 2);
        root_container.pack_start(&bottombar.actionbar, false, true, 0);

        Content { root_container, profile_list, bottombar }
    }
}

impl ProfileList {
    fn new() -> ProfileList {
        // scrollbars and nice border/shadow
        let container = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        container.set_shadow_type(gtk::ShadowType::EtchedIn);
        container.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);

        // list (treeview) from the model
        let model = Arc::new(create_model().unwrap()); // MAY PANIC
        let list = gtk::TreeView::with_model(&*model);
        list.set_vexpand(true);
        
        container.add(&list);

        // stuff that i dont really understand
        ProfileList::create_columns(&model, &list);


        ProfileList { container, list, model }
    }

    fn create_columns(model: &Arc<gtk::ListStore>, list: &gtk::TreeView) {
        // connection type (wireless / wired)
        {
            let renderer = gtk::CellRendererPixbuf::new();
            let column = gtk::TreeViewColumn::new();
            column.pack_start(&renderer, true);
            column.add_attribute(&renderer, "icon-name", Columns::Connection as i32);
            column.set_sort_column_id(Columns::Connection as i32);
            list.append_column(&column);
        }

        // Interface (wlp2s0, eth0, ...)
        {
            let renderer = gtk::CellRendererText::new();
            let column = gtk::TreeViewColumn::new();
            column.pack_start(&renderer, true);
            column.set_title("Interface");
            column.add_attribute(&renderer, "text", Columns::Interface as i32);
            column.set_sort_column_id(Columns::Interface as i32);
            list.append_column(&column);
        }


        // ESSID
        {
            let renderer = gtk::CellRendererText::new();
            let column = gtk::TreeViewColumn::new();
            column.pack_start(&renderer, true);
            column.set_title("ESSID");
            column.add_attribute(&renderer, "text", Columns::ESSID as i32);
            column.set_sort_column_id(Columns::ESSID as i32);
            list.append_column(&column);
        }

        // status (active / enabled / disabled )
        {
            let renderer = gtk::CellRendererPixbuf::new();
            let column = gtk::TreeViewColumn::new();
            column.pack_start(&renderer, true);
            column.set_title("Status");
            column.add_attribute(&renderer, "icon-name", Columns::Status as i32);
            column.set_sort_column_id(Columns::Status as i32);
            list.append_column(&column);
        }

    }
}

impl BottomBar {
    fn new() -> BottomBar {
        // bar
        let actionbar = gtk::ActionBar::new();
        
        // buttons
        let switch_button = gtk::Button::with_label("Switch to");
        let enable_button = gtk::Button::with_label("Enable");
        let disable_button = gtk::Button::with_label("Disable");
        let edit_button = gtk::Button::with_label("Edit");
        let delete_button = gtk::Button::with_label("Delete");
        delete_button.get_style_context().add_class("destructive-action");

        actionbar.pack_start(&switch_button);
        actionbar.pack_start(&enable_button);
        actionbar.pack_start(&disable_button);
        actionbar.pack_start(&edit_button);
        actionbar.pack_end(&delete_button);

        BottomBar { 
            actionbar,
            switch_button,
            enable_button,
            disable_button,
            edit_button,
            delete_button,
        }
    }
}

fn create_model() -> Result<gtk::ListStore, std::io::Error> {
    let col_types: [glib::Type; 4] = [
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
        glib::Type::String,
    ];

    let col_indices = [0,1,2,3];

    let profiles = netctl::get_profiles()?;

    let store = gtk::ListStore::new(&col_types);

    for p in profiles {
        let values:[&dyn ToValue; 4] = [
            if p.connection == netctl::Connection::Wired {&"network-wired"} else {&"network-wireless"},
            &p.interface,
            &p.essid,
            &"zoom-in",
        ];
        store.set(&store.append(), &col_indices, &values);
    }

    Ok(store)
}

pub fn build_ui(application: &gtk::Application) {
    let app = App::new(application);
    app.window.show_all();
}