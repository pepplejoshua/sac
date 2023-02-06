#!/bin/bash

if [ $# -eq 0 ]
then
  echo "need sac file to compile..."
elif [ $# -eq 1 ]
then
  cargo build --release --quiet
  ./target/release/sac
  arm-linux-gnueabihf-gcc -static -mcpu=cortex-a7 "$1.s" -o "$1"
  time -p qemu-arm "$1"
  echo
  rm $1
else 
  echo "need 1 sac file to compile... got $#";
fi