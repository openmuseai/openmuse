// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::pin::Pin;

use crate::bridge::Err as CxxErr;

impl CxxErr {
    /// Translates a Starlark evaluation result to a C++ GN error if it failed.
    ///
    /// If the result is Ok, returns `Some(v)`.
    /// If the result is Err, populates this C++ `Err` object and returns
    /// `None`.
    pub fn handle<T>(self: Pin<&mut Self>, result: starlark::Result<T>) -> Option<T> {
        match result {
            Ok(v) => Some(v),
            Err(e) => {
                self.fill(&e);
                None
            },
        }
    }

    /// Populates this C++ `Err` object with the details of the given Starlark
    /// error.
    pub fn fill(mut self: Pin<&mut Self>, err: &starlark::Error) {
        // The "diagnostic" contains the location.
        // Since the GN error type already deals with the location, instead of
        // having the diagnostic in the error message, we just pass the metadata to gn.
        let message = err.without_diagnostic().to_string();

        // This is the root cause of the error.
        let help = err
            .kind()
            .source()
            .map(|e| e.to_string())
            .unwrap_or_default();

        if let Some(span) = err.span() {
            let filename = span.filename();
            let source = span.file.source();
            let resolved = span.file.resolve_span(span.span);

            // Ensure the file is registered in C++ InputFileManager.
            let file_ref = crate::bridge::NewInputFile(filename, source);

            crate::bridge::PopulateErrWithLocation(
                self.as_mut(),
                &message,
                &help,
                file_ref,
                // GN is 1-indexed, starlark-rs is 0-indexed.
                resolved.begin.line as i32 + 1,
                resolved.begin.column as i32 + 1,
                resolved.end.line as i32 + 1,
                resolved.end.column as i32 + 1,
            );
        } else {
            crate::bridge::PopulateErrWithMessage(self.as_mut(), &message, &help);
        }

        self.fill_frames(err);
    }

    fn fill_frames(mut self: Pin<&mut Self>, err: &starlark::Error) {
        // Process call stack frames and append them as nested errors.
        for frame in err.call_stack().frames.iter().rev() {
            if let Some(loc) = &frame.location {
                let frame_filename = loc.filename();
                let frame_source = loc.file.source();
                let frame_resolved = loc.file.resolve_span(loc.span);

                // Note: This is not de-duplicated. If you have a 10-deep stack trace in the
                // same file, you will have the same InputFile duplicated 10 times in
                // InputFileManager. We don't really care though, especially because starlark
                // bans recursion.
                let frame_file_ref = crate::bridge::NewInputFile(frame_filename, frame_source);
                // Regular GN writes "whence it was called". In starlark, you can do things
                // like: ```
                // def foo():
                //   ...
                // bar = foo
                // bar()
                // ```
                // so rather than "it", we actually specify the name of the function that was
                // called.
                let frame_msg = format!("whence '{}' was called.", frame.name);

                crate::bridge::AppendSubErr(
                    self.as_mut(),
                    &frame_msg,
                    frame_file_ref,
                    // GN is 1-indexed, starlark-rs is 0-indexed.
                    frame_resolved.begin.line as i32 + 1,
                    frame_resolved.begin.column as i32 + 1,
                    frame_resolved.end.line as i32 + 1,
                    frame_resolved.end.column as i32 + 1,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[track_caller]
    fn assert_err_eq(value: starlark::Result<()>, want: &[&str]) {
        assert!(value.is_err());
        let mut err = crate::bridge::NewErr();
        err.pin_mut().handle(value);
        assert!(err.has_error());
        assert_eq!(crate::bridge::ErrToString(&err), want.join("\n") + "\n");
    }

    fn eval_starlark(code: &str) -> starlark::Result<()> {
        let globals = starlark::environment::Globals::standard();
        starlark::environment::Module::with_temp_heap(|module| {
            let mut eval = starlark::eval::Evaluator::new(&module);
            let ast = starlark::syntax::AstModule::parse(
                "//test.scl",
                code.to_owned(),
                &starlark::syntax::Dialect::Standard,
            )
            .unwrap();
            eval.eval_module(ast, &globals).map(|_| ())
        })
    }

    #[test]
    fn test_call_stack_error() {
        let _setup = crate::TestWithScope::new();
        let code = r#"
def foo():
  1 + "a"

foo()
"#;
        let res = eval_starlark(code);
        assert_err_eq(
            res,
            &[
                r#"ERROR at //test.scl:3:3: Operation `+` not supported for types `int` and `string`"#,
                r#"  1 + "a""#,
                r#"  ^------"#,
                r#"See //test.scl:5:1: whence 'foo' was called."#,
                r#"foo()"#,
                r#"^----"#,
            ],
        );
    }
}
