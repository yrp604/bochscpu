#![feature(const_vec_new)]
#![feature(trait_alias)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

mod logfunctions;
mod params;
mod sim;

pub mod hook;
