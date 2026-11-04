/// Errors returned by action command line argument parsing and formatting.
#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("arguments must be a list, tuple, or depset")]
    ArgumentsMustBeListTupleOrDepset,
    #[error("map_each must return a list[str], str, or None")]
    MapEachInvalidReturn,
    #[error("Format string must contain exactly one '%s', got: {0}")]
    InvalidFormatString(String),
    #[error("trying to mutate a frozen Args value")]
    CannotMutateFrozenArgs,
    #[error("Expected first argument of add to be a string flag when value is specified")]
    ExpectedAddStringFlag,
    #[error("Expected first argument of add_all to be a string flag when values is specified")]
    ExpectedAddAllStringFlag,
    #[error("Expected first argument of add_joined to be a string flag when values is specified")]
    ExpectedAddJoinedStringFlag,
    // The starlark-rs API doesn't provide a way to check if something is a closure.
    // So we provide a "user validates" approach instead.
    #[error(
        "map_each was specified without allow_closure.\n\
         For optimal performance, map_each should be a top-level function.\n\
         If you do this, set allow_closure = False.\n\
         Otherwise, set allow_closure = True"
    )]
    MapEachRequiresAllowClosure,
    #[error("None is not allowed unless mapped by map_each")]
    NoneNotAllowed,
}

impl From<Error> for starlark::Error {
    fn from(err: Error) -> Self {
        Self::new_other(err)
    }
}
