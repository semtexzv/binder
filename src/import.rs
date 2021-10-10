pub use byteorder::{LittleEndian, ReadBytesExt, LE, WriteBytesExt, NativeEndian};

pub use nix::{
    sys::stat::Mode,
    fcntl::{open, OFlag},
    sys::mman::{mmap, ProtFlags, MapFlags, munmap},
    unistd::close,
};


pub use std::{
    ops::DerefMut,
    sync::{Weak, Mutex, Arc},
    slice::from_raw_parts,
    mem::size_of,
    i8::MAX,
    os::unix::prelude::RawFd,
    mem,
    io::Write,
};
pub use once_cell::sync::Lazy;


pub type Result<T, E = nix::Error> = std::result::Result<T, E>;

/// Safe to use with any wholly initialized memory `ptr`
pub unsafe fn raw_byte_repr<'a, T>(ptr: &'a T) -> &'a [u8]
{
    from_raw_parts(
        ptr as *const _ as *const u8,
        std::mem::size_of::<T>(),
    )
}


#[derive(thiserror::Error, Debug)]
pub enum BinderError {
    #[error("Security")]
    Security,
    #[error("BadParcelable")]
    BadParcelable,
    #[error("IllegalArgument")]
    IllegalArgument,
    #[error("NullPointer")]
    NUllPointer,
    #[error("IllegalState")]
    IllegalState,
    #[error("NetworkMainThread")]
    NetworkMainThread,
    #[error("Unsupported")]
    Unsupported,
    #[error("ServiceSpecific")]
    ServiceSpecivic,
    #[error("Parcelable")]
    Parcelable,
    #[error("TransactionFailed")]
    TransactionFailed,
    #[error("Other")]
    Other,
}

impl BinderError {
    pub fn from_status(s: i32) -> Self {
        match s {
            -1 => Self::Security,
            -2 => Self::BadParcelable,
            -3 => Self::IllegalArgument,
            -4 => Self::NUllPointer,
            -5 => Self::IllegalState,
            -6 => Self::NetworkMainThread,
            -7 => Self::Unsupported,
            -8 => Self::ServiceSpecivic,
            -9 => Self::Parcelable,
            -129 => Self::TransactionFailed,
            other => panic!("Invalid status", )
        }
    }
}