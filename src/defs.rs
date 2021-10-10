
const MAGIC: u8 = b'b';

ioctl_readwrite!(binder_write_read, MAGIC, 1, sys::binder_write_read);
ioctl_write_ptr!(binder_set_idle_timeout, MAGIC, 3, i64);
ioctl_write_ptr!(binder_set_max_threads, MAGIC, 5, u32);
ioctl_write_ptr!(binder_set_idle_priority, MAGIC,6, i32);
ioctl_write_int!(binder_set_context_mgr, MAGIC, 7);
ioctl_write_int!(binder_thread_exit, MAGIC, 8);
ioctl_readwrite!(binder_version, MAGIC, 9, sys::binder_version);
