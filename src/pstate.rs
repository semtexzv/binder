use crate::import::*;
use crate::defs::*;

const VM_SIZE: usize = ((1 * 1024 * 1024) - (4096 * 2));
const MAX_THREADS: u32 = 15;

pub static PROC_STATE: Lazy<Mutex<Option<Weak<ProcessState>>>> = Lazy::new(|| Mutex::new(None));


/// Global per-process state of a binder connection
pub struct ProcessState {
    pub fd: RawFd,
    mmap: Box<[u8]>,
}

impl ProcessState {
    /// Initialize new ProcessState
    fn new() -> Result<ProcessState> {
        let fd = open("/dev/binder", OFlag::O_RDWR | OFlag::O_CLOEXEC, Mode::empty())?;
        if fd == 0 {
            panic!("Could not open binder device");
        }
        trace!("Opened binder device");
        unsafe {
            let mut version = sys::binder_version::default();
            binder_version(fd, &mut version)?;
            if version.protocol_version != sys::BINDER_CURRENT_PROTOCOL_VERSION as _ {
                panic!("Invalid binder version");
            }
            binder_set_max_threads(fd, &MAX_THREADS)?;
        }
        trace!("Inited binder threads");
        let mut data = vec![0; VM_SIZE].into_boxed_slice();

        let mut binder = ProcessState {
            fd,
            mmap: data,
        };

        let mapped = (&mut binder.mmap).as_mut_ptr() as *mut ::nix::libc::c_void;

        unsafe {
            mmap(mapped, VM_SIZE, ProtFlags::PROT_READ,
                 MapFlags::MAP_PRIVATE | MapFlags::MAP_NORESERVE, fd, 0)?;
        }
        trace!("Mapped binder memory");
        Ok(binder)
    }

    /// Create or return current ProcessState.
    pub fn current() -> Result<Arc<Self>> {
        let mut state = PROC_STATE.lock().unwrap();

        if let Some(ref mut state) = state.deref_mut() {
            if let Some(ptr) = state.upgrade() {
                return Ok(ptr);
            }
        }
        let ret = ProcessState::new()?;
        let ret = Arc::new(ret);
        *state = Some(Arc::downgrade(&ret));
        return Ok(ret);
    }
}

impl Drop for ProcessState {
    fn drop(&mut self) {
        unsafe {
            munmap(self.mmap.as_mut_ptr() as _, VM_SIZE).unwrap();
            close(self.fd).unwrap();
        }
    }
}
