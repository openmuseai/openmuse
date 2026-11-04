// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_FFI_VALUE_H_
#define TOOLS_GN_FFI_VALUE_H_

#include <memory>

#include "cxx.h"
#include "gn/value.h"

class Scope;
struct ParseNodePtr;
struct SliceAny;

enum class ValueType : uint8_t;

// Teach rust how to convert Value::Type to an enum that rust is aware of.
namespace rust {
ValueType cxx_to_rust(Value::Type t);
}

size_t ValueSize();
// SetValue* is called with potentially uninitialized Value objects.
// These functions roughly correspond to calling the corresponding constructor
// with in-place construction.
void SetValueNone(Value& self, ParseNodePtr origin);
void SetValueBool(Value& self, ParseNodePtr origin, bool b);
void SetValueInt(Value& self, ParseNodePtr origin, int64_t i);
void SetValueString(Value& self, ParseNodePtr origin, rust::Str s);
struct Any;
// Sets the value to a list of `size` elements. Returns a pointer to the start
// of the vector.
//
// Safety: Rust is required to convert this to a Slice<Value>(pointer, size)
Any* SetValueList(Value& self, ParseNodePtr origin, size_t size);
void SetValueScope(Value& self,
                   ParseNodePtr origin,
                   std::unique_ptr<Scope> scope);
// Returns a "std::vector<Value>".
//
// Safety: Rust is required to convert this to a Slice<Value>.
SliceAny GetValueList(const Value& self);

// GN values are never created in starlark in production code.
// If a value may ever be returned, it will be passed as a mutable output
// parameter.
std::unique_ptr<Value> NewValueForTesting();

#endif  // TOOLS_GN_FFI_VALUE_H_
