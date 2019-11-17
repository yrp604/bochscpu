#![feature(trait_alias)]

#[macro_use]
extern crate ctor;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub static NUM_CPUS: usize = 255;

pub type Address = u64;
pub type PhyAddress = u64;

mod logfunctions;
mod params;
mod sim;
mod syncunsafecell;

pub mod cpu;
pub mod hook;
pub mod mem;
