#!/usr/bin/env sh

unset CDPATH

rm -rfv runtime-env
mkdir -p runtime-env/plugins
cp -v target/debug/libhello_world.so runtime-env/plugins/hello_world.plugin
cp -v target/debug/libbye_world.so runtime-env/plugins/bye_world.plugin
cp -v target/debug/client runtime-env/client
cd runtime-env
export LD_LIBRARY_PATH="$(rustc --print sysroot)/lib:$LD_LIBRARY_PATH"
export RUST_LOG="client=debug,hello_world=debug,bye_world=debug"
valgrind --leak-check=full ./client