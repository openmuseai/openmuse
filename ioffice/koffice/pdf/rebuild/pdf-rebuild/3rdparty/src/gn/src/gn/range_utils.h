// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_RANGE_UTILS_H_
#define TOOLS_GN_RANGE_UTILS_H_

#include <concepts>
#include <ranges>
#include <vector>

template <typename R, typename T>
concept RangeOf = std::ranges::input_range<R> &&
                  std::convertible_to<std::ranges::range_reference_t<R>, T>;

// std::ranges::to<std::vector> should be preferred, but isn't available on the
// older versions of mac used in CI.
template <std::ranges::input_range R>
auto to_vec(R&& range) {
  using ValType = std::ranges::range_value_t<R>;
  // For common ranges (where begin() and end() return the same type, such as
  // std::vector or simple views), we can use std::vector's iterator
  // constructor.
  if constexpr (std::ranges::common_range<R>) {
    return std::vector<ValType>(range.begin(), range.end());
  } else {
    // For non-common ranges (where begin() and end() have different types, such
    // as lazy split views or generators using sentinels), std::vector's
    // iterator constructor will fail to compile. We must use a range-based for
    // loop, which natively supports sentinel comparisons of different types.
    std::vector<ValType> vec;
    if constexpr (std::ranges::sized_range<R>) {
      vec.reserve(std::ranges::size(range));
    }
    for (auto&& item : range) {
      vec.push_back(std::forward<decltype(item)>(item));
    }
    return vec;
  }
}

#endif  // TOOLS_GN_RANGE_UTILS_H_
