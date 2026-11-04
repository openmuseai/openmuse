// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_FFI_SLICE_H_
#define TOOLS_GN_FFI_SLICE_H_

#include <array>
#include <cstdint>
#include <type_traits>
#include <vector>

#include "gn/ffi/bridge.h"

// Consumes a std::vector<T> to return a Slice to rust.
// This is akin to unique_pointer.release(), where we give up ownership of the
// slice and pass it to rust.
//
// Safety: Rust is *required* to cast this to an OwnedSlice<T>.
// This will guaruntee that the vector is destroyed.
template <typename T>
inline SliceAny IntoSlice(std::vector<T> vec) {
  // Rust knows how to free the slice itself (just call free on the pointer),
  // but does not know how to call the destructor on individual elements.
  static_assert(std::is_trivially_destructible_v<T>,
                "T must be trivially destructible to avoid leaks");
  if (vec.empty()) {
    return SliceAny{0, nullptr};
  }
  SliceAny slice{vec.size(), reinterpret_cast<Any*>(vec.data())};

  // Construct on stack buffer to prevent C++ compiler from running destructor
  // on vec
  std::array<uint8_t, sizeof(std::vector<T>)> buf;
  new (&buf) std::vector<T>(std::move(vec));

  return slice;
}

// Returns a view of a std::vector, which may or may not be const.
//
// Safety: This uses const_cast to cast away constness because SliceAny is a
// unified FFI representation that uses a mutable Any* pointer (which is
// required to support both mutable and immutable slices in Rust).
//
// If the API intends to return immutable objects, the rust caller is
// responsible for wrapping the API in a function that returns Immutable<T>.
template <typename T>
inline SliceAny AsSlice(const std::vector<T>& vec) {
  return SliceAny{vec.size(),
                  reinterpret_cast<Any*>(const_cast<T*>(vec.data()))};
}

#endif  // TOOLS_GN_FFI_SLICE_H_
