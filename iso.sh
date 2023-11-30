#!/bin/sh

if [ ! -f target/target/debug/okernel ]; then
    echo "ERROR: kernel image not found. Please build it first." >&2
    exit 1
fi
cp target/target/debug/okernel root/boot/okernel
grub-mkrescue -o o.iso root &> /dev/null
