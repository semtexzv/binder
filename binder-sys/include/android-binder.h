#include <unistd.h> // needed for pid_t and uid_t
#include <stdint.h>

// TODO: Figure out how to select and use the binder.h header from the correct bionic
// version. The file binder.h is a copy from NDK.

//#include <linux/android/binder.h>
#include "./binder.h"

#define B_PACK_CHARS(c1, c2, c3, c4) ((((c1)<<24)) | (((c2)<<16)) | (((c3)<<8)) | (c4))

enum Protocol {
    version = BINDER_CURRENT_PROTOCOL_VERSION,
};

enum BinderType {
    BINDER = BINDER_TYPE_BINDER,
    WEAK_BINDER = BINDER_TYPE_WEAK_BINDER,
    HANDLE = BINDER_TYPE_HANDLE,
    WEAK_HANDLE = BINDER_TYPE_WEAK_HANDLE,
    FD = BINDER_TYPE_FD,
};

enum IBinder {
    FIRST_CALL_TRANSACTION  = 0x00000001,
    LAST_CALL_TRANSACTION   = 0x00ffffff,
    PING_TRANSACTION        = B_PACK_CHARS('_','P','N','G'),
    DUMP_TRANSACTION        = B_PACK_CHARS('_','D','M','P'),
    INTERFACE_TRANSACTION   = B_PACK_CHARS('_', 'N', 'T', 'F'),
    SYSPROPS_TRANSACTION    = B_PACK_CHARS('_', 'S', 'P', 'R'),
    FLAG_ONEWAY             = 0x00000001
};
