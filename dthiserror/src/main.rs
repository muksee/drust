#![feature(error_generic_member_access)]
#![feature(error_reporter)]
#![feature(error_in_core)]
use std::{
    backtrace::{self, Backtrace},
    fmt::Display,
    io::{self, Error, ErrorKind},
};
use thiserror::{self, Error};

use core::error as coreerror;
use std::error as stderror;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum MyError1 {
    #[error("Hello world")]
    Io {
        #[from]
        source: io::Error,
        backtrace: Backtrace,
    },
}

#[derive(Error, Debug)]
pub struct MyError2 {
    msg: String,
    backtrace: Backtrace, // automatically detected
}

impl Display for MyError2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
    let e = MyError1::Io {
        source: Error::from(ErrorKind::AddrInUse),
        backtrace: backtrace::Backtrace::capture(),
    };

    let r = std::error::Report::from(e)
        .show_backtrace(true)
        .pretty(true);

    println!("{:?}", r);
}
