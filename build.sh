#! /bin/bash

cd asm
clang -c *.s -O3

mkdir -p build
ar rcs libneon.a *.o
mv libneon.a build/libneon.a

rm *.o

cd ..

cp asm/build/libneon.a target/release/libneon.a
cp asm/build/libneon.a target/debug/libneon.a
cp asm/build/libneon.a target/release/deps/libneon.a
cp asm/build/libneon.a target/debug/deps/libneon.a