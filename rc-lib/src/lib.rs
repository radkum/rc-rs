mod error;
mod rc_compiler;

pub(crate) use error::*;

pub type Result<T> = RcResult<T>;
pub use rc_compiler::ResourceCompiler;
