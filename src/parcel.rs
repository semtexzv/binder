use crate::import::*;
use crate::defs::*;
use std::io::{Cursor, Read};
use std::{io, fmt};
use std::os::raw::c_void;
use std::fmt::{Debug, Formatter};

const STRICT_MODE_PENALTY_GATHER: i32 = 0x40 << 16;

/// Function which can be used to customize how the data buffers owned by parcel are handled
pub type ParcelDropFunc = fn(
    data: *mut u8,
    data_size: usize,
    objects: *mut sys::binder_size_t,
    objects_size: usize,
    cookie: *mut c_void,
);

pub struct Parcel {
    /// Data buffer of the parcel
    /// TODO: We can't properly use vec here ( if we create vec from shared buffer, the reallocation will be invalid)
    data: Cursor<Vec<u8>>,
    /// Offset buffer of the parcel, will contain stuff pertaining to object translation
    offsets: Cursor<Vec<sys::binder_size_t>>,
    /// If this parcel represents a buffer from binder shared memory,
    /// we'll need to manually deallocate it instead of the default drop impl
    dropper: Option<ParcelDropFunc>,
    cookie: *mut c_void,
}

impl Debug for Parcel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Parcel").finish()
    }
}

impl Default for Parcel {
    fn default() -> Self {
        Self {
            data: Cursor::new(vec![]),
            offsets: Cursor::new(vec![]),
            dropper: None,
            cookie: std::ptr::null_mut(),
        }
    }
}

impl Drop for Parcel {
    fn drop(&mut self) {
        if let Some(dropper) = self.dropper {
            let data = mem::replace(&mut self.data, Cursor::new(vec![])).into_inner().into_raw_parts();
            let offsets = mem::replace(&mut self.offsets, Cursor::new(vec![])).into_inner().into_raw_parts();
            dropper(data.0, data.1, offsets.0, offsets.1, self.cookie)
        }
    }
}

impl Parcel {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_parts(
        data: Vec<u8>,
        offsets: Vec<sys::binder_size_t>,
        dropper: ParcelDropFunc,
        cookie: *mut c_void,
    ) -> Self {
        Self {
            data: Cursor::new(data),
            offsets: Cursor::new(offsets),
            dropper: Some(dropper),
            cookie,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data.get_ref()
    }

    pub fn offsets(&self) -> &[sys::binder_size_t] {
        &self.offsets.get_ref()
    }

    #[inline(always)]
    pub fn write_buf(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.data.write(buf)
    }

    #[inline(always)]
    pub fn read_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.data.read(buf)
    }


    pub fn write_str16(&mut self, s: &str) {
        self.data.write_i32::<LittleEndian>(s.len() as _).unwrap();
        for c in s.encode_utf16() {
            self.data.write_u16::<LittleEndian>(c).unwrap();
        }

        while (self.data.get_ref().len() % 4) != 0 {
            self.data.write_u8(0).unwrap();
        }
    }

    pub fn read_str16(&mut self) -> String {
        let len = self.data.read_i32::<LittleEndian>().unwrap() as usize;
        let mut data = vec![0u8; len * 2 + 2];
        let d = self.data.read(data.as_mut()).unwrap();
        let r = {
            unsafe { from_raw_parts(data.as_ptr() as *const u16, len * 2) }
        };
        String::from_utf16(r).unwrap()
    }

    pub fn write_interface_token(&mut self, iface: &str) {
        self.data.write_i32::<LittleEndian>(STRICT_MODE_PENALTY_GATHER).unwrap();
        self.write_str16(iface)
    }

    pub fn write_obj<F: Flattenable>(&mut self, obj: F) -> Result<()> {
        unimplemented!()
    }

    pub fn read_obj<F: Flattenable>(&mut self) -> Result<F> {
        unimplemented!()
    }

    /*
    pub fn write_strong_binder(&mut self) {}
    pub fn write_weak_binder(&mut self) {}
    pub fn write_native_handle(&mut self) {}
    pub fn write_file_descriptior(&mut self, fd: u32, take_ownership: bool) {
        let mut obj = sys::flat_binder_object::default();
        obj.hdr.type_ = sys::BINDER_TYPE_FD;
        obj.flags = 0x7F | sys::FLAT_BINDER_FLAG_ACCEPTS_FDS;
        obj.__bindgen_anon_1.handle = fd;
        obj.cookie = (if take_ownership { 1 } else { 0 });
        self.write_object(obj, true);
    }

    fn write_object(&mut self, obj: sys::flat_binder_object, null_md: bool) {}
     */
}

/// A thing that can be sent across binder boundaries
pub trait Flattenable: Sized {
    fn flatten(&self) -> Result<sys::flat_binder_object>;
    fn inflate(obj: sys::flat_binder_object) -> Result<Self>;
}

impl Flattenable for RawFd {
    fn flatten(&self) -> Result<sys::flat_binder_object> {
        let mut obj = sys::flat_binder_object::default();
        obj.hdr.type_ = sys::BINDER_TYPE_FD;
        obj.flags = 0x7F | sys::FLAT_BINDER_FLAG_ACCEPTS_FDS;
        obj.__bindgen_anon_1.handle = *self as u32;
        obj.cookie = 0;
        return Ok(obj);
    }

    fn inflate(obj: sys::flat_binder_object) -> Result<Self> {
        if obj.hdr.type_ != sys::BINDER_TYPE_FD {
            return Err(nix::Error::UnsupportedOperation);
        }

        unsafe {
            Ok(obj.__bindgen_anon_1.handle as Self)
        }
    }
}