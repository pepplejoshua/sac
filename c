#!/bin/bash

if [ $# -eq 0 ]
then
  echo "need arm asm file to compile..."
elif [ $# -eq 1 ]
then
  cargo build --release --quiet
  ./target/release/sac
  arm-linux-gnueabi-gcc -static $1 -o sac_o
  qemu-arm ./sac_o
  echo;
  rm ./sac_o
else 
  echo "need 1 arm asm file to compile... got $#";
fi