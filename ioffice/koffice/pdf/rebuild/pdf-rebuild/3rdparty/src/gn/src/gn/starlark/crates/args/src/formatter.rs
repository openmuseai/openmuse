// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use allocative::Allocative;
use starlark::{
    typing::Ty,
    values::{type_repr::StarlarkTypeRepr, Freeze, FreezeResult, Freezer},
};

use crate::errors::Error;

/// Helper to format argument values using a template containing `%s`.
#[derive(Debug, Clone, Allocative)]
pub struct Formatter {
    before: String,
    after: String,
}

fn consume_partial(chars: &mut std::str::Chars<'_>, fmt: &str) -> starlark::Result<(String, bool)> {
    let mut s = String::new();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('%') => s.push('%'),
                Some('s') => return Ok((s, false)),
                _ => return Err(Error::InvalidFormatString(fmt.to_owned()).into()),
            }
        } else {
            s.push(c);
        }
    }
    Ok((s, true))
}

impl Formatter {
    /// Parses a format string and returns a `Formatter` if valid (must contain
    /// exactly one `%s`). Literal percents may be escaped as `%%`.
    pub fn new(fmt: &str) -> starlark::Result<Self> {
        let mut chars = fmt.chars();
        let (before, eof) = consume_partial(&mut chars, fmt)?;
        if eof {
            return Err(Error::InvalidFormatString(fmt.to_owned()).into());
        }
        let (after, eof) = consume_partial(&mut chars, fmt)?;
        if !eof {
            return Err(Error::InvalidFormatString(fmt.to_owned()).into());
        }
        Ok(Self { before, after })
    }

    /// Formats the string by replacing `%s` with the input string.
    pub fn format(&self, s: &str) -> String {
        format!("{}{}{}", self.before, s, self.after)
    }
}

impl Freeze for Formatter {
    type Frozen = Self;

    fn freeze(self, _freezer: &Freezer) -> FreezeResult<Self::Frozen> {
        Ok(self)
    }
}

impl StarlarkTypeRepr for Formatter {
    type Canonical = String;

    fn starlark_type_repr() -> Ty {
        String::starlark_type_repr()
    }
}

#[cfg(test)]
mod tests {
    use super::Formatter;

    // For consistency with bazel, test cases copied directly from:
    // https://github.com/bazelbuild/bazel/blob/master/src/test/java/com/google/devtools/build/lib/actions/SingleStringArgFormatterTest.java
    #[test]
    fn test_valid_formats() {
        let cases = [
            ("hello %s", "hello world"),
            ("%s hello", "world hello"),
            ("hello %s, hello", "hello world, hello"),
            ("hello %%s %s", "hello %s world"),
        ];
        for (fmt, expected) in cases {
            let f = Formatter::new(fmt).unwrap();
            assert_eq!(f.format("world"), expected);
        }
    }

    #[test]
    fn test_invalid_formats() {
        let cases = [
            "hello",
            "hello %%s",
            "hello %s %s",
            "%s hello %s",
            "hello %",
            "hello %f",
            "hello %s %f",
            "hello %s %",
        ];
        for fmt in cases {
            assert!(
                Formatter::new(fmt).is_err(),
                "Expected invalid format: {fmt}"
            );
        }
    }
}
