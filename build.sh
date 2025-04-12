#! /bin/bash

cd asm
clang -c *.s -O3

mkdir -p build
ar rcs libneon.a *.o
mv libneon.a build/libneon.a

rm *.o

cd ..