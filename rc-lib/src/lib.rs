mod resource_compiler;
mod error;

pub(crate) use error::*;

pub type Result<T> = RcResult<T>;
pub use resource_compiler::ResourceCompiler;