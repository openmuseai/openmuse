// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use starlark::{
    typing::Ty,
    values::{
        list::UnpackList, type_repr::StarlarkTypeRepr, Freeze, FreezeError, Freezer, UnpackValue,
        Value,
    },
};
use types::{Label, PackageRef, PathResolver};

use crate::attr::LabelOrFile;

/// The rust type for the starlark value passed to attr.label(allow_files = ...)
#[derive(Debug, Clone, PartialEq, Eq, allocative::Allocative)]
pub enum AllowFiles {
    None,
    All,
    Some(Vec<String>),
}

impl StarlarkTypeRepr for AllowFiles {
    type Canonical = either::Either<bool, UnpackList<String>>;

    fn starlark_type_repr() -> Ty {
        Self::Canonical::starlark_type_repr()
    }
}

impl<'v> UnpackValue<'v> for AllowFiles {
    type Error = starlark::Error;

    fn unpack_value_impl(value: Value<'v>) -> Result<Option<Self>, Self::Error> {
        Ok(Self::Canonical::unpack_value(value)?.map(|c| match c {
            either::Either::Left(false) => Self::None,
            either::Either::Left(true) => Self::All,
            either::Either::Right(list) => Self::Some(list.items),
        }))
    }
}

impl AllowFiles {
    pub(crate) fn matches(&self, path: &str) -> bool {
        match self {
            Self::None => false,
            Self::All => true,
            Self::Some(exts) => {
                let p = std::path::Path::new(path);
                let file_name = p.file_name().and_then(|e| e.to_str()).unwrap_or("");
                exts.iter().any(|ext| {
                    if ext.starts_with('.') {
                        file_name.ends_with(ext)
                    } else {
                        file_name
                            .strip_suffix(ext)
                            .is_some_and(|prefix| prefix.is_empty() || prefix.ends_with('.'))
                    }
                })
            },
        }
    }

    pub(crate) fn validate(&self, path: &str) -> starlark::Result<()> {
        match self {
            Self::None => Err(crate::Error::NotALabel(path.to_owned()).into()),
            Self::All => Ok(()),
            Self::Some(exts) => {
                if self.matches(path) {
                    Ok(())
                } else {
                    Err(crate::Error::DisallowedExtension {
                        file: std::path::Path::new(path).to_path_buf(),
                        allowed: exts.clone(),
                    }
                    .into())
                }
            },
        }
    }
}

impl Freeze for AllowFiles {
    type Frozen = Self;

    fn freeze(self, _freezer: &Freezer) -> Result<Self::Frozen, FreezeError> {
        Ok(self)
    }
}

pub(crate) fn parse_label_like(
    s: &str,
    allow_files: &AllowFiles,
    relative_to: &PackageRef,
    path_resolver: &PathResolver,
) -> starlark::Result<LabelOrFile> {
    Ok(
        if let Some(label) = Label::parse_maybe_label(s, relative_to)? {
            LabelOrFile::Label(label)
        } else {
            // It's a file.
            allow_files.validate(s)?;
            LabelOrFile::File(path_resolver.source_file(relative_to, s)?)
        },
    )
}

#[cfg(test)]
mod tests {
    use starlark::values::FrozenHeap;

    use super::*;
    use crate::{
        cfg::AttrCfg,
        schema::{AllowFilesSchema, AttrKind, AttrSchema},
        Attr,
    };

    #[test]
    fn test_allow_files_matching() {
        let path_resolver = types::PathResolver::new_for_testing();
        let heap = FrozenHeap::new();

        let check_match = |pattern: &str, file: &str| -> bool {
            let schema = AttrSchema {
                kind: AttrKind::Label,
                default: None,
                disallow_empty: false,
                allow_files: AllowFilesSchema::Many(AllowFiles::Some(vec![pattern.to_owned()])),
                cfg: AttrCfg::CurrentToolchain,
                doc: String::new(),
            };
            Attr::create(
                &schema,
                Some(Value::new_frozen(heap.alloc(file))),
                PackageRef::new("//allow_files").unwrap(),
                &path_resolver,
            )
            .is_ok()
        };

        assert!(check_match(".cc", "file.cc"));
        assert!(check_match(".cc", "subdir/file.cc"));
        assert!(!check_match(".cc", "file.h"));
        assert!(!check_match(".cc", "cc"));
        assert!(!check_match(".cc", "nonexistent.cc"));

        assert!(check_match("cc", "file.cc"));
        assert!(!check_match("cc", "file.h"));
        assert!(check_match("cc", "cc"));

        assert!(check_match("foo.cc", "foo.cc"));
        assert!(check_match("foo.cc", "test.foo.cc"));
        assert!(!check_match("foo.cc", "bar.cc"));
    }
}
