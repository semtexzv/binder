#!/bin/bash

export ANDROID_SDK_HOME=~/Android/Sdk
export ANDROID_NDK_HOME=${ANDROID_SDK_HOME}/ndk/21.3.6528147/

cargo ndk --platform 21 --target arm-linux-androideabi build --example getsvc
adb push target/arm-linux-androideabi/debug/examples/getsvc /data/local/tmp
adb shell chmod 755 /data/local/tmp/getsvc
adb shell 'RUST_BACKTRACE=full /data/local/tmp/getsvc'