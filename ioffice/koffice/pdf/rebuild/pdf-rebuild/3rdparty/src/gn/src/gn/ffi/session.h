// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_FFI_SESSION_H_
#define TOOLS_GN_FFI_SESSION_H_

#include <span>

class Err;
class ParseNode;
struct Session;
class Scope;
class Value;

struct ParseNodePtr;

// Executes the starlark file and loads the requested variables.
// Example: session_load(
//   session,
//   Value(":foo.scl"),
//   {"foo", "bar"}
//   scope for //dir/BUILD.gn,
//   ParseNodePtr{parse_node},
//   err
// )
//
// In this example, we execute the file //dir/foo.scl.
// We then get the variables foo and bar and insert them into the dest_scope.
// Unlike `import`, all imported variables *must* be used.
bool session_load(const Session& session,
                  const Value& label,
                  std::span<const Value> keys,
                  Scope& dest_scope,
                  ParseNodePtr parse_node,
                  Err& err);

#endif  // TOOLS_GN_FFI_SESSION_H_