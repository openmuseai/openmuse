use starlark::{environment::GlobalsBuilder, eval::Evaluator, values::Value};
use starlark_derive::starlark_module;

use crate::{BuiltinProviders, Error};

/// Registers the global `provider()` function.
#[starlark_module]
pub(crate) fn register_providers_globals(builder: &mut GlobalsBuilder) {
    fn provider<'v>(
        #[starlark(require = named)] fields: Value<'v>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        let mut field_names = Vec::new();
        for field in fields
            .iterate(eval.heap())
            .map_err(|_| Error::FieldsMustBeIterable)?
        {
            let s = field
                .unpack_str()
                .ok_or(Error::FieldsMustBeStrings)?
                .to_owned();
            field_names.push(s);
        }

        let provider_type = crate::provider_type::ProviderType::new(field_names)?;
        Ok(eval.heap().alloc_complex(provider_type))
    }
}

pub fn register_providers(builder: &mut GlobalsBuilder) -> BuiltinProviders {
    register_providers_globals(builder);
    crate::builtins::register_builtin_providers(builder)
}
