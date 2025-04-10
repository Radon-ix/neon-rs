#! /bin/bash

cd asm
clang -c *.s -O3

mkdir -p build
cd build
ar rcs libneon.a ../*.o

cd ../..

rm asm/*.o

cp asm/build/libneon.a target/release/libneon.a
cp asm/build/libneon.a target/debug/libneon.a