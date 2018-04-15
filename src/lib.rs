extern crate stripper_lib;

use std::io;
use std::path::Path;
use stripper_lib::{
    loop_over_files,
    parse_cmts,
    regenerate_comments,
    strip_comments,
};

#[derive(Clone, Copy, Debug)]
pub enum Library {
    Cairo,
    Gdk,
    GdkPixbuf,
    Glib,
    Gio,
    Gtk,
    Pango,
    SourceView,
    PangoCairo,
    GMime
}

fn docs(lib: Library) -> Option<&'static str> {
    match lib {
        Library::Glib => Some(include_str!("../glib/docs.md")),
        Library::Gio => Some(include_str!("../gio/docs.md")),
        Library::Cairo => Some(include_str!("../cairo/docs.md")),
        Library::Gdk => Some(include_str!("../gdk/docs.md")),
        Library::GdkPixbuf => Some(include_str!("../gdk-pixbuf/docs.md")),
        Library::Gtk => Some(include_str!("../gtk/docs.md")),
        Library::GMime => Some(include_str!("../gmime/docs.md")),
        _ => None,
    }
}

fn vendor_docs(lib: Library) -> Option<&'static str> {
    match lib {
        Library::Glib => Some(include_str!("../glib/vendor.md")),
        Library::Gio => Some(include_str!("../gio/vendor.md")),
        Library::Pango => Some(include_str!("../pango/vendor.md")),
        Library::Gdk => Some(include_str!("../gdk/vendor.md")),
        Library::GdkPixbuf => Some(include_str!("../gdk-pixbuf/vendor.md")),
        Library::Gtk => Some(include_str!("../gtk/vendor.md")),
        Library::SourceView => Some(include_str!("../gtksource/vendor.md")),
        Library::PangoCairo => Some(include_str!("../pangocairo/vendor.md")),
        Library::GMime => Some(include_str!("../gmime/vendor.md")),
        _ => None,
    }
}

/// Embeds the docs.
///
/// `path` is the root directory to process.
///
/// `ignores` is the list of files to skip (relative to `path`).
pub fn embed<P: AsRef<Path>>(library: Library, path: P, ignores: &[&str]) {
    if let Some(docs) = docs(library) {
        do_embed(docs, path.as_ref(), ignores);
    }
    if let Some(docs) = vendor_docs(library) {
        do_embed(docs, path.as_ref(), ignores);
    }
}

fn do_embed(docs: &str, path: &Path, ignores: &[&str]) {
    let mut infos = parse_cmts(docs.lines(), true);
    loop_over_files(path, &mut |w, s| regenerate_comments(w, s, &mut infos, true, true),
        &ignores, false);
}

/// Remove any doc comments.
///
/// `path` is the root directory to process.
///
/// `ignores` is the list of files to skip (relative to `path`).
pub fn purge<P: AsRef<Path>>(path: P, ignores: &[&str]) {
    loop_over_files(path.as_ref(), &mut |w, s| strip_comments(w, s, &mut io::sink(), true),
        &ignores, false);
}
