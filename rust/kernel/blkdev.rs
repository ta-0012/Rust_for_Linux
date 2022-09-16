// SPDX-License-Identifier: GPL-2.0

//! Block devices.
//!
//! Also called "blk devices", "blkdev".
//!
//! C header: [`include/linux/blkdev.h`](../../../../include/linux/blkdev.h)


#[allow(missing_docs)]
// use core::marker::PhantomPinned;
// use core::pin::Pin;
use crate::bindings;
use crate::error::{
    code::*,
//    Error,
    Result};
// use crate::str::CStr;
use crate::bindings::gendisk;
use crate::bindings::dev_t;
use bindings::__register_blkdev;


struct Blkdev(*mut bindings::block_device);

#[allow(dead_code)]
impl Blkdev {
    fn alloc( disk: *mut gendisk, partno: u8) -> Result<Self> {
        let bdev = unsafe { bindings::bdev_alloc(disk, partno) };
        if bdev.is_null() {
            return Err(ENOMEM);
        }
        unsafe {
            (*bdev).bd_disk = disk;
            // line 45996 in bindings_generated.rs
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


/// register(0,"mydev",probe);
pub fn register(
major: core::ffi::c_uint,
name: *const core::ffi::c_char,
probe: ::core::option::Option<unsafe extern "C" fn(devt: dev_t)>
    ) -> Result {
    /*
    let mut blkdev = Blkdev::alloc()?;
    // need: gendisk, partno
    blkdev.add(blkdev, )
    // need: self(Blkdev), dev_t
    */
    // try to call register function directly.
    let _rs = unsafe { __register_blkdev(major,name,probe) };
    Ok(())
}
