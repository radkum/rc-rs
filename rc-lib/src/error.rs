use snafu::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum RcError {
    #[snafu(display("Could not open the rc file at {}: {}", path.display(), source))]
    OpenRc { path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not read the rc file. {}", source))]
    ReadRc { source: std::io::Error },

    #[snafu(display("Could not write the to buffer. {}", source))]
    WriteRc { source: std::io::Error },

    #[snafu(display("There is no parsed data"))]
    NotParsed { },
}

pub type RcResult<T> = Result<T, RcError>;