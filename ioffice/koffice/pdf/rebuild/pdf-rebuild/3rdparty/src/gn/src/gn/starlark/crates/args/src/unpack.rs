use either::Either;
use starlark::{
    eval::Evaluator,
    typing::Ty,
    values::{list::ListRef, type_repr::StarlarkTypeRepr, UnpackValue, Value, ValueTyped},
};

use crate::{formatter::Formatter, FrozenArgs};

impl<'v> UnpackValue<'v> for Formatter {
    type Error = starlark::Error;

    fn unpack_value_impl(value: Value<'v>) -> Result<Option<Self>, Self::Error> {
        let s = <&'v str>::unpack_value_err(value)?;
        Self::new(s).map(Some)
    }
}

#[derive(Debug, Default)]
pub struct FrozenArgsSequence<'v>(pub Vec<Either<&'v str, ValueTyped<'v, FrozenArgs>>>);

impl<'v> StarlarkTypeRepr for FrozenArgsSequence<'v> {
    type Canonical = starlark::values::Value<'v>;

    fn starlark_type_repr() -> Ty {
        Ty::list(Ty::any())
    }
}

impl<'v> UnpackValue<'v> for FrozenArgsSequence<'v> {
    type Error = starlark::Error;

    fn unpack_value_impl(value: Value<'v>) -> Result<Option<Self>, Self::Error> {
        // We unpack the list elements one by one, because when you pass [1] to
        // UnpackList<&str>, it says "expected list, but got list".
        // This way, it says "expected string | Args, but got int"
        Ok(Some(FrozenArgsSequence(
            <&ListRef>::unpack_value_err(value)?
                .iter()
                .map(<Either<&'v str, ValueTyped<'v, FrozenArgs>>>::unpack_value_err)
                .collect::<Result<Vec<_>, _>>()?,
        )))
    }
}

impl<'v> FrozenArgsSequence<'v> {
    pub fn expand(&self, eval: &mut Evaluator<'v, '_, '_>) -> starlark::Result<Vec<String>> {
        let mut command = Vec::new();
        for item in &self.0 {
            match item {
                Either::Left(s) => command.push((*s).to_owned()),
                Either::Right(args) => {
                    crate::expand::expand_into(&mut command, args.as_ref(), eval)?;
                },
            }
        }
        Ok(command)
    }
}
