// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <string_view>

#include "cxx.h"
#include "gn/string_atom.h"

// `intern_string` is located in its own translation unit (`intern_string.cc`)
// to prevent link-time dependencies on the `ctx` crate for standalone cargo
// unit tests.
//
// By isolating `intern_string` in its own object file (`intern_string.o`), the
// linker only pulls in this file and does not pull in `cxx_api.o`.
extern "C" rust::Str intern_string(rust::Str s) {
  StringAtom atom(std::string_view{s});
  return rust::Str(atom.str());
}
