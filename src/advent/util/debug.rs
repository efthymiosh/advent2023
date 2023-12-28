use std::{io, io::prelude::*};
use std::time::Duration;
use std::thread::sleep;

#[allow(dead_code)]
pub(crate) fn pause() {
    io::stdin().read_exact(&mut [0]).unwrap();
}

#[allow(dead_code)]
pub(crate) fn wait_for(milliseconds: u32) {
    sleep(Duration::new(0, milliseconds * 1000 * 1000));
}
