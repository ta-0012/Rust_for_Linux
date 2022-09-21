// SPDX-License-Identifier: GPL-2.0

//! Rust block device sample.

use kernel::blkdev;
use kernel::prelude::*;

module! {
    type: RustBlkdev,
    name: "rust_blkdev",
    author: "Yukina Tatsuta",
    description: "Rust block device sample",
    license: "GPL",
}

struct RustBlkdev {
    _dev: Pin<Box<blkdev::Registration>>,
}

impl kernel::Module for RustBlkdev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust block device sample (init)\n");
        let mut blkdev_reg = blkdev::Registration::new_pinned(name, module)?;
        blkdev_reg.as_mut().register(module)?;
        Ok(RustBlkdev { _dev: blkdev_reg })
    }
}

impl Drop for RustBlkdev {
    fn drop(&mut self) {
        pr_info!("Rust block device sample (exit)\n");
        //    TODO: do test to confirm drop works as I expected.
    }
}
