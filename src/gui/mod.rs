#![allow(clippy::clippy::redundant_field_names)]

use std::{io, path::PathBuf};

use relm::{Component, Widget, init};
use gtk::Orientation::Vertical;
use gtk::prelude::*;
use relm_derive::{Msg, widget};

use crate::netctl;

// Columns for the TreeView
struct Column {
    pub type_: glib::Type,
    pub title: &'static str,
    pub renderer: Renderer,
}

pub enum Renderer {
    Text,
    Pixbuf,
}

const NCOLS: usize = 4;
const COLUMNS: [Column; NCOLS] = [
    Column {type_: glib::Type::String, title: "", renderer: Renderer::Pixbuf},
    Column {type_: glib::Type::String, title: "Interface", renderer: Renderer::Text},
    Column {type_: glib::Type::String, title: "ESSID", renderer: Renderer::Text},
    Column {type_: glib::Type::String, title: "Status", renderer: Renderer::Pixbuf},
];

const COLUMN_TYPES: [glib::Type; NCOLS] = {
    let mut array = [glib::Type::String; NCOLS];

    let mut i = 0;
    while i < NCOLS {
        array[i] = COLUMNS[i].type_;
        i += 1;
    }

    array
};

const COLUMN_INDICES: [u32; NCOLS] = {
    let mut array = [0 as u32; NCOLS];

    let mut i = 0;
    
    while i < NCOLS {
        array[i] = i as u32;
        i += 1;
    }

    array
};



////////
// HEADERBAR
////////
#[derive(Msg)]
pub enum HeaderMsg {
    Add,
}

pub struct HeaderModel {
    primary_menu: Component<PrimaryMenu>,
}

// Headerbar
#[widget]
impl Widget for Header {
    fn model() -> HeaderModel {
        let primary_menu = init::<PrimaryMenu>(()).expect("PrimaryMenu");
        HeaderModel { primary_menu }
    }

    fn update(&mut self, event: HeaderMsg) {
        match event {
            HeaderMsg::Add => println!("add"),
        }
    }

    // do stuff with gtk-rs that relm can not do
    fn init_view(&mut self) {
        self.add_button.get_style_context().add_class("suggested-action");
        gtk::MenuButtonExt::set_direction(&self.primary_menu_button, gtk::ArrowType::None);
    }

    view! {
        gtk::HeaderBar {
            title: Some("Netti"),
            show_close_button: true,
            has_subtitle: false,

            #[name="add_button"]
            gtk::Button {
                clicked => HeaderMsg::Add,
                label: "Add profile",
            },
            #[name="primary_menu_button"]
            gtk::MenuButton {
                use_popover: true,
                popover: Some(self.model.primary_menu.widget()),
                child: {
                    pack_type: gtk::PackType::End,
                },

            }
        }
    }
}

#[derive(Msg)]
pub enum PrimaryMenuMsg {
    About,
}

// Primary menu popover
#[widget]
impl Widget for PrimaryMenu {
    fn model() {}
    fn update(&mut self, event: PrimaryMenuMsg) {
        match event {
            PrimaryMenuMsg::About => println!("about"),
        }
    }

    view! {
        gtk::Popover {
            gtk::Box {
                spacing: 1,
                orientation: Vertical,
                border_width: 5,
                gtk::Button {
                    label: "works!",
                    clicked => PrimaryMenuMsg::About,
                },
            },
        },
    }
}

//////
// MAIN WINDOW
//////

#[derive(Msg)]
pub enum WinMsg {
    Quit,
}

pub struct WinModel {
    header: Component<Header>,
    list_store: gtk::ListStore,
}

#[widget]
impl Widget for Win {
    fn model() -> WinModel {
        // append custom icons to default icon theme
        let icon_theme = gtk::IconTheme::get_default().unwrap_or_default();
        icon_theme.append_search_path(PathBuf::from("./resources/icons/"));
        gtk::IconTheme::load_icon(&icon_theme, "network-ppp-symbolic", 16, gtk::IconLookupFlags::empty()).unwrap();


        let header = init::<Header>(()).expect("Header");
        let res = create_and_fill_list_store();

        match res {
            Ok(list_store) => WinModel { header, list_store },
            Err(e) => panic!("vittu: {}", e),
        }
    }

    fn update(&mut self, event:WinMsg) {
        match event {
            WinMsg::Quit => gtk::main_quit(),
        }
    }

    fn init_view(&mut self) {
        // cosmetic options for "Delete"-button
        self.delete_button.get_style_context().add_class("destructive-action");
        
        // create the columns for the treeview
        create_columns(&self.tree_view);

    }
    
    view! {
        #[name="window"]
        gtk::Window {
            title: "Title",
            property_default_width: 100,
            property_default_height: 500,
            titlebar: Some(self.model.header.widget()),

            #[name="app"]
            gtk::Box {
                orientation: Vertical,
                gtk::ScrolledWindow {
                    shadow_type: gtk::ShadowType::EtchedIn,
                    property_vscrollbar_policy: gtk::PolicyType::Automatic,
                    property_hscrollbar_policy: gtk::PolicyType::Never,
                    #[name="tree_view"]
                    gtk::TreeView {
                        vexpand: true,
                        model: Some(&self.model.list_store),
                    }
                },
                #[name="actionbar"]
                gtk::ActionBar {
                    #[name="switch_button"]
                    gtk::Button {
                        label: "Switch to",
                    },
                    #[name="enable_button"]
                    gtk::Button {
                        label: "Enable",
                    },
                    #[name="disable_button"]
                    gtk::Button {
                        label: "Disable",
                    },
                    #[name="delete_button"]
                    gtk::Button {
                        label: "Delete",
                        child: {
                            pack_type: gtk::PackType::End,
                        }
                    },
                },
            },
            delete_event(_,_) => (WinMsg::Quit, Inhibit(false)),
        }
    }
}


fn create_and_fill_list_store() -> Result<gtk::ListStore, io::Error> {
    let list_store = gtk::ListStore::new(&COLUMN_TYPES);

    for profile in netctl::profile_iter()? {
        list_store.set(&list_store.append(), &COLUMN_INDICES, &[
            &profile.connection.icon_name(),
            &profile.interface,
            &profile.essid,
            &"on", // fix this
        ]);
    }

    Ok(list_store)
}

// creates columns for the TreeView
fn create_columns(tree_view: &gtk::TreeView) {
    
    for (i, col) in COLUMNS.iter().enumerate() {
        
        let tree_view_col = gtk::TreeViewColumn::new();
        tree_view_col.set_title(col.title);

        match col.renderer {
            Renderer::Text => {
                let renderer = gtk::CellRendererText::new();
                tree_view_col.pack_start(&renderer, true);
                tree_view_col.add_attribute(&renderer, "text", i as i32);
            },
            Renderer::Pixbuf => {
                let renderer = gtk::CellRendererPixbuf::new();
                tree_view_col.pack_start(&renderer, true);
                tree_view_col.add_attribute(&renderer, "icon-name", i as i32);
            },
        };

        tree_view.append_column(&tree_view_col);

    }
}

pub fn run() {
    Win::run(()).unwrap();
}