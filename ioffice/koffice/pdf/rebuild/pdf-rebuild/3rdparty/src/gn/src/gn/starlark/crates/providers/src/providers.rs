// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use args::FrozenArgsSequence;
use depset::UnpackFileDepset;
use starlark::{
    collections::SmallMap,
    typing::Ty,
    values::{
        list::FrozenListRef, structs::FrozenStructRef, type_repr::StarlarkTypeRepr,
        typing::TypeInstanceId, FrozenHeapRef, FrozenValue, OwnedFrozenValue, UnpackValue as _,
        Value,
    },
};
use types::File;

use crate::ProviderInstance;

/// Helper to unpack a frozen list of providers to useful metadata.
#[derive(Debug, Default)]
pub struct Providers {
    /// The output files produced by this target, parsed from the `DefaultInfo`
    /// provider.
    ///
    /// `DefaultInfo` represents the default outputs of a target rule (similar
    /// to Bazel's `DefaultInfo`). In GN, it contains the `files` depset
    /// which lists the direct and transitive outputs of the rule.
    pub outputs_phony: Option<File>,

    /// Additional input files declared by this target, parsed from the
    /// `GnInputsInfo` provider.
    ///
    /// `GnInputsInfo` is a built-in provider used by target rules to declare
    /// additional, dynamic input files that the rule's tool dependencies or
    /// parent targets must track.
    pub inputs_phony: Option<File>,

    /// Command-line substitution variables propagated by this target, parsed
    /// from the `GnSubstitutionsInfo` provider.
    ///
    /// `GnSubstitutionsInfo` carries key-value substitutions (packaged as a
    /// struct) that are used by toolchains and command-line execution
    /// blocks to expand variables and flags dynamically.
    pub substitutions: SmallMap<&'static str, FrozenArgsSequence<'static>>,

    /// All provider instances mapped by their ProviderType's TypeInstanceId.
    pub value: SmallMap<TypeInstanceId, FrozenValue>,

    /// The frozen heap for the rule implementation, keeping substitutions and
    /// value alive.
    _heap: FrozenHeapRef,
}

impl StarlarkTypeRepr for Providers {
    type Canonical = Value<'static>;

    fn starlark_type_repr() -> Ty {
        Ty::list(Ty::any())
    }
}

impl TryFrom<OwnedFrozenValue> for Providers {
    type Error = starlark::Error;

    fn try_from(value: OwnedFrozenValue) -> Result<Self, Self::Error> {
        let mut providers = Self {
            _heap: value.owner().clone(),
            ..Default::default()
        };
        let list = <&FrozenListRef>::unpack_value_err(value.value())?;
        for &item in list.iter() {
            let instance = <&ProviderInstance<'static>>::unpack_value_err(item.to_value())?;

            if providers
                .value
                .insert(instance.provider_type.id, item)
                .is_some()
            {
                return Err(
                    crate::errors::Error::DuplicateProvider(instance.ty_name().to_owned()).into(),
                );
            }

            match instance.provider_type.id {
                crate::builtins::DEFAULT_INFO_ID => {
                    let files = instance.values[0].unwrap();
                    providers.outputs_phony = UnpackFileDepset::unpack_value_err(files)
                        .map_err(|_| {
                            crate::errors::Error::DefaultInfoFilesMustBeFileDepset(files.to_repr())
                        })?
                        .0;
                },
                crate::builtins::INPUTS_INFO_ID => {
                    let files = instance.values[0].unwrap();
                    providers.inputs_phony = UnpackFileDepset::unpack_value_err(files)
                        .map_err(|_| {
                            crate::errors::Error::GnInputsInfoFilesMustBeFileDepset(files.to_repr())
                        })?
                        .0;
                },
                crate::builtins::SUBSTITUTIONS_INFO_ID => {
                    // GnSubstitutionsInfo(substitutions = struct)
                    // Safety: We already checked it was frozen earlier.
                    let substitutions_val = instance.values[0].unwrap();
                    let substitutions_struct = FrozenStructRef::from_value(unsafe {
                        substitutions_val.unpack_frozen().unwrap_unchecked()
                    })
                    .ok_or_else(|| {
                        starlark::Error::from(
                            crate::errors::Error::GnSubstitutionsInfoSubstitutionsMustBeStruct(
                                substitutions_val.to_repr(),
                            ),
                        )
                    })?;

                    providers.substitutions = substitutions_struct
                        .iter()
                        .map(|(k, v)| {
                            Ok((
                                k.as_str(),
                                <FrozenArgsSequence>::unpack_value_err(v.to_value())?,
                            ))
                        })
                        .collect::<Result<_, starlark::Error>>()?;
                },
                _ => {},
            }
        }

        Ok(providers)
    }
}

#[cfg(test)]
mod tests {

    use types::File;

    use crate::Providers;

    fn new_assert() -> testutils::Assert {
        let mut a = testutils::Assert::default();

        a.modify_globals(|builder| {
            depset::depset_globals!(builder, testutils::eval_context::FakeEvalContext);
            let builtin_providers = crate::globals::register_providers(builder);
            // Note: In production code, the module would be preloaded instead of loading
            // into globals.
            builder.set(
                "GnInputsInfo",
                builtin_providers.module.get("GnInputsInfo").unwrap(),
            );
            builder.set(
                "GnSubstitutionsInfo",
                builtin_providers.module.get("GnSubstitutionsInfo").unwrap(),
            );
        });
        a
    }

    #[test]
    fn test_providers_unpacking() {
        let mut a = new_assert();

        let val = a.pass("[]");
        let providers = Providers::try_from(val).unwrap();
        assert_eq!(providers.outputs_phony, None);
        assert_eq!(providers.inputs_phony, None);
        assert!(providers.substitutions.is_empty());
        assert!(providers.value.is_empty());

        let custom_info_ty = a.pass("CustomInfo = provider(fields = ['foo']); CustomInfo");
        a.modify_globals(move |builder| {
            builder.set("CustomInfo", custom_info_ty.clone());
        });

        let val = a.pass(
            r#"[
    DefaultInfo(files = depset([make_file("a")])),
    GnInputsInfo(files = depset([make_file("b")])),
    GnSubstitutionsInfo(substitutions = struct(key = ["val"])),
    CustomInfo(foo = 1),
]"#,
        );
        let providers = Providers::try_from(val).unwrap();

        assert_eq!(providers.outputs_phony, Some(File::intern("a")));
        assert_eq!(providers.inputs_phony, Some(File::intern("b")));

        let keys: Vec<&str> = providers.substitutions.keys().copied().collect();
        assert_eq!(keys, vec!["key"]);

        assert_eq!(providers.value.len(), 4);
        assert!(providers
            .value
            .contains_key(&crate::builtins::DEFAULT_INFO_ID));
        assert!(providers
            .value
            .contains_key(&crate::builtins::INPUTS_INFO_ID));
        assert!(providers
            .value
            .contains_key(&crate::builtins::SUBSTITUTIONS_INFO_ID));
    }

    #[track_caller]
    fn assert_unpack_fails(a: &mut testutils::Assert, expr: &str, expected_err: &str) {
        let val = a.pass(expr);
        let err = Providers::try_from(val).unwrap_err();
        assert_eq!(err.to_string(), expected_err);
    }

    #[test]
    fn test_providers_unpack_fails() {
        let mut a = new_assert();

        assert_unpack_fails(
            &mut a,
            "[DefaultInfo(files = depset()), DefaultInfo(files = depset())]",
            "Duplicate provider: DefaultInfo",
        );

        assert_unpack_fails(
            &mut a,
            r#"[DefaultInfo(files = "not-a-depset")]"#,
            r#"DefaultInfo.files must be a depset of files, got "not-a-depset""#,
        );

        assert_unpack_fails(
            &mut a,
            r#"[DefaultInfo(files = depset(["not-a-file"]))]"#,
            "DefaultInfo.files must be a depset of files, got depset(...)",
        );

        assert_unpack_fails(
            &mut a,
            r#"[GnInputsInfo(files = "not-a-depset")]"#,
            r#"GnInputsInfo.files must be a depset of files, got "not-a-depset""#,
        );

        assert_unpack_fails(
            &mut a,
            r#"[GnSubstitutionsInfo(substitutions = {"key": ["val"]})]"#,
            r#"GnSubstitutionsInfo.substitutions must be a struct, got {"key": ["val"]}"#,
        );

        assert_unpack_fails(
            &mut a,
            r#"[GnSubstitutionsInfo(substitutions = struct(key = "not-a-list"))]"#,
            r#"Expected `list`, but got `string (repr: "not-a-list")`"#,
        );

        assert_unpack_fails(
            &mut a,
            r#"[GnSubstitutionsInfo(substitutions = struct(key = [123]))]"#,
            "Expected `Args | str`, but got `int (repr: 123)`",
        );
    }
}
