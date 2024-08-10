use std::path::Path;

use gdk_pixbuf::{InterpType, Pixbuf , PixbufLoader};

use gtk::{
    CellLayoutExt,
    CellRendererPixbuf,
    CellRendererText,
    ListStore,
    ListStoreExt,
    
    StaticType,
    ToValue,
    TreeIter,
    TreeModelExt,
    TreeSelectionExt ,
    TreeView,
    TreeViewColumn,
    TreeViewColumnExt,
    TreeViewExt,
    Type ,
    WidgetExt,
} ;

use id3::Tag;

const THUMBNAIL_COLUMN: u32 = 0;
const TITLE_COLUMN: u32 = 1;
const ARTIST_COLUMN: u32 = 2;
const ALBUM_COLUMN: u32 = 3;
const YEAR_COLUMN: u32 = 4;
const GENRE_COLUMN: u32 = 5;
const TRACK_COLUMN: u32 = 6;
const PATH_COLUMN: u32 = 7;
const PIXBUF_COLUMN: u32 = 8;
const IMAGE_SIZE: i32 = 64;
const THUMBNAIL_SIZE: i32 = 64;

pub struct Playlist {
    pub list_store: ListStore,
    pub tree_view: TreeView,
}

impl Playlist { 
    pub fn new() -> Self { 
        let model = ListStore::new(&[
            Pixbuf::static_type(),
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Pixbuf::static_type(),
        ]);

        let treeview = TreeView::new_with_model(&model);
        treeview.set_hexpand(true);
        treeview.set_vexpand(true);

        Playlist {
            list_store: model,
            tree_view: treeview,
        }
        
    }

    fn create_columns(treeview : &TreeView) { 
        Self::add_pixbuf_column(treeview , THUMBNAIL_COLUMN as i32 , Visible) ;

        Self::add_text_column(treeview, "Title", TITLE_COLUMN) ;
        Self::add_text_column(treeview, "Artist", ARTIST_COLUMN) ;
        Self::add_text_column(treeview, "Album", ALBUM_COLUMN) ;
        Self::add_text_column(treeview, "Genre", GENRE_COLUMN) ;
        Self::add_text_column(treeview, "Year", YEAR_COLUMN) ;
        Self::add_text_column(treeview, "Track", TRACK_COLUMN) ;
        Self::add_pixbuf_column(treeview , PIXBUF_COLUMN as i32 , Invisible) ;
    }
}