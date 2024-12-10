mod rc_compiler;
mod error;

pub(crate) use error::*;

pub type Result<T> = RcResult<T>;
pub use rc_compiler::ResourceCompiler;