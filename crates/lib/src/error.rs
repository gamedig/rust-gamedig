use error_stack::Report;
use std::result::Result as StdResult;

pub(crate) type ResultConstrustor<T, E> = StdResult<T, Report<E>>;

