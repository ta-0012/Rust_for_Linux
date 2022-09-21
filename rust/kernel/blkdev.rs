// SPDX-License-Identifier: GPL-2.0

//! Block devices.
//!
//! Also called "blk devices", "blkdev".
//!
//! C header: [`include/linux/blkdev.h`](../../../../include/linux/blkdev.h)

use alloc::boxed::Box;
use bindings::__register_blkdev;
use core::marker::PhantomPinned;
use core::pin::Pin;

use crate::bindings::{
    self,
    __blk_alloc_disk,
    //bdev_add, block_device,
    dev_t,
    gendisk,
    lock_class_key,
    unregister_blkdev,
};
use crate::error::{code::*, Error, Result};
use crate::pr_notice;
use crate::str::CStr;

/// Block device.
///
/// # Invariants
///
///   - [`self.0`] is valid and non-null.
struct Blkdev(*mut gendisk);

impl Blkdev {
    fn alloc(major: i32) -> Result<Self> {
        static mut LKCLASS: lock_class_key = lock_class_key {};
        // SAFETY: `LKCLASS` is valid and non-null
        let disk = unsafe { __blk_alloc_disk(bindings::NUMA_NO_NODE, &mut LKCLASS) };
        if disk.is_null() {
            return Err(ENOMEM);
        }

        unsafe { bindings::set_capacity(disk, 1) };
        //SAFETY:
        unsafe {
            (*disk).major = major;
            (*disk).first_minor = 0;
            (*disk).minors = 1;
            (*disk).disk_name[0] = 'r' as i8;
            (*disk).disk_name[1] = 'u' as i8;
            (*disk).disk_name[2] = 's' as i8;
            (*disk).disk_name[3] = 't' as i8;
            (*disk).disk_name[4] = '_' as i8;
            (*disk).disk_name[5] = 'b' as i8;
            (*disk).disk_name[6] = 'l' as i8;
            (*disk).disk_name[7] = 'k' as i8;
            (*disk).disk_name[8] = 'd' as i8;
            (*disk).disk_name[9] = 'e' as i8;
            (*disk).disk_name[10] = 'v' as i8;
            (*disk).disk_name[11] = 0 as i8;
        }

        if unsafe { ((*disk).part0).is_null() } {
            pr_notice!("part0 is null!");
        }
        // INVARIANTS:
        //   - [`self.0`] is valid and non-null.
        Ok(Self(disk))
    }

    fn add(&mut self) -> Result<i32> {
        let num = unsafe {
            bindings::device_add_disk(core::ptr::null_mut(), self.0, core::ptr::null_mut())
        };
        pr_notice!("num: {}", num);
        Ok(num)
    }
}

impl Drop for Blkdev {
    fn drop(&mut self) {
        unsafe { bindings::del_gendisk(self.0) };
    }
}

/// `RegistrationInner` has actual information of devices.
pub struct RegistrationInner {
    _dev: dev_t,
    _blkdev: Blkdev,
    _pin: PhantomPinned,
}

/// Struct to register devices
pub struct Registration {
    name: &'static CStr,
    major: u32,
    _this_module: &'static crate::ThisModule,
    inner: Option<RegistrationInner>,
}

impl Registration {
    /// Makes an empty Registration
    pub fn new(name: &'static CStr, this_module: &'static crate::ThisModule) -> Result<Self> {
        if name.as_char_ptr().is_null() {
            return Err(ENOMEM);
        }
        // SAFETY: `name` is valid, and non-null
        // and it will live at least as long as [`self.0`]
        let major = unsafe { __register_blkdev(0, name.as_char_ptr(), None) };
        if major < 0 {
            return Err(Error::from_kernel_errno(major));
        }
        // This unwrap() never fail because __register_blkdev
        // returns i32 and we already checked major is non-negative.
        let major = major.try_into().unwrap();
        // Reminds it because we use it to test.
        pr_notice!("Successfully registered. (major number: {})", major);
        Ok(Registration {
            name,
            major,
            _this_module: this_module,
            inner: None,
        })
    }

    /// Makes new `Registration` by calling `new` and pin it.
    pub fn new_pinned(
        name: &'static CStr,
        this_module: &'static crate::ThisModule,
    ) -> Result<Pin<Box<Self>>> {
        let new_result = Self::new(name, this_module)?;
        Ok(Pin::from(Box::try_new(new_result)?))
    }

    /// Registers an instance.
    /// This function not expect to be called more than once.
    pub fn register(
        self: Pin<&mut Self>,
        _this_module: &'static crate::ThisModule,
        //      _name: &'static CStr,
    ) -> Result {
        // checks whether `inner` is None in order to use `get_checked_mut` safe later.
        if self.inner.is_some() {
            return Err(EINVAL);
        }
        let dev: dev_t = 0;

        let mut blkdev = Blkdev::alloc(self.major.try_into().unwrap())?;
        let _result = Blkdev::add(&mut blkdev);
        //        Blkdev::add(&mut blkdev, &mut this_module);
        //SAFETY: `self.inner` is None, so no contents can be moved.
        let this = unsafe { self.get_unchecked_mut() };
        this.inner = Some(RegistrationInner {
            _dev: dev,
            _blkdev: blkdev,
            _pin: PhantomPinned,
        });
        Ok(())
    }
}

// SAFETY: `Registration` does not expose any of its state across threads
// (it is fine for multiple threads to have a shared reference to it).
unsafe impl Sync for Registration {}

impl Drop for Registration {
    fn drop(&mut self) {
        // TODO: unregister and free device.
        // SAFETY: According to the type invariants:
        //   - [`self.major`] will live at least as long as [`self`].
        //   - [`self.name`] will live at least as long as [`self`].
        unsafe { unregister_blkdev(self.major, self.name.as_char_ptr()) };
    }
}
