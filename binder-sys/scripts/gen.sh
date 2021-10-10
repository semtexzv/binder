#!/bin/env bash

#bindgen include/android-binder.h -o src/ffi.rs

SYSROOT=/home/mhornicky/Android/Sdk/ndk/21.3.6528147/toolchains/llvm/prebuilt/linux-x86_64/sysroot

bindgen ${SYSROOT}/usr/include/linux/android/binder.h \
  --disable-name-namespacing --no-layout-tests --no-prepend-enum-name --with-derive-default \
  -o src/ffi.rs \
  -- \
  --sysroot=${SYSROOT} \
  --include ${SYSROOT}/usr/include/sys/types.h \
  -I${SYSROOT}/usr/include \
  -I${SYSROOT}/usr/include/arm-linux-androideabi
