use std::path::Path;

use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};

use gio::Application;
use gtk::{
    ApplicationWindow, CellLayoutExt, CellRendererPixbuf, CellRendererText, DialogExt, FileChooserExt, FileFilterExt, ImageExt, ListStore, ListStoreExt, ListStoreExtManual, StaticType, ToValue, TreeIter, TreeModelExt, TreeSelectionExt, TreeView, TreeViewColumn, TreeViewColumnExt, TreeViewExt, Type, WidgetExt
};

use id3::Tag;
use std::path::PathBuf;
use gtk::Image;

use gtk::{FileChooserAction, FileChooserDialog, FileFilter};
use gtk_sys::{GTK_RESPONSE_CANCEL, GTK_RESPONSE_OK};

const RESPONSE_CANCEL: i32 = GTK_RESPONSE_CANCEL;
const RESPONSE_OK: i32 = GTK_RESPONSE_OK;

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
const INTERP_HYPER: InterpType = 3;

#[derive(PartialEq)]
enum Visibility {
    Visible,
    Invisible,
}
pub struct Playlist {
    pub list_store: ListStore,
    pub tree_view: TreeView,
}

use Visibility::*;

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

        self::Playlist::create_columns(&treeview);

        Playlist {
            list_store: model,
            tree_view: treeview,
        }
    }

    fn create_columns(treeview: &TreeView) {
        Self::add_pixbuf_column(treeview, THUMBNAIL_COLUMN as i32, Visible);

        Self::add_text_column(treeview, "Title", TITLE_COLUMN as i32);
        Self::add_text_column(treeview, "Artist", ARTIST_COLUMN as i32);
        Self::add_text_column(treeview, "Album", ALBUM_COLUMN as i32);
        Self::add_text_column(treeview, "Genre", GENRE_COLUMN as i32);
        Self::add_text_column(treeview, "Year", YEAR_COLUMN as i32);
        Self::add_text_column(treeview, "Track", TRACK_COLUMN as i32);
        Self::add_pixbuf_column(treeview, PIXBUF_COLUMN as i32, Invisible);
    }

    fn add_text_column(treeview: &TreeView, title: &str, column: i32) {
        let view_column = TreeViewColumn::new();
        view_column.set_title(title);
        let cell = CellRendererText::new();
        view_column.set_expand(true);
        view_column.pack_start(&cell, true);

        view_column.add_attribute(&cell, "text", column);
        treeview.append_column(&view_column);
    }

    fn add_pixbuf_column(treeview: &TreeView, column: i32, visibility: Visibility) {
        let view_column = TreeViewColumn::new();
        if visibility == Visible {
            let cell = CellRendererPixbuf::new();
            view_column.pack_start(&cell, true);
            view_column.add_attribute(&cell, "pixbuf", column);
        }

        treeview.append_column(&view_column);
    }

    pub fn view(&self) -> &TreeView {
        &self.tree_view
    }

    fn set_pixbuf(&self, row: &TreeIter, tag: &Tag) {
        if let Some(picture) = tag.pictures().next() {
            let pixbuf_loader = PixbufLoader::new();
            pixbuf_loader.set_size(IMAGE_SIZE, IMAGE_SIZE);
            pixbuf_loader.loader_write(&picture.data).unwrap();
            if let Some(pixbuf) = pixbuf_loader.get_pixbuf() {
                let thumbnail = pixbuf
                    .scale_simple(THUMBNAIL_SIZE, THUMBNAIL_SIZE, INTERP_HYPER)
                    .unwrap();
                self.list_store
                    .set_value(row, THUMBNAIL_COLUMN, &thumbnail.to_value());
                self.list_store
                    .set_value(row, PIXBUF_COLUMN, &pixbuf.to_value());
            }

            pixbuf_loader.close().unwrap();
        }
    }

    pub fn add(&self, path: &Path) {
        let filename = path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        let row = self.list_store.append();

        if let Ok(tag) = Tag::read_from_path(path) {
            let title = tag.title().unwrap_or(filename);
            let artist = tag.artist().unwrap_or("(no artist)");
            let album = tag.album().unwrap_or("(no album)");
            let genre = tag.genre().unwrap_or("(no genre)");
            let year = tag
                .year()
                .map(|y| y.to_string())
                .unwrap_or_else(|| "(no year)".to_string());
            let track = tag
                .track()
                .map(|t| t.to_string())
                .unwrap_or_else(|| "(no track)".to_string());
            let total_tracks = tag
                .total_tracks()
                .map(|t| t.to_string())
                .unwrap_or_else(|| "(no total tracks)".to_string());
            let track_value = format!("{} of {}", track, total_tracks);

            self.set_pixbuf(&row, &tag);
            self.list_store
                .set_value(&row, TITLE_COLUMN, &title.to_value());
            self.list_store
                .set_value(&row, ARTIST_COLUMN, &artist.to_value());
            self.list_store
                .set_value(&row, ALBUM_COLUMN, &album.to_value());
            self.list_store
                .set_value(&row, GENRE_COLUMN, &genre.to_value());
            self.list_store
                .set_value(&row, YEAR_COLUMN, &year.to_value());
            self.list_store
                .set_value(&row, TRACK_COLUMN, &track_value.to_value());
        } else {
            self.list_store
                .set_value(&row, TITLE_COLUMN, &filename.to_value());
        }

        let path = path.to_str().unwrap_or_default();
        self.list_store
            .set_value(&row, PATH_COLUMN, &path.to_value());
    }

    pub fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
        let mut file = None;
        let dialog = FileChooserDialog::new(
            Some("Select an MP3 file"),
            Some(parent),
            FileChooserAction::Open,
        );
        let filter = FileFilter::new();
        filter.add_mime_type("audio/mp3");
        filter.set_name("MP3 audio files");
        dialog.add_filter(&filter);
        dialog.add_button("Cancel", RESPONSE_CANCEL);
        dialog.add_button("Accept", RESPONSE_OK);
        let result = dialog.run();
        if result == RESPONSE_OK {
            file = dialog.get_filename();
        }

        dialog.destroy();
        file
    }

    pub fn remove_selected(&self) {
        let selection = self.tree_view.get_selection();
        if let Some((model, iter)) = selection.get_selected() {
            self.list_store.remove(&iter);
        }
    }

    pub fn pixbuf(&self) -> Option<Pixbuf> { 
        let selection = self.tree_view.get_selection() ;
        if let Some((_ , iter )) = selection.get_selected(){
            let value  = self.list_store.get_value(&iter, PIXBUF_COLUMN as i32) ;
            return value.get::<Pixbuf>() ;
        }
        None
    }

}



pub fn set_cover(cover: &Image, playlist: &Playlist) {
    cover.set_from_pixbuf(playlist.pixbuf().as_ref());
    cover.show();
}
