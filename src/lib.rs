#![feature(
    i128_type,
    rustc_private,
)]

// From rustc.
#[macro_use]
extern crate log;
extern crate log_settings;
#[macro_use]
extern crate rustc;
extern crate rustc_const_math;
extern crate rustc_data_structures;
extern crate syntax;

// From crates.io.
extern crate byteorder;

mod cast;
mod const_eval;
mod error;
mod eval_context;
mod lvalue;
mod memory;
mod operator;
mod step;
mod terminator;
mod traits;
mod value;

pub use error::{
    EvalError,
    EvalResult,
};

pub use eval_context::{
    EvalContext,
    Frame,
    ResourceLimits,
    StackPopCleanup,
    eval_main,
};

pub use lvalue::{
    Lvalue,
    LvalueExtra,
};

pub use memory::{
    AllocId,
    Memory,
    MemoryPointer,
};

pub use value::{
    PrimVal,
    PrimValKind,
    Value,
};

pub use const_eval::{
    eval_body_as_integer,
};
