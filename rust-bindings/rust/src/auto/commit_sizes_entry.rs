// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2020_1", feature = "dox"))]
use glib::translate::*;
use ostree_sys;
#[cfg(any(feature = "v2020_1", feature = "dox"))]
use ObjectType;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CommitSizesEntry(Boxed<ostree_sys::OstreeCommitSizesEntry>);

    match fn {
        copy => |ptr| ostree_sys::ostree_commit_sizes_entry_copy(mut_override(ptr)),
        free => |ptr| ostree_sys::ostree_commit_sizes_entry_free(ptr),
        get_type => || ostree_sys::ostree_commit_sizes_entry_get_type(),
    }
}

impl CommitSizesEntry {
    #[cfg(any(feature = "v2020_1", feature = "dox"))]
    pub fn new(checksum: &str, objtype: ObjectType, unpacked: u64, archived: u64) -> Option<CommitSizesEntry> {
        unsafe {
            from_glib_full(ostree_sys::ostree_commit_sizes_entry_new(checksum.to_glib_none().0, objtype.to_glib(), unpacked, archived))
        }
    }
}
