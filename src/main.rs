#![allow(clippy::ptr_arg)]

use std::{fs::File, io::Write};

use component::{Components, readable::ReadableComponents};
use file::FilePresets;

use crate::systems::readable::ReadableSystems;
mod merge;
mod component;
mod constants;
mod extra_bits;
mod file;
mod init;
mod instr;
mod location;
mod object;
mod resources;
mod save;
#[allow(dead_code)]
mod shape;
mod system;
mod systems;
mod ui;
pub fn main() {
}