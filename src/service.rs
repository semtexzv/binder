use crate::import::*;
use crate::defs::*;
use crate::pstate::ProcessState;
use crate::parcel::{Parcel, Flattenable};
use crate::tstate::IPCThreadState;
use std::ops::Deref;


pub struct IBinder(u32);

impl Flattenable for IBinder {
    fn flatten(&self) -> Result<sys::flat_binder_object> {
        unimplemented!()
    }

    fn inflate(obj: sys::flat_binder_object) -> Result<Self> {
        unimplemented!()
    }
}

pub struct Service {
    handle: u32,
    state: Arc<IPCThreadState>,
}

impl Service {
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            state: unimplemented!()
        }
    }
}

const BINDER_SERVICE_MANAGER: u32 = 0;

const SVC_MGR_GET_SERVICE: u32 = 1;
const _SVC_MGR_CHECK_SERVICE: u32 = 2;
const SVC_MGR_ADD_SERVICE: u32 = 3;
const SVC_MGR_LIST_SERVICES: u32 = 4;

const INTERFACE_SERVICE_MANAGER: &str = "android.os.IServiceManager";

pub struct ServiceManager {
    state: Arc<IPCThreadState>
}

impl ServiceManager {
    pub fn current() -> Self {
        ServiceManager {
            state: Arc::new(IPCThreadState::current())
        }
    }
    pub fn query(&mut self, name: &str) -> Result<Service> {
        trace!("Querying for interface: {}", name);
        let mut p = Parcel::new();
        p.write_interface_token(INTERFACE_SERVICE_MANAGER);
        p.write_str16(name);
        trace!("Sending transaction");
        let mut r = self.state.transact(
            BINDER_SERVICE_MANAGER,
            SVC_MGR_GET_SERVICE,
            p,
            0,
        )?;
        panic!("R: {:?}", r);
        //let obj: IBinder = r.read_obj()?;
        Ok(unimplemented!())
    }
}