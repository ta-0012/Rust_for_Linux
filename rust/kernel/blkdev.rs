// SPDX-License-Identifier: GPL-2.0

//! Block devices.
//!
//! Also called "blk devices", "blkdev".
//!
//! C header: [`include/linux/blkdev.h`](../../../../include/linux/blkdev.h)


use crate::bindings;
use crate::error::{
    code::*,
//    Error,
    Result};
use crate::str::CStr;
use crate::bindings::gendisk;

struct Blkdev(*mut bindings::block_device);

impl Blkdev {
    fn alloc( disk: *mut gendisk, partno: u8) -> Result<Self> {
        let bdev = unsafe { bindings::bdev_alloc(disk, partno) };
        if bdev.is_null() {
            return Err(ENOMEM);
        }
        unsafe {
            (*bdev).bd_disk = disk;
            // line 46000
        }

        Ok(Self(bdev))
    }

    fn add(
        &mut self, dev: bindings::dev_t
        //bdev_add(bdev: *mut block_device, dev: dev_t);
        )
        -> Result {
        unsafe { bindings::bdev_add(self.0, dev) }; // dosen't have return value?
        Ok(())
    }
}

pub struct Registration {
    name: &'static CStr,
}


