use core::result::Result;
use did_core::{constants::super_lock, debug};

use crate::error::Error;


pub fn main() -> Result<(), Error> {
    let super_lock = super_lock();
    debug!("super_lock: {:?}", super_lock);

    Ok(())
}

