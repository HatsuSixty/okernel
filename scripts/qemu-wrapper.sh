#!/bin/bash

if [ -z "$1" ]; then
    echo "ERROR: kernel executable not provided" >&2
    exit 1
fi

SCRIPTDIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)

IMGNAME=disk.img
if [ ! -f $IMGNAME ]; then
    sudo $SCRIPTDIR/create-disk.sh $IMGNAME
fi

qemu-system-i386 -drive file=${IMGNAME},format=raw -kernel $@
