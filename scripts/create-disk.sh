#!/bin/bash

if [ "$UID" != 0 ]; then
    echo "ERROR: this script must be executed as root" >&2
    exit 1
fi

if [ -z "$1" ]; then
    echo "ERROR: file name not provided" >&2
    exit 1
fi

IMGNAME=$1

qemu-img create $IMGNAME 512M
parted $IMGNAME mklabel msdos mkpart primary fat32 2048s 100% set 1 boot on

LOOPDEV=$(losetup --find --show -P $IMGNAME)
mkfs.fat -F32 ${LOOPDEV}p1
losetup --detach $LOOPDEV

chmod 666 $IMGNAME
