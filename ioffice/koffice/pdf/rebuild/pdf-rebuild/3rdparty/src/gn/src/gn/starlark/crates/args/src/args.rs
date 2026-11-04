// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{cell::RefCell, fmt, fmt::Display};

use allocative::Allocative;
use starlark::{
    environment::{Methods, MethodsBuilder, MethodsStatic},
    eval::Evaluator,
    starlark_simple_value,
    values::{
        Coerce, Freeze, FreezeResult, Freezer, FrozenValue, ProvidesStaticType, StarlarkValue,
        Trace, Tracer, Value, ValueLike as _,
    },
};
use starlark_derive::{starlark_module, starlark_value, NoSerialize};

use crate::{formatter::Formatter, Error};

/// Internal representation of individual arguments stored in `Args`.
#[derive(Debug, Clone, Trace, Coerce, ProvidesStaticType, NoSerialize, Allocative)]
#[repr(C)]
pub enum ArgValue<V> {
    Scalar {
        arg_name: Option<String>,
        value: V,
        format: Option<Formatter>,
    },
    All {
        flag: Option<String>,
        values: V,
        map_each: Option<V>,
        format_each: Option<Formatter>,
        before_each: Option<String>,
        terminate_with: Option<String>,
        omit_if_empty: bool,
        uniquify: bool,
    },
    Joined {
        flag: Option<String>,
        values: V,
        join_with: String,
        map_each: Option<V>,
        format_each: Option<Formatter>,
        format_joined: Option<Formatter>,
        omit_if_empty: bool,
        uniquify: bool,
    },
}

impl ArgValue<FrozenValue> {
    pub fn to_value<'v>(&self) -> ArgValue<Value<'v>> {
        match self {
            Self::Scalar {
                arg_name,
                value,
                format,
            } => ArgValue::Scalar {
                arg_name: arg_name.clone(),
                value: value.to_value(),
                format: format.clone(),
            },
            Self::All {
                flag,
                values,
                map_each,
                format_each,
                before_each,
                terminate_with,
                omit_if_empty,
                uniquify,
            } => ArgValue::All {
                flag: flag.clone(),
                values: values.to_value(),
                map_each: map_each.map(|m| m.to_value()),
                format_each: format_each.clone(),
                before_each: before_each.clone(),
                terminate_with: terminate_with.clone(),
                omit_if_empty: *omit_if_empty,
                uniquify: *uniquify,
            },
            Self::Joined {
                flag,
                values,
                join_with,
                map_each,
                format_each,
                format_joined,
                omit_if_empty,
                uniquify,
            } => ArgValue::Joined {
                flag: flag.clone(),
                values: values.to_value(),
                join_with: join_with.clone(),
                map_each: map_each.map(|m| m.to_value()),
                format_each: format_each.clone(),
                format_joined: format_joined.clone(),
                omit_if_empty: *omit_if_empty,
                uniquify: *uniquify,
            },
        }
    }
}

impl Freeze for ArgValue<Value<'_>> {
    type Frozen = ArgValue<FrozenValue>;

    fn freeze(self, freezer: &Freezer) -> FreezeResult<Self::Frozen> {
        match self {
            ArgValue::Scalar {
                arg_name,
                value,
                format,
            } => Ok(ArgValue::Scalar {
                arg_name,
                value: value.freeze(freezer)?,
                format: format.freeze(freezer)?,
            }),
            ArgValue::All {
                flag,
                values,
                map_each,
                format_each,
                before_each,
                terminate_with,
                omit_if_empty,
                uniquify,
            } => Ok(ArgValue::All {
                flag,
                values: values.freeze(freezer)?,
                map_each: map_each.map(|m| m.freeze(freezer)).transpose()?,
                format_each: format_each.freeze(freezer)?,
                before_each,
                terminate_with,
                omit_if_empty,
                uniquify,
            }),
            ArgValue::Joined {
                flag,
                values,
                join_with,
                map_each,
                format_each,
                format_joined,
                omit_if_empty,
                uniquify,
            } => Ok(ArgValue::Joined {
                flag,
                values: values.freeze(freezer)?,
                join_with,
                map_each: map_each.map(|m| m.freeze(freezer)).transpose()?,
                format_each: format_each.freeze(freezer)?,
                format_joined: format_joined.freeze(freezer)?,
                omit_if_empty,
                uniquify,
            }),
        }
    }
}

/// The mutable Starlark `Args` object used to construct command lines for
/// actions.
#[derive(Debug, Default, ProvidesStaticType, NoSerialize, Allocative)]
pub struct Args<'v> {
    /// List of arguments added to the builder.
    pub(crate) arguments: RefCell<Vec<ArgValue<Value<'v>>>>,
}

/// The frozen Starlark `Args` object, which is read-only and thread-safe.
#[derive(Debug, Default, ProvidesStaticType, NoSerialize, Allocative)]
pub struct FrozenArgs {
    /// List of frozen arguments.
    pub(crate) arguments: Vec<ArgValue<FrozenValue>>,
}

unsafe impl<'v> Trace<'v> for Args<'v> {
    fn trace(&mut self, tracer: &Tracer<'v>) {
        for arg in self.arguments.borrow_mut().iter_mut() {
            arg.trace(tracer);
        }
    }
}

starlark_simple_value!(FrozenArgs);

impl<'v> starlark::values::AllocValue<'v> for Args<'v> {
    #[inline]
    fn alloc_value(self, heap: starlark::values::Heap<'v>) -> starlark::values::Value<'v> {
        heap.alloc_complex(self)
    }
}

impl<'v> Display for Args<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "args")
    }
}

impl Display for FrozenArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "args")
    }
}

#[starlark_value(type = "Args")]
impl<'v> StarlarkValue<'v> for Args<'v> {
    type Canonical = FrozenArgs;

    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new("Args", |builder| {
            args_methods(builder);
        });
        Some(RES.methods())
    }
}

#[starlark_value(type = "Args")]
impl<'v> StarlarkValue<'v> for FrozenArgs {
    type Canonical = Self;

    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new("Args", |builder| {
            args_methods(builder);
        });
        Some(RES.methods())
    }
}

impl<'v> Freeze for Args<'v> {
    type Frozen = FrozenArgs;

    fn freeze(self, freezer: &Freezer) -> FreezeResult<Self::Frozen> {
        Ok(FrozenArgs {
            arguments: self
                .arguments
                .into_inner()
                .into_iter()
                .map(|arg| arg.freeze(freezer))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

// Inline it to ensure that the error doesn't actually need to be passed as a
// function parameter.
#[inline]
fn arg_name_and_value<'v>(
    arg_name_or_value: Value<'v>,
    value: Option<Value<'v>>,
    err: Error,
) -> starlark::Result<(Option<String>, Value<'v>)> {
    if let Some(val) = value {
        if let Some(arg_name) = arg_name_or_value.unpack_str() {
            Ok((Some(arg_name.to_owned()), val))
        } else {
            Err(starlark::Error::from(err))
        }
    } else {
        Ok((None, arg_name_or_value))
    }
}

fn get_mutable_args<'v>(
    this: Value<'v>,
) -> starlark::Result<std::cell::RefMut<'v, Vec<ArgValue<Value<'v>>>>> {
    if let Some(args) = this.downcast_ref::<Args<'v>>() {
        Ok(args.arguments.borrow_mut())
    } else if this.downcast_ref::<FrozenArgs>().is_some() {
        Err(starlark::Error::new_other(Error::CannotMutateFrozenArgs))
    } else {
        unreachable!();
    }
}

/// Registers the Starlark methods of the `Args` class (`add`, `add_all`,
/// `add_joined`).
#[starlark_module]
pub fn args_methods(builder: &mut MethodsBuilder) {
    fn add<'v>(
        this: Value<'v>,
        arg_name_or_value: Value<'v>,
        value: Option<Value<'v>>,
        #[starlark(require = named)] format: Option<Formatter>,
    ) -> starlark::Result<Value<'v>> {
        let mut args = get_mutable_args(this)?;

        let (arg_name, val) =
            arg_name_and_value(arg_name_or_value, value, Error::ExpectedAddStringFlag)?;

        args.push(ArgValue::Scalar {
            arg_name,
            value: val,
            format,
        });
        Ok(this)
    }

    fn add_all<'v>(
        this: Value<'v>,
        arg_name_or_values: Value<'v>,
        values: Option<Value<'v>>,
        #[starlark(require = named)] map_each: Option<Value<'v>>,
        #[starlark(require = named)] format_each: Option<Formatter>,
        #[starlark(require = named)] before_each: Option<&str>,
        #[starlark(require = named)] terminate_with: Option<&str>,
        #[starlark(require = named, default = true)] omit_if_empty: bool,
        #[starlark(require = named, default = false)] uniquify: bool,
        #[starlark(require = named)] allow_closure: Option<bool>,
    ) -> starlark::Result<Value<'v>> {
        // See a comment on the error message for more details on why this is needed.
        if map_each.is_some() && allow_closure.is_none() {
            return Err(Error::MapEachRequiresAllowClosure.into());
        }

        let mut args = get_mutable_args(this)?;

        let (flag, values) =
            arg_name_and_value(arg_name_or_values, values, Error::ExpectedAddAllStringFlag)?;

        args.push(ArgValue::All {
            flag,
            values,
            map_each,
            format_each,
            before_each: before_each.map(String::from),
            terminate_with: terminate_with.map(String::from),
            omit_if_empty,
            uniquify,
        });
        Ok(this)
    }

    fn add_joined<'v>(
        this: Value<'v>,
        arg_name_or_values: Value<'v>,
        values: Option<Value<'v>>,
        #[starlark(require = named)] join_with: &str,
        #[starlark(require = named)] map_each: Option<Value<'v>>,
        #[starlark(require = named)] format_each: Option<Formatter>,
        #[starlark(require = named)] format_joined: Option<Formatter>,
        #[starlark(require = named, default = true)] omit_if_empty: bool,
        #[starlark(require = named, default = false)] uniquify: bool,
        #[starlark(require = named)] allow_closure: Option<bool>,
    ) -> starlark::Result<Value<'v>> {
        // See a comment on the error message for more details on why this is needed.
        if map_each.is_some() && allow_closure.is_none() {
            return Err(Error::MapEachRequiresAllowClosure.into());
        }

        let mut args = get_mutable_args(this)?;

        let (flag, values) = arg_name_and_value(
            arg_name_or_values,
            values,
            Error::ExpectedAddJoinedStringFlag,
        )?;

        args.push(ArgValue::Joined {
            flag,
            values,
            join_with: join_with.to_owned(),
            map_each,
            format_each,
            format_joined,
            omit_if_empty,
            uniquify,
        });
        Ok(this)
    }
}

impl<'v> FrozenArgs {
    /// Expands the stored arguments list into command-line arguments and input
    /// files.
    pub fn expand(&self, eval: &mut Evaluator<'v, '_, '_>) -> starlark::Result<Vec<String>> {
        let mut command = Vec::new();
        crate::expand::expand_into(&mut command, self, eval)?;
        Ok(command)
    }
}
