// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::fd::{FromRawFd, IntoRawFd};
use std::path::Path;

use libc::c_char;

// From this library
use crate::core::errors::ScriptError;
use crate::core::partition::PartitionList;
use crate::fdisk::Fdisk;

use crate::ffi_utils;

/// `sfdisk`-compatible script.
#[derive(Debug)]
#[repr(transparent)]
pub struct Script<'fdisk> {
    pub(crate) inner: *mut libfdisk::fdisk_script,
    _marker: PhantomData<&'fdisk Fdisk<'fdisk>>,
}

impl<'fdisk> Script<'fdisk> {
    #[doc(hidden)]
    #[allow(dead_code)]
    /// Increments the script's reference counter.
    pub(crate) fn incr_ref_counter(&mut self) {
        unsafe { libfdisk::fdisk_ref_script(self.inner) }
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Decrements the script's reference counter.
    pub(crate) fn decr_ref_counter(&mut self) {
        unsafe { libfdisk::fdisk_unref_script(self.inner) }
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Wraps a boxed raw `libfdisk::fdisk_script` pointer in a safe reference.
    pub(crate) unsafe fn ref_from_boxed_ptr<'a>(
        ptr: Box<*mut libfdisk::fdisk_script>,
    ) -> (*mut *mut libfdisk::fdisk_script, &'a Self) {
        let raw_ptr = Box::into_raw(ptr);
        let entry_ref = unsafe { &*(raw_ptr as *const _ as *const Self) };

        (raw_ptr, entry_ref)
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Wraps a boxed raw `libfdisk::fdisk_script` pointer in a safe reference.
    pub(crate) unsafe fn mut_from_boxed_ptr<'a>(
        ptr: Box<*mut libfdisk::fdisk_script>,
    ) -> (*mut *mut libfdisk::fdisk_script, &'a mut Self) {
        let raw_ptr = Box::into_raw(ptr);
        let entry_ref = unsafe { &mut *(raw_ptr as *mut Self) };

        (raw_ptr, entry_ref)
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    /// Creates a new `Script` instance.
    pub(crate) fn new(_: &'fdisk Fdisk, inner: *mut libfdisk::fdisk_script) -> Script<'fdisk> {
        log::debug!("Script::new creating a new `Script` instance");

        Self {
            inner,
            _marker: PhantomData,
        }
    }

    #[doc(hidden)]
    /// Reads and parses a script file's content.
    fn read_file(ptr: &mut Self, file: &mut File) -> Result<(), ScriptError> {
        let file_stream = ffi_utils::read_only_c_file_stream_from(file).map_err(|e| {
            let err_msg = format!("failed to read script file {e}");
            ScriptError::IoError(err_msg)
        })?;

        // Rust complains with note: expected raw pointer `*mut _IO_FILE` found raw pointer `*mut FILE`
        // however from glibc/libio/bits/types/FILE.h we have the following definition
        // typedef struct _IO_FILE FILE
        // the `as *mut _` conversion solves the misidentified type mismatch.
        let result = unsafe { libfdisk::fdisk_script_read_file(ptr.inner, file_stream as *mut _) };

        match result {
            0 => {
                log::debug!("Script::read_file file read");

                Ok(())
            }
            code => {
                let err_msg = "failed to read file".to_owned();
                log::debug!("Script::read_file {}. libfdisk::fdisk_script_read_file returned error code: {:?}", err_msg, code);

                Err(ScriptError::Read(err_msg))
            }
        }
    }

    /// Imports the file at `file_path`.
    pub fn import_file<T>(&mut self, file_path: T) -> Result<(), ScriptError>
    where
        T: AsRef<Path>,
    {
        let file_path = file_path.as_ref();
        log::debug!("Script::import_file importing file: {:?}", file_path);

        let mut file = OpenOptions::new().read(true).open(file_path).map_err(|e| {
            let err_msg = format!("failed to open file {} {e}", file_path.display());
            ScriptError::IoError(err_msg)
        })?;

        Self::read_file(self, &mut file)
    }

    /// Imports from an open [`File`].
    pub fn import_stream(&mut self, file: &mut File) -> Result<(), ScriptError> {
        log::debug!("Script::import_stream importing from file");

        Self::read_file(self, file)
    }

    /// Sets a callback function (`fn_read_line`) for reading characters from a [`File`] stream, and storing them in
    /// a buffer. Reading should stop if a newline character is found (in which case the buffer
    /// will contain that newline character), or if we reach the end of the file.
    ///
    /// `fn_read_line` should return the number of characters read if the operation succeeds,
    /// otherwise it should emit an I/O Error.
    pub fn set_custom_read_line<R>(&mut self, fn_read_line: R) -> Result<(), ScriptError>
    where
        R: FnMut(&mut File, &mut [c_char]) -> io::Result<usize> + 'static,
    {
        #[doc(hidden)]
        /// Callback function used by the `libfdisk::fdisk_script_read_file` and
        /// `libfdisk::fdisk_script_read_line` functions to read a file.
        unsafe extern "C" fn read_callback<R>(
            script: *mut libfdisk::fdisk_script,
            buf: *mut libc::c_char,
            count: usize,
            file_stream: *mut libfdisk::FILE,
        ) -> *mut libc::c_char
        where
            R: FnMut(&mut File, &mut [c_char]) -> io::Result<usize> + 'static,
        {
            // Build a temporary Rust `File` object from a C FILE struct.
            let rc = unsafe { libc::fileno(file_stream as *mut _) };
            let mut file = match rc {
                -1 =>
                    panic!("Script::read_callback failed to obtain a file file descriptor from the FILE struct passed to the custom file reader function"),
                fd =>
                    unsafe { File::from_raw_fd(fd) },
            };

            // Create a buffer to read data into.
            let mut buffer: Vec<libc::c_char> = vec![0; count - 1];

            // Rebuild the `fn_read_line` function from the c_void pointer passed as user data.
            let mut user_data_ptr = MaybeUninit::<*mut libc::c_void>::zeroed();
            unsafe {
                user_data_ptr.write(libfdisk::fdisk_script_get_userdata(script));
            }

            // Since we set the custom reader function ourselves user_data is never NULL.
            let user_data = unsafe { user_data_ptr.assume_init() };
            let read = &mut *(user_data as *mut R);

            match read(&mut file, buffer.as_mut_slice()) {
                Ok(total_read) => {
                    // Add terminating null character.
                    buffer.push(0);

                    // Copy Rust buffer -> C buf
                    let res = unsafe { libc::strncpy(buf, buffer.as_ptr(), total_read + 1) };

                    // Release the borrowed file descriptor without closing it.
                    let _ = file.into_raw_fd();

                    res
                }
                Err(e) => {
                    log::debug!("Script::read_callback error while reading file. {:?}", e);

                    // Release the borrowed file descriptor without closing it.
                    let _ = file.into_raw_fd();

                    std::ptr::null_mut()
                }
            }
        }

        // Moving the closure to the heap with `Box::new`, to live there for some unknown period of
        // time.  Then, call `Box::into_raw` on it, to get a raw pointer to the closure, and
        // prevent the memory it uses from being deallocated.
        let user_data = Box::into_raw(Box::new(fn_read_line));

        let result =
            unsafe { libfdisk::fdisk_script_set_userdata(self.inner, user_data as *mut _) };

        match result {
            0 => {
                log::debug!(
                    "Script::set_custom_read_line set custom file reader function as user data"
                );

                let result = unsafe {
                    libfdisk::fdisk_script_set_fgets(self.inner, Some(read_callback::<R>))
                };

                match result {
                    0 => {
                        log::debug!("Script::set_custom_read_line custom reader function set");

                        // FIXME the callback function is long lived. If the function is called
                        // several times, we risk a substantial memory leak until the end of the program,
                        // since `user_data` is never released between calls.

                        Ok(())
                    }
                    code => {
                        let err_msg =
                            "failed to set custom file reader callback function".to_owned();
                        log::debug!("Script::set_custom_read_line {}. libfdisk::fdisk_script_set_fgets returned error code: {:?}", err_msg, code);

                        // Deallocate closure on the heap.
                        let _ = unsafe { Box::from_raw(user_data) };

                        Err(ScriptError::Config(err_msg))
                    }
                }
            }
            code => {
                let err_msg = "failed to set custom file reader function as user data".to_owned();
                log::debug!("Script::set_custom_read_line {}. libfdisk::fdisk_script_set_userdata returned error code: {:?}", err_msg, code);

                // Deallocate closure on the heap.
                let _ = unsafe { Box::from_raw(user_data) };

                Err(ScriptError::Config(err_msg))
            }
        }
    }

    /// Pulls a line from a [`File`] into the specified buffer.
    pub fn read_line(
        &mut self,
        file: &mut File,
        buffer: &mut [libc::c_char],
    ) -> Result<(), ScriptError> {
        log::debug!("Script::read_line reading a line from file");

        let file_stream = ffi_utils::read_only_c_file_stream_from(file).map_err(|e| {
            let err_msg = format!("failed to read file stream {e}");
            ScriptError::IoError(err_msg)
        })?;

        let result = unsafe {
            libfdisk::fdisk_script_read_line(
                self.inner,
                file_stream as *mut _,
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };

        match result {
            0 => {
                log::debug!("Script::read_line read a line from file");
                Ok(())
            }
            code => {
                let err_msg = if code == 1 {
                    "no line to read from file".to_owned()
                } else {
                    "failed to read a line from file".to_owned()
                };
                log::debug!("Script::read_line {}. libfdisk::fdisk_script_read_line returned error code: {:?}", err_msg, code);

                Err(ScriptError::Read(err_msg))
            }
        }
    }

    /// Returns the number of lines in the `Script`.
    pub fn count_lines(&self) -> Option<usize> {
        log::debug!("Script::count_lines getting the number of script lines");

        let result = unsafe { libfdisk::fdisk_script_get_nlines(self.inner) };

        match result {
            count if count < 0 => {
                let err_msg = "failed to get the number of script lines".to_owned();
                log::debug!("Script::count_lines {}. libfdisk::fdisk_script_get_nlines returned error code: {:?}", err_msg, count);

                None
            }
            count => {
                log::debug!("Script::count_lines number of script lines: {:?}", count);

                Some(count as usize)
            }
        }
    }

    /// Returns the [`Partition`](crate::core::partition::Partition) entries in the partition table defined in this `Script`.
    pub fn partition_table_entries(&self) -> Option<PartitionList> {
        log::debug!("Script::partition_table_entries getting partition table entries");

        let mut ptr = MaybeUninit::<*mut libfdisk::fdisk_table>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_script_get_table(self.inner));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                let err_msg = "failed to get partition table entries".to_owned();
                log::debug!("Script::partition_table_entries {}. libfdisk::fdisk_script_get_table returned a NULL pointer", err_msg);

                None
            }
            ptr => {
                log::debug!("Script::partition_table_entries got partition table entries");
                let entries = PartitionList::borrow_ptr(ptr);

                Some(entries)
            }
        }
    }

    /// Replaces the entries in the partition table defined in this `Script` by those provided.
    pub fn override_partition_table(&mut self, entries: PartitionList) -> Result<(), ScriptError> {
        log::debug!("Script::override_partition_table overriding entries in partition table");

        let result = unsafe { libfdisk::fdisk_script_set_table(self.inner, entries.inner) };

        match result {
            0 => {
                log::debug!("Script::override_partition_table overrode entries in partition table");

                Ok(())
            }
            code => {
                let err_msg = "failed to override entries in partition table".to_owned();
                log::debug!("Script::override_partition_table {}. libfdisk::fdisk_script_set_table returned error code: {:?}", err_msg, code);

                Err(ScriptError::Override(err_msg))
            }
        }
    }

    #[doc(hidden)]
    /// Enables/Disables JSON output.
    fn json_output(ptr: &mut Self, enable: bool) -> Result<(), ScriptError> {
        let op_str = if enable {
            "enable".to_owned()
        } else {
            "disable".to_owned()
        };
        let op = if enable { 1 } else { 0 };

        let result = unsafe { libfdisk::fdisk_script_enable_json(ptr.inner, op) };

        match result {
            0 => {
                log::debug!("Script::json_output {}d JSON output.", op_str);
                Ok(())
            }
            code => {
                let err_msg = format!("failed to {} JSON output", op_str);
                log::debug!("Script::json_output {}. libfdisk::fdisk_script_enable_json returned error code: {:?}", err_msg, code );

                Err(ScriptError::Config(err_msg))
            }
        }
    }

    /// Sets `Script` to output JSON when writing to disk.
    pub fn enable_json_output(&mut self) -> Result<(), ScriptError> {
        log::debug!("Script::enable_json_output enabling JSON output");

        Self::json_output(self, true)
    }

    /// Sets `Script` to output an `sfdisk`-compatible content when writing to disk.
    pub fn disable_json_output(&mut self) -> Result<(), ScriptError> {
        log::debug!("Script::disable_json_output disabling JSON output");

        Self::json_output(self, false)
    }

    #[doc(hidden)]
    /// Writes a script's content to file.
    fn write_file(ptr: &mut Self, file: &mut File) -> Result<(), ScriptError> {
        let file_stream = ffi_utils::write_only_c_file_stream_from(file).map_err(|e| {
            let err_msg = format!("failed to write file stream {e}");
            ScriptError::IoError(err_msg)
        })?;

        let result = unsafe { libfdisk::fdisk_script_write_file(ptr.inner, file_stream as *mut _) };

        match result {
            0 => {
                log::debug!("Script::write_file wrote script to file");

                Ok(())
            }
            code => {
                let err_msg = "failed to write script to file".to_owned();
                log::debug!("Script::write_file {}. libfdisk::fdisk_script_write_file returned error code: {:?}", err_msg, code);

                Err(ScriptError::Write(err_msg))
            }
        }
    }

    /// Writes this `Script`'s content to a file at `file_path`.
    pub fn export_to_file<T>(&mut self, file_path: T) -> Result<(), ScriptError>
    where
        T: AsRef<Path>,
    {
        let file_path = file_path.as_ref();
        log::debug!("Script::export_to_file exporting to file: {:?}", file_path);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_path)
            .map_err(|e| {
                let err_msg = format!("failed to open file {} {e}", file_path.display());
                ScriptError::IoError(err_msg)
            })?;

        Self::write_file(self, &mut file)
    }

    /// Writes this `Script`'s content to an open [`File`]'s.
    pub fn export_to_stream(&mut self, file: &mut File) -> Result<(), ScriptError> {
        log::debug!("Script::export_to_stream exporting to file stream");

        Self::write_file(self, file)
    }

    #[doc(hidden)]
    /// Compose a script that reflects partitioner's configuration
    fn read_context(
        ptr: &mut Self,
        ctx_ptr: *mut libfdisk::fdisk_context,
    ) -> Result<(), ScriptError> {
        let result = unsafe { libfdisk::fdisk_script_read_context(ptr.inner, ctx_ptr) };

        match result {
            0 => {
                log::debug!("Script::read_context composed script");

                Ok(())
            }
            code => {
                let err_msg = "failed to compose script".to_owned();
                log::debug!("Script::read_context {}. libfdisk::fdisk_script_read_context returned error code: {:?}", err_msg, code);

                Err(ScriptError::Compose(err_msg))
            }
        }
    }

    /// Composes a script that reflects the configuration of the [`Fdisk`] that created this `Script`.
    pub fn compose_script(&mut self) -> Result<(), ScriptError> {
        log::debug!("Script::compose_script composing script from associated `Fdisk`");

        Self::read_context(self, std::ptr::null_mut())
    }

    /// Composes a `Script` that reflects the configuration of the referenced [`Fdisk`].
    pub fn compose_script_from(&mut self, context: &Fdisk) -> Result<(), ScriptError> {
        log::debug!("Script::compose_script composing script from external `Fdisk`");

        Self::read_context(self, context.inner)
    }

    #[doc(hidden)]
    /// Sets `Script` header.
    fn set_header(
        ptr: &mut Self,
        name: *const libc::c_char,
        value: *const libc::c_char,
    ) -> Result<(), ScriptError> {
        let result = unsafe { libfdisk::fdisk_script_set_header(ptr.inner, name, value) };

        match result {
            0 => {
                log::debug!("Script::set_header header set");

                Ok(())
            }
            code => {
                let err_msg = "failed to set header".to_owned();
                log::debug! {"Script::set_header {}. libfdisk::fdisk_script_set_header returned error code: {:?}", err_msg, code};

                Err(ScriptError::Compose(err_msg))
            }
        }
    }

    /// Adds a header to the `Script`.
    ///
    /// Headers are set one per line, and defined as global options applied to the entire partition table.
    ///
    /// **Note:** this method can specify:
    /// - arbitrary custom headers,
    /// - default built-in headers: `unit` and `label`,
    /// - or some partition table specific headers, for example `uuid` and `name` for GPT partition tables.
    pub fn add_header<T>(&mut self, name: T, value: T) -> Result<(), ScriptError>
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();
        let name_cstr = ffi_utils::as_ref_str_to_c_string(name).map_err(|e| {
            let err_msg = format!("failed to convert value to `CString` {e}");
            ScriptError::CStringConversion(err_msg)
        })?;

        let value = value.as_ref();
        let value_cstr = ffi_utils::as_ref_str_to_c_string(value).map_err(|e| {
            let err_msg = format!("failed to convert value to `CString` {e}");
            ScriptError::CStringConversion(err_msg)
        })?;

        log::debug!(
            "Script::add_header adding header named: {:?} with value: {:?}",
            name,
            value
        );

        Self::set_header(self, name_cstr.as_ptr(), value_cstr.as_ptr())
    }

    /// Removes a header from the `Script`.
    pub fn remove_header<T>(&mut self, name: T) -> Result<(), ScriptError>
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();
        let name_cstr = ffi_utils::as_ref_str_to_c_string(name).map_err(|e| {
            let err_msg = format!("failed to convert value to `CString` {e}");
            ScriptError::CStringConversion(err_msg)
        })?;

        log::debug!("Script::add_header removing header named: {:?}", name,);

        Self::set_header(self, name_cstr.as_ptr(), std::ptr::null())
    }

    /// Returns the value of a `Script` header.
    pub fn header_value<T>(&self, name: T) -> Option<&str>
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();
        // We assume a name not representable as a C string has no associated
        // header value.
        let name_cstr = ffi_utils::as_ref_str_to_c_string(name).ok()?;

        log::debug!("Script::header_value getting script header: {:?}", name);

        let mut ptr = MaybeUninit::<*const libc::c_char>::zeroed();
        unsafe {
            ptr.write(libfdisk::fdisk_script_get_header(
                self.inner,
                name_cstr.as_ptr(),
            ));
        }

        match unsafe { ptr.assume_init() } {
            ptr if ptr.is_null() => {
                log::debug!("Script::header_value script has no header named: {:?}. libfdisk::fdisk_script_get_header returned a NULL pointer", name);

                None
            }
            value_ptr => {
                let value = ffi_utils::const_char_array_to_str_ref(value_ptr).ok();
                log::debug!(
                    "Script::header_value header named: {:?} has value: {:?}",
                    name,
                    value
                );

                value
            }
        }
    }

    /// Returns `true` if the header `label` was defined.
    pub fn has_header_label(&self) -> bool {
        let state = unsafe { libfdisk::fdisk_script_has_force_label(self.inner) == 1 };
        log::debug!("Script::has_header_label value: {:?}", state);

        state
    }
}

impl<'fdisk> AsRef<Script<'fdisk>> for Script<'fdisk> {
    #[inline]
    fn as_ref(&self) -> &Script<'fdisk> {
        self
    }
}

impl<'fdisk> Drop for Script<'fdisk> {
    fn drop(&mut self) {
        log::debug!("Script::drop deallocating `Script` instance");

        unsafe { libfdisk::fdisk_unref_script(self.inner) }
    }
}
