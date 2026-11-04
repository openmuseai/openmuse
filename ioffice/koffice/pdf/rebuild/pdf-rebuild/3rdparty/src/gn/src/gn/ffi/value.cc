//  Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/ffi/value.h"

#include <new>
#include <string>

#include "gn/ffi/bridge.h"
#include "gn/ffi/slice.h"

namespace rust {
ValueType cxx_to_rust(Value::Type t) {
  return static_cast<ValueType>(t);
}
}  // namespace rust

size_t ValueSize() {
  return sizeof(Value);
}

void SetValueNone(Value& self, ParseNodePtr origin) {
  new (&self) Value(origin.ptr, Value::NONE);
}

void SetValueBool(Value& self, ParseNodePtr origin, bool b) {
  new (&self) Value(origin.ptr, b);
}

void SetValueInt(Value& self, ParseNodePtr origin, int64_t i) {
  new (&self) Value(origin.ptr, i);
}

void SetValueString(Value& self, ParseNodePtr origin, rust::Str s) {
  new (&self) Value(origin.ptr, std::string(s.data(), s.size()));
}

Any* SetValueList(Value& self, ParseNodePtr origin, size_t size) {
  new (&self) Value(origin.ptr, Value::LIST);
  self.list_value().resize(size);
  return reinterpret_cast<Any*>(self.list_value().data());
}

void SetValueScope(Value& self,
                   ParseNodePtr origin,
                   std::unique_ptr<Scope> scope) {
  new (&self) Value(origin.ptr, std::move(scope));
}

SliceAny GetValueList(const Value& self) {
  return AsSlice(self.list_value());
}

std::unique_ptr<Value> NewValueForTesting() {
  return std::make_unique<Value>();
}
