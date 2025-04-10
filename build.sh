#! /bin/bash

cd asm
clang -c *.s -O3
cd ..

ar rcs asm/build/libneon.a asm/*.o

rm asm/*.o

cp asm/build/libneon.a target/aarch64-unknown-linux-gnu/release/libneon.a
cp asm/build/libneon.a target/aarch64-unknown-linux-gnu/debug/libneon.a