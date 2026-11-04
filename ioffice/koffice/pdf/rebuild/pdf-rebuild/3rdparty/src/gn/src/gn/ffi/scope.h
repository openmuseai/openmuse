// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_FFI_SCOPE_H_
#define TOOLS_GN_FFI_SCOPE_H_

#include <memory>

#include "cxx.h"

struct ParseNodePtr;
class Scope;
class Settings;
struct SliceAny;
class Value;

// Constructs a new child Scope, populates placeholder Values for the given
// keys, and returns a "std::vector<Value&>" where vec[i] is the value for
// keys[i].
//
// Safety: Rust is required to convert this to an OwnedSlice<&Value>.
SliceAny NewScope(const Scope& parent_scope,
                  rust::Slice<const rust::Str> keys,
                  std::unique_ptr<Scope>& out_scope);

// Constructs a new "struct", populates placeholder Values for the given
// keys, and returns a "std::vector<Value&>" where vec[i] is the value for
// keys[i].
//
// A "struct" is a scope which can only be used for dot-lookup. It has no
// parent scope, as that would allow struct.foo to lookup the parent
// scope's foo.
//
// Safety: Rust is required to convert this to an OwnedSlice<&Value>.
SliceAny NewStruct(const Settings& settings,
                   rust::Slice<const rust::Str> keys,
                   std::unique_ptr<Scope>& out_scope);

// Returns a "std::vector<KeyValue>"-like object.
//
// Safety: Rust is required to convert this to an OwnedSlice<KeyValue>.
SliceAny GetScopeItems(const Scope& scope);

// Returns a pointer to the value in the scope or nullptr if not found.
const Value* GetValue(const Scope& scope, rust::Str ident);

// Adds a value slot to the scope under `ident` and returns a reference to it.
Value& SetValue(Scope& scope, rust::Str ident, ParseNodePtr origin);

#endif  // TOOLS_GN_FFI_SCOPE_H_
