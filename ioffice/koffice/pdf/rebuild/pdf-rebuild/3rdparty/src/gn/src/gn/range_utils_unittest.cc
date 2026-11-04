// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/range_utils.h"

#include <ranges>
#include <vector>

#include "util/test/test.h"

TEST(RangeUtilsTest, ToVec) {
  // Test converting a common range (like a transformed vector) to vector.
  std::vector<int> input = {1, 2, 3, 4, 5};
  std::vector<int> output =
      to_vec(input | std::views::transform([](int i) { return i + 1; }));
  ASSERT_EQ(output, (std::vector<int>{2, 3, 4, 5, 6}));

  // Test converting a non-common range (like a take_view of an infinite
  // iota_view) to vector.
  std::vector<int> output_non_common =
      to_vec(std::views::take(std::views::iota(0), 5));
  ASSERT_EQ(output_non_common, (std::vector<int>{0, 1, 2, 3, 4}));
}
