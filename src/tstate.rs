use crate::import::*;
use crate::defs::*;
use crate::pstate::ProcessState;
use crate::parcel::Parcel;
use std::os::raw::c_void;

const READ_SIZE: usize = 32 * 4;

#[derive(Debug)]
pub enum Reply {
    Status(i32),
    Parcel(Parcel),
}

#[repr(packed)]
pub struct CallBuf {
    cmd: sys::binder_driver_command_protocol,
    tx: sys::binder_transaction_data,
}

pub struct IPCThreadState {
    process: Arc<ProcessState>,
}

impl IPCThreadState {
    pub fn current() -> Self {
        IPCThreadState {
            process: ProcessState::current().unwrap(),
        }
    }

    pub fn transact(&self, handle: u32, code: u32, data: Parcel, mut flags: sys::transaction_flags) -> Result<Reply> {
        flags |= sys::TF_ACCEPT_FDS;
        let call = self.write_transaction(sys::BC_TRANSACTION, handle, code, flags, data)?;
        trace!("Write ok");
        let reply = unsafe { self.wait_for_reply(raw_byte_repr(&call))? };
        Ok(reply)
    }


    unsafe fn wait_for_reply(&self, out: &[u8]) -> Result<Reply> {
        let mut bwr = sys::binder_write_read::default();

        bwr.write_size = out.len() as sys::binder_size_t;
        bwr.write_buffer = out.as_ptr() as sys::binder_uintptr_t;
        bwr.write_consumed = 0;

        let reply = [0; READ_SIZE];

        bwr.read_size = reply.len() as sys::binder_size_t;
        bwr.read_buffer = reply.as_ptr() as sys::binder_uintptr_t;
        bwr.read_consumed = 0;

        trace!("BWR");
        let res = binder_write_read(self.process.fd, &mut bwr).unwrap();
        trace!("BWR OK {:?}", res);

        let mut reply = &reply[..(bwr.read_consumed as usize)];
        loop {
            let c = reply.read_u32::<LittleEndian>().unwrap();

            match c {
                sys::BR_NOOP => {}
                sys::BR_ERROR => panic!("Error"),
                sys::BR_TRANSACTION_COMPLETE => {}
                sys::BR_SPAWN_LOOPER => {}
                sys::BR_REPLY => {
                    let data: sys::binder_transaction_data = std::ptr::read(reply.as_ptr() as *const _);
                    if data.flags & sys::TF_STATUS_CODE != 0 {
                        // Return status code reply
                        debug!("Status reply: {:?}", &data.data.buf[..]);
                        let res = *(data.data.ptr.buffer as *const i32);
                        panic!("Returning :{:?}", res);
                        return Ok(Reply::Status(res));
                    } else {

                        // TODO: Don't use vector, custom buffer type with manual realloc if possible
                        let dbuffer = Vec::from_raw_parts(
                            data.data.ptr.buffer as *mut _,
                            data.data_size as usize,
                            data.offsets_size as usize,
                        );

                        let obuffer = Vec::from_raw_parts(
                            data.data.ptr.offsets as *mut _,
                            data.offsets_size as usize,
                            data.offsets_size as usize,
                        );

                        return Ok(Reply::Parcel(Parcel::from_parts(
                            dbuffer,
                            obuffer,
                            free_buffer,
                            std::ptr::null_mut(),
                        )));
                    }
                    debug!("Replt");
                    unimplemented!()
                }
                sys::BR_FAILED_REPLY => {
                    panic!("Failed reply")
                }
                other => { self.process_cmd(other)?; }
            }
        }
    }

    fn process_cmd(&self, cmd: u32) -> Result<()> {
        Ok(())
    }

    fn write_transaction(&self, cmd: u32, handle: u32, code: u32, flags: u32, data: Parcel) -> Result<CallBuf> {
        let mut bt = sys::binder_transaction_data::default();

        bt.target.ptr = 0;
        bt.target.handle = handle;
        bt.code = code;
        bt.flags = flags;
        bt.cookie = 0;

        bt.data.ptr.buffer = data.data().as_ptr() as sys::binder_uintptr_t;
        bt.data_size = data.data().len() as sys::binder_size_t;

        bt.data.ptr.offsets = data.offsets().as_ptr() as sys::binder_uintptr_t;
        bt.offsets_size = (data.offsets().len() * mem::size_of::<sys::binder_size_t>()) as sys::binder_size_t;

        Ok(CallBuf {
            cmd,
            tx: bt,
        })
    }
}

pub fn free_buffer(
    data: *mut u8,
    data_size: usize,
    objects: *mut sys::binder_size_t,
    objects_size: usize,
    cookie: *mut c_void,
) {}
