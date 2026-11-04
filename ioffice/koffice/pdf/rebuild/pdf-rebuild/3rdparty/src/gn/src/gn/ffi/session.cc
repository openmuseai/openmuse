// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/ffi/session.h"

#include <vector>

#include "gn/build_settings.h"
#include "gn/err.h"
#include "gn/ffi/bridge.h"
#include "gn/functions.h"
#include "gn/parse_tree.h"
#include "gn/scope.h"
#include "gn/source_dir.h"
#include "gn/tokenizer.h"

struct Session;

namespace {

bool IsValidIdentifier(std::string_view str) {
  if (str.empty())
    return false;
  if (!Tokenizer::IsIdentifierFirstChar(str[0]))
    return false;
  for (char c : str.substr(1)) {
    if (!Tokenizer::IsIdentifierContinuingChar(c))
      return false;
  }
  return true;
}

}  // namespace

bool session_load(const Session& session,
                  const Value& label,
                  std::span<const Value> keys,
                  Scope& dest_scope,
                  ParseNodePtr parse_node,
                  Err& err) {
  if (label.type() != Value::STRING) {
    err = Err(label.origin(), "Invalid load path.",
              "First argument to load must be a string corresponding to the "
              "label for the file to load.");
    return false;
  }
  std::string_view label_str = label.string_value();
  if (!label_str.ends_with(".scl")) {
    err = Err(label.origin(), "The file to load must be a '.scl' file.");
    return false;
  }

  std::vector<rust::Str> keys_slice;
  keys_slice.reserve(keys.size());
  for (const auto& key : keys) {
    if (key.type() != Value::STRING) {
      err = Err(key.origin(), "Invalid variable to load.",
                "Arguments to load must be strings.");
      return false;
    }
    std::string_view val_str = key.string_value();
    if (!IsValidIdentifier(val_str)) {
      err = Err(key.origin(), "Invalid variable to load.",
                "Arguments to load must be valid identifiers.");
      return false;
    }
    keys_slice.push_back(rust::Str(StringAtom(val_str).str()));
  }

  session.load_values(
      rust::Str(label.string_value()),
      rust::Str(dest_scope.GetSourceDir().SourceWithNoTrailingSlash()),
      rust::Slice<const rust::Str>(keys_slice), dest_scope,
      *dest_scope.settings(), parse_node, err);

  return !err.has_error();
}
