// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// The consolidated cxx FFI bridge defining all shared C++ classes, structs,
/// methods, and constructors utilized by the high-level Rust wrappers.
///
/// This file does several things:
/// * It generates types usable by rust.
/// * The `cxxbridge --header` command can be ran to re-generate the C++
///   headers.
///   * This allows for C++ code to #include rust types
/// * The `cxxbridge` command generates shims to allow us to use C++ types in
///   rust.
use crate::session::Session;

#[cxx::bridge]
// Allow let_underscore_drop because the cxx::bridge generated code has non-binding
// lets on C++ types with destructors.
#[allow(let_underscore_drop)]
// CxxBridge requires a module, but we don't want one. So we make a private one
// and re-export all fields.
mod dummy {
    struct Any {
        _private: u8,
    }

    // A &[T] compatible with both opaque and non-opaque types.
    #[derive(Clone, Copy)]
    struct SliceAny {
        len: usize,
        ptr: *mut Any,
    }

    struct KeyValue<'a> {
        key: &'a str,
        value: &'a Value,
    }

    // cxxbridge marks any function that takes a raw pointer unsafe.
    // By providing a thin wrapper around the pointer, we can remove the unsafe.
    #[derive(Clone, Copy, Default)]
    struct ParseNodePtr {
        ptr: *const ParseNode,
    }

    #[derive(Clone, Copy)]
    enum ValueType {
        None = 0,
        Boolean = 1,
        Integer = 2,
        String = 3,
        List = 4,
        Scope = 5,
    }
    unsafe extern "C++" {
        // include! simply tells cxxbridge to put the #include in the generated C++
        // source code. It does not do anything on the rust side.
        include!("gn/err.h");
        include!("gn/ffi/err.h");
        include!("gn/ffi/scope.h");
        include!("gn/ffi/test_with_scope.h");
        include!("gn/ffi/value.h");
        include!("gn/label.h");
        include!("gn/output_file.h");
        include!("gn/scope.h");
        include!("gn/settings.h");
        include!("gn/source_dir.h");
        include!("gn/test_with_scope.h");
        include!("gn/value.h");

        type Err;
        pub fn has_error(self: &Err) -> bool;
        // Dead code for production, used in tests only
        #[allow(dead_code)]
        pub fn NewErr() -> UniquePtr<Err>;
        // Dead code for production, used in tests only
        #[allow(dead_code)]
        pub(in crate::err) fn ErrToString(err: &Err) -> String;

        pub(in crate::err) fn PopulateErrWithLocation(
            err: Pin<&mut Err>,
            message: &str,
            help: &str,
            file: &InputFile,
            start_line: i32,
            start_column: i32,
            end_line: i32,
            end_column: i32,
        );
        pub(in crate::err) fn PopulateErrWithMessage(err: Pin<&mut Err>, message: &str, help: &str);
        pub(in crate::err) fn AppendSubErr(
            err: Pin<&mut Err>,
            message: &str,
            file: &InputFile,
            start_line: i32,
            start_column: i32,
            end_line: i32,
            end_column: i32,
        );

        type InputFile;
        pub(in crate::err) fn NewInputFile<'a, 'b>(name: &'a str, code: &'a str) -> &'b InputFile;

        type OutputFile;
        #[cxx_return_type = "std::string_view"]
        pub(in crate::output_file) fn value(self: &OutputFile) -> &str;

        type SourceDir;
        #[cxx_return_type = "std::string_view"]
        pub(in crate::label) fn SourceWithNoTrailingSlash(self: &SourceDir) -> &str;

        type Label;
        pub(in crate::label) fn dir(self: &Label) -> &SourceDir;
        #[cxx_return_type = "const std::string&"]
        pub fn name(self: &Label) -> &str;

        type Settings;
        pub(in crate::settings) fn toolchain_label(self: &Settings) -> &Label;

        type Scope;
        // Constructs a new child Scope, populates placeholder Values for the given
        // keys, and returns an owned slice of references to the placeholders.
        // For example, NewScope(&scope, ["foo", "bar"]) would return
        // [scope["foo"], scope["bar"]].
        // The caller is then responsible for filling in the values as needed.
        pub(in crate::scope) fn NewScope(
            parent_scope: &Scope,
            keys: &[&str],
            out_scope: &mut UniquePtr<Scope>,
        ) -> SliceAny;
        pub(in crate::scope) fn NewStruct(
            settings: &Settings,
            keys: &[&str],
            out_scope: &mut UniquePtr<Scope>,
        ) -> SliceAny;
        // Returns an OwnedSlice<KeyValue> corresponding to references to each element.
        pub(in crate::scope) fn GetScopeItems(scope: &Scope) -> SliceAny;
        pub(in crate::scope) fn GetValue(scope: &Scope, ident: &str) -> *const Value;
        pub(in crate::scope) fn SetValue<'a>(
            scope: Pin<&'a mut Scope>,
            ident: &str,
            origin: ParseNodePtr,
        ) -> Pin<&'a mut Value>;
        #[rust_name = "settings_cxx"]
        pub(in crate::scope) fn settings(self: &Scope) -> *const Settings;

        type TestWithScope;
        pub(in crate::test_with_scope) fn NewTestWithScope() -> UniquePtr<TestWithScope>;
        #[rust_name = "scope_cxx"]
        pub(in crate::test_with_scope) fn scope(self: Pin<&mut TestWithScope>) -> *mut Scope;

        type Value;
        type ParseNode;
        // We allow dead code because this isn't used in production and we
        // can't tag things in the bridge with cfg(test).
        #[allow(dead_code)]
        pub(in crate::value) fn NewValueForTesting() -> UniquePtr<Value>;
        pub(in crate::value) fn ValueSize() -> usize;
        #[cxx_return_type = "Value::Type"]
        #[cxx_name = "type"]
        // We can't call this "type" in rust since it's a keyword.
        pub(in crate::value) fn kind(self: &Value) -> ValueType;
        pub(in crate::value) fn boolean_value(self: &Value) -> &bool;
        pub(in crate::value) fn int_value(self: &Value) -> &i64;
        #[cxx_return_type = "const std::string&"]
        pub(in crate::value) fn string_value(self: &Value) -> &str;
        #[cxx_name = "GetValueList"]
        pub(in crate::value) fn list_value_cxx(val: &Value) -> SliceAny;
        pub(in crate::value) fn scope_value(self: &Value) -> *const Scope;
        pub(in crate::value) fn SetValueNone(val: Pin<&mut Value>, origin: ParseNodePtr);
        pub(in crate::value) fn SetValueBool(val: Pin<&mut Value>, origin: ParseNodePtr, b: bool);
        pub(in crate::value) fn SetValueInt(val: Pin<&mut Value>, origin: ParseNodePtr, i: i64);
        pub(in crate::value) fn SetValueString(val: Pin<&mut Value>, origin: ParseNodePtr, s: &str);
        // Initialises self as a list of `size` elements and returns a pointer to the
        // start.
        pub(in crate::value) fn SetValueList(
            val: Pin<&mut Value>,
            origin: ParseNodePtr,
            size: usize,
        ) -> *mut Any;
        pub(in crate::value) fn SetValueScope(
            val: Pin<&mut Value>,
            origin: ParseNodePtr,
            scope: UniquePtr<Scope>,
        );
    }

    extern "Rust" {
        type Session;

        #[Self = "Session"]
        #[cxx_name = "new_cxx"]
        fn new(source_root: &str, source_root_rel: &str) -> Box<Session>;

        #[Self = "Session"]
        fn new_for_testing() -> Box<Session>;

        fn load_values(
            self: &'static Session,
            label: &str,
            relative_to: &str,
            keys: &[&str],
            scope: Pin<&mut Scope>,
            settings: &Settings,
            origin: ParseNodePtr,
            err: Pin<&mut Err>,
        );
    }
}

pub use dummy::*;
