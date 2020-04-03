pub mod value;
pub mod table;
pub mod argument;
pub mod command_invocation;
pub mod list;
pub mod dict;
pub mod binary;
pub mod command;
pub mod job;
pub mod parser;
pub mod r#struct;
pub mod scope;
pub mod stream;
pub mod printer;
pub mod pretty_printer;
pub mod errors;
pub mod ast;
pub mod help;
pub mod execution_context;
pub mod serialize;

pub mod serialization {
    include!(concat!(env!("OUT_DIR"), "/lang.serialization.rs"));
}
