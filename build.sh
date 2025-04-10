#! /bin/bash

clang -O3 asm/*.s -shared -o asm/build/libneon.so

cp asm/build/libneon.so target/release/libneon.so
cp asm/build/libneon.so target/debug/libneon.so