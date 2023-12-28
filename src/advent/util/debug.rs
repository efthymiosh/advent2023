use std::{io, io::prelude::*};

#[allow(dead_code)]
pub(crate) fn pause() {
    io::stdin().read_exact(&mut [0]).unwrap();
}
