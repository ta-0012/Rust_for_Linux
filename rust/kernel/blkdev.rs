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
/*
struct RegistrationInner<const N: usize> {
    _dev: bindings::dev_t,
    _used: usize,
    _blkdevs: [Option<Blkdev>; N],
    _pin: PhantomPinned,
}

pub struct Registration<const N: usize> {
    _name: &'static CStr,
    _minors_start: u16,
    _this_module: &'static crate::ThisModule,
    _inner: Option<RegistrationInner<N>>,
}

impl<const N: usize> Registration<{ N } > {
    pub fn new () -> Result<u16> {
        Ok(1_u16)
    }
}
*/


