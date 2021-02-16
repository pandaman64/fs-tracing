//! fs-tracing is a drop-in replacement for [`std::fs`](std::fs).
//!
//! # Errors

// CR pandaman: implement error wrapper
// CR pandaman: consider whether to #[instrument] non-fallible functions such as builders.
// CR pandaman: implement nightly only functions?
// CR pandaman: propose that #[instrument] can take parent parameter
// https://github.com/tokio-rs/tracing/issues/879
// CR pandaman: report to the rust-analyzer team the following:
// 1. autocompleting a trait method signature removes attributes (such as #[instrument])
// 2. autocompletion should show methods from the implementing trait

#![deny(unsafe_code)]
// CR pandaman: promote to deny
#![warn(
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rustdoc,
    trivial_casts
)]
// std does not have ones.
#![allow(clippy::new_without_default, clippy::len_without_is_empty)]

mod error;

use std::{
    ffi, fmt, fs, io,
    path::{Path, PathBuf},
    process, time,
};
use tracing::{debug, instrument};

/// Wrapper for [`fs::DirBuilder`](std::fs::DirBuilder).
pub struct DirBuilder {
    inner: fs::DirBuilder,
}

impl fmt::Debug for DirBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

// CR pandaman: implement DirBuilderExt for DirBuilder (unix only)

impl DirBuilder {
    /// Wrapper for [`DirBuilder::new`](std::fs::DirBuilder::new).
    pub fn new() -> Self {
        Self {
            inner: fs::DirBuilder::new(),
        }
    }

    /// Wrapper for [`DirBuilder::recursive`](std::fs::DirBuilder::recursive).
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.inner.recursive(recursive);
        self
    }

    /// Wrapper for [`DirBuilder::create`](std::fs::DirBuilder::create).
    pub fn create<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        // CR pandaman: consult doc for tracing::instrument to mimic the ordinary ordering
        #[instrument(skip(this), fields(self = ?this, path = ?path))]
        fn create(this: &DirBuilder, path: &Path) -> io::Result<()> {
            this.inner.create(path).map_err(error::Error::wrap_std)
        }

        create(self, path.as_ref())
    }
}

/// Wrapper for [`fs::DirEntry`](std::fs::DirEntry).
pub struct DirEntry {
    inner: fs::DirEntry,
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

// CR pandaman: implement DirEntryExt for DirEntry (unix only)

impl DirEntry {
    /// Wrapper for [`DirEntry::path`](std::fs::DirEntry::path).
    pub fn path(&self) -> PathBuf {
        self.inner.path()
    }

    /// Wrapper for [`DirEntry::metadata`](std::fs::DirEntry::metadata).
    #[instrument]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.inner
            .metadata()
            .map(|inner| Metadata { inner })
            .map_err(error::Error::wrap_std)
    }

    /// Wrapper for [`DirEntry::file_type`](std::fs::DirEntry::file_type).
    #[instrument]
    pub fn file_type(&self) -> io::Result<FileType> {
        self.inner
            .file_type()
            .map(|inner| FileType { inner })
            .map_err(error::Error::wrap_std)
    }

    /// Wrapper for [`DirEntry::file_name`](std::fs::DirEntry::file_name).
    pub fn file_name(&self) -> ffi::OsString {
        self.inner.file_name()
    }
}

/// Wrapper for [`fs::File`](std::fs::File).
pub struct File {
    inner: fs::File,
}

// CR pandaman: implement extension traits

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<File> for process::Stdio {
    fn from(file: File) -> Self {
        Self::from(file.inner)
    }
}

impl io::Read for File {
    #[instrument]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf).map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.inner
            .read_vectored(bufs)
            .map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_to_end(buf).map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.inner
            .read_to_string(buf)
            .map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.inner.read_exact(buf).map_err(error::Error::wrap_std)
    }
}

impl io::Read for &File {
    #[instrument]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&self.inner).read(buf).map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        (&self.inner)
            .read_vectored(bufs)
            .map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        (&self.inner)
            .read_to_end(buf)
            .map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        (&self.inner)
            .read_to_string(buf)
            .map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (&self.inner)
            .read_exact(buf)
            .map_err(error::Error::wrap_std)
    }
}

impl io::Seek for File {
    #[instrument]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.inner.seek(pos).map_err(error::Error::wrap_std)
    }
}

impl io::Seek for &File {
    #[instrument]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        (&self.inner).seek(pos).map_err(error::Error::wrap_std)
    }
}

impl io::Write for File {
    #[instrument]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf).map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush().map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.inner
            .write_vectored(bufs)
            .map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.inner.write_all(buf).map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.inner.write_fmt(fmt).map_err(error::Error::wrap_std)
    }
}

impl io::Write for &File {
    #[instrument]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&self.inner).write(buf).map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn flush(&mut self) -> io::Result<()> {
        (&self.inner).flush().map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        (&self.inner)
            .write_vectored(bufs)
            .map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        (&self.inner).write_all(buf).map_err(error::Error::wrap_std)
    }

    #[instrument]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        (&self.inner).write_fmt(fmt).map_err(error::Error::wrap_std)
    }
}

impl File {
    /// Wrapper for [`File::open`](std::fs::File::open).
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        #[instrument]
        fn open(path: &Path) -> io::Result<File> {
            fs::File::open(path)
                .map(|inner| File { inner })
                .map_err(error::Error::wrap_std)
        }

        open(path.as_ref())
    }

    /// Wrapper for [`File::create`](std::fs::File::create).
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        #[instrument]
        fn create(path: &Path) -> io::Result<File> {
            fs::File::create(path)
                .map(|inner| File { inner })
                .map_err(error::Error::wrap_std)
        }

        create(path.as_ref())
    }

    /// Wrapper for [`File::sync_all`](std::fs::File::sync_all).
    #[instrument]
    pub fn sync_all(&self) -> io::Result<()> {
        self.inner.sync_all().map_err(error::Error::wrap_std)
    }

    /// Wrapper for [`File::sync_data`](std::fs::File::sync_data).
    #[instrument]
    pub fn sync_data(&self) -> io::Result<()> {
        self.inner.sync_data().map_err(error::Error::wrap_std)
    }

    /// Wrapper for [`File::set_len`](std::fs::File::set_len),
    #[instrument]
    pub fn set_len(&self, size: u64) -> io::Result<()> {
        self.inner.set_len(size).map_err(error::Error::wrap_std)
    }

    /// Wrapper for [`File::metadata`](std::fs::File::metadata).
    #[instrument]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.inner
            .metadata()
            .map(|inner| Metadata { inner })
            .map_err(error::Error::wrap_std)
    }

    /// Wrapper for [`File::try_clone`](std::fs::File::try_clone).
    #[instrument]
    pub fn try_clone(&self) -> io::Result<File> {
        self.inner
            .try_clone()
            .map(|inner| File { inner })
            .map_err(error::Error::wrap_std)
    }

    /// Wrapper for [`File::set_permissions`](std::fs::File::set_permissions).
    #[instrument]
    pub fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        self.inner
            .set_permissions(perm.inner)
            .map_err(error::Error::wrap_std)
    }
}

/// Wrapper for [`fs::FileType`](std::fs::FileType).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FileType {
    inner: fs::FileType,
}

impl fmt::Debug for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

// CR pandaman: implement FileTypeExt for FileType

impl FileType {
    /// Wrapper for [`FileType::is_dir`](std::fs::FileType::is_dir).
    pub fn is_dir(&self) -> bool {
        self.inner.is_dir()
    }

    /// Wrapper for [`FileType::is_file`](std::fs::FileType::is_file).
    pub fn is_file(&self) -> bool {
        self.inner.is_file()
    }

    /// Wrapper for [`FileType::is_symlink`](std::fs::FileType::is_symlink).
    pub fn is_symlink(&self) -> bool {
        self.inner.is_symlink()
    }
}

/// Wrapper for [`fs::Metadata`](std::fs::Metadata).
#[derive(Clone)]
pub struct Metadata {
    inner: fs::Metadata,
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

// CR pandaman: implement MetadataExt for Metadata

impl Metadata {
    /// Wrapper for [`Metadata::file_type`](std::fs::Metadata::file_type).
    pub fn file_type(&self) -> FileType {
        FileType {
            inner: self.inner.file_type(),
        }
    }

    /// Wrapper for [`Metadata::is_dir`](std::fs::Metadata::is_dir).
    pub fn is_dir(&self) -> bool {
        self.inner.is_dir()
    }

    /// Wrapper for [`Metadata::is_file`](std::fs::Metadata::is_file).
    pub fn is_file(&self) -> bool {
        self.inner.is_file()
    }

    /// Wrapper for [`Metadata::len`](std::fs::Metadata::len).
    pub fn len(&self) -> u64 {
        self.inner.len()
    }

    /// Wrapper for [`Metadata::permissions`](std::fs::Metadata::permissions).
    pub fn permissions(&self) -> Permissions {
        Permissions {
            inner: self.inner.permissions(),
        }
    }

    /// Wrapper for [`Metadata::modified`](std::fs::Metadata::modified).
    #[instrument]
    pub fn modified(&self) -> io::Result<time::SystemTime> {
        self.inner.modified().map_err(error::Error::wrap_std)
    }

    /// Wrapper for [`Metadata::accessed`](std::fs::Metadata::accessed).
    #[instrument]
    pub fn accessed(&self) -> io::Result<time::SystemTime> {
        self.inner.accessed().map_err(error::Error::wrap_std)
    }

    /// Wrapper for [`Metadata::created`](std::fs::Metadata::created).
    #[instrument]
    pub fn created(&self) -> io::Result<time::SystemTime> {
        self.inner.created().map_err(error::Error::wrap_std)
    }
}

/// Wrapper for [`fs::OpenOptions`](std::fs::OpenOptions).
#[derive(Clone)]
pub struct OpenOptions {
    inner: fs::OpenOptions,
}

impl fmt::Debug for OpenOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

// CR pandaman: implement OpenOptionsExt for OpenOptions

impl OpenOptions {
    /// Wrapper for [`OpenOptions::new`](std::fs::OpenOptions::new).
    pub fn new() -> Self {
        Self {
            inner: fs::OpenOptions::new(),
        }
    }

    /// Wrapper for [`OpenOptions::read`](std::fs::OpenOptions::read).
    pub fn read(&mut self, read: bool) -> &mut Self {
        self.inner.read(read);
        self
    }

    /// Wrapper for [`OpenOptions::write`](std::fs::OpenOptions::write).
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.inner.write(write);
        self
    }

    /// Wrapper for [`OpenOptions::append`](std::fs::OpenOptions::append).
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.inner.append(append);
        self
    }

    /// Wrapper for [`OpenOptions::truncate`](std::fs::OpenOptions::truncate).
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.inner.truncate(truncate);
        self
    }

    /// Wrapper for [`OpenOptions::create`](std::fs::OpenOptions::create).
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.inner.create(create);
        self
    }

    /// Wrapper for [`OpenOptions::create_new`](std::fs::OpenOptions::create_new).
    pub fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.inner.create_new(create_new);
        self
    }

    /// Wrapper for [`OpenOptions::open`](std::fs::OpenOptions::open).
    pub fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        #[instrument(skip(this), fields(self = ?this, path = ?path))]
        fn open(this: &OpenOptions, path: &Path) -> io::Result<File> {
            this.inner
                .open(path)
                .map(|inner| File { inner })
                .map_err(error::Error::wrap_std)
        }

        open(self, path.as_ref())
    }
}

/// Wrapper for [`fs::Permissions`](std::fs::Permissions).
#[derive(Clone, PartialEq, Eq)]
pub struct Permissions {
    inner: fs::Permissions,
}

impl fmt::Debug for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

// CR pandaman: implement PermissionExt for Permissions

impl Permissions {
    /// Wrapper for [`Permissions::readonly`](std::fs::Permissions::readonly).
    pub fn readonly(&self) -> bool {
        self.inner.readonly()
    }

    /// Wrapper for [`Permissions::set_readonly`](std::fs::Permissions::set_readonly).
    pub fn set_readonly(&mut self, readonly: bool) {
        self.inner.set_readonly(readonly)
    }
}

/// Wrapper for [`fs::ReadDir`](std::fs::ReadDir).
pub struct ReadDir {
    inner: fs::ReadDir,
    // CR pandaman: consider adding a Span context here
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    #[instrument]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|result| {
            result
                .map(|inner| DirEntry { inner })
                .map_err(error::Error::wrap_std)
        })
    }
}

/// Wrapper for [`fs::canonicalize`](std::fs::canonicalize).
pub fn canonicalize<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    #[instrument]
    fn canonicalize(path: &Path) -> io::Result<PathBuf> {
        fs::canonicalize(path).map_err(error::Error::wrap_std)
    }

    canonicalize(path.as_ref())
}

/// Wrapper for [`fs::copy`](std::fs::copy).
pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
    #[instrument]
    fn copy(from: &Path, to: &Path) -> io::Result<u64> {
        // CR pandaman: I don't know why copying between the same file can result in a truncated file
        if from == to {
            // CR pandaman: consider the appropriate log level
            debug!("`from' and `to' point to the same file");
        }

        fs::copy(from, to).map_err(error::Error::wrap_std)
    }

    copy(from.as_ref(), to.as_ref())
}

/// Wrapper for [`fs::create_dir`](std::fs::create_dir).
pub fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    #[instrument]
    fn create_dir(path: &Path) -> io::Result<()> {
        fs::create_dir(path).map_err(error::Error::wrap_std)
    }

    create_dir(path.as_ref())
}

/// Wrapper for [`fs::create_dir_all`](std::fs::create_dir_all).
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    #[instrument]
    fn create_dir_all(path: &Path) -> io::Result<()> {
        fs::create_dir_all(path).map_err(error::Error::wrap_std)
    }

    create_dir_all(path.as_ref())
}

/// Wrapper for [`fs::hard_link`](std::fs::hard_link).
pub fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    #[instrument]
    fn hard_link(original: &Path, link: &Path) -> io::Result<()> {
        fs::hard_link(original, link).map_err(error::Error::wrap_std)
    }

    hard_link(original.as_ref(), link.as_ref())
}

/// Wrapper for [`fs::metadata`](std::fs::metadata).
pub fn metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
    #[instrument]
    fn metadata(path: &Path) -> io::Result<Metadata> {
        fs::metadata(path)
            .map(|inner| Metadata { inner })
            .map_err(error::Error::wrap_std)
    }

    metadata(path.as_ref())
}

/// Wrapper for [`fs::read`](std::fs::read).
pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    #[instrument]
    fn read(path: &Path) -> io::Result<Vec<u8>> {
        fs::read(path).map_err(error::Error::wrap_std)
    }

    read(path.as_ref())
}

/// Wrapper for [`fs::read_dir`](std::fs::read_dir).
pub fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir> {
    #[instrument]
    fn read_dir(path: &Path) -> io::Result<ReadDir> {
        fs::read_dir(path)
            .map(|inner| ReadDir { inner })
            .map_err(error::Error::wrap_std)
    }

    read_dir(path.as_ref())
}

/// Wrapper for [`fs::read_link`](std::fs::read_link).
pub fn read_link<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    #[instrument]
    fn read_link(path: &Path) -> io::Result<PathBuf> {
        fs::read_link(path).map_err(error::Error::wrap_std)
    }

    read_link(path.as_ref())
}

/// Wrapper for [`fs::read_to_string`](std::fs::read_to_string).
pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    #[instrument]
    fn read_to_string(path: &Path) -> io::Result<String> {
        fs::read_to_string(path).map_err(error::Error::wrap_std)
    }

    read_to_string(path.as_ref())
}

/// Wrapper for [`fs::remove_dir`](std::fs::remove_dir).
pub fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    #[instrument]
    fn remove_dir(path: &Path) -> io::Result<()> {
        fs::remove_dir(path).map_err(error::Error::wrap_std)
    }

    remove_dir(path.as_ref())
}

/// Wrapper for [`fs::remove_dir_all`](std::fs::remove_dir_all).
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    #[instrument]
    fn remove_dir_all(path: &Path) -> io::Result<()> {
        fs::remove_dir_all(path).map_err(error::Error::wrap_std)
    }

    remove_dir_all(path.as_ref())
}

/// Wrapper for [`fs::remove_file`](std::fs::remove_file).
pub fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    #[instrument]
    fn remove_file(path: &Path) -> io::Result<()> {
        fs::remove_file(path).map_err(error::Error::wrap_std)
    }

    remove_file(path.as_ref())
}

/// Wrapper for [`fs::rename`](std::fs::rename).
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    #[instrument]
    fn rename(from: &Path, to: &Path) -> io::Result<()> {
        fs::rename(from, to).map_err(error::Error::wrap_std)
    }

    rename(from.as_ref(), to.as_ref())
}

/// Wrapper for [`fs::set_permissions`](std::fs::set_permissions).
pub fn set_permissions<P: AsRef<Path>>(path: P, perm: Permissions) -> io::Result<()> {
    #[instrument]
    fn set_permissions(path: &Path, perm: Permissions) -> io::Result<()> {
        fs::set_permissions(path, perm.inner).map_err(error::Error::wrap_std)
    }

    set_permissions(path.as_ref(), perm)
}

/// Wrapper for [`fs::symlink_metadata`](std::fs::symlink_metadata).
pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
    #[instrument]
    fn symlink_metadata(path: &Path) -> io::Result<Metadata> {
        fs::symlink_metadata(path)
            .map(|inner| Metadata { inner })
            .map_err(error::Error::wrap_std)
    }

    symlink_metadata(path.as_ref())
}

/// Wrapper for [`fs::write`](std::fs::write).
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    #[instrument]
    fn write(path: &Path, contents: &[u8]) -> io::Result<()> {
        fs::write(path, contents).map_err(error::Error::wrap_std)
    }

    write(path.as_ref(), contents.as_ref())
}
