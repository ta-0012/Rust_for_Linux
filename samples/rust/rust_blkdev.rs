// SPDX-License-Identifier: GPL-2.0

//! Rust block device sample.

use kernel::prelude::*;
// use kernel::blkdev;

module! {
    type: RustBlkdev,
    name: "rust_blkdev",
    author: "Rust for Linux Contributors",
    description: "Rust block device sample",
    license: "GPL",
}

struct RustBlkdev {
//    _dev: blkdev::Blkdev,
}


impl kernel::Module for RustBlkdev {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust block device sample (init)\n");
        pr_info!("name:{}\n", _name);
        /*
        let num = blkdev::Registration<1>::new();
        pr_info!("test: {}",num);
*/
        Ok(RustBlkdev {})
    }
}

impl Drop for RustBlkdev {
    fn drop(&mut self) {
        pr_info!("Rust block device sample (exit)\n");
    }
}
