// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/substitution_type.h"

#include <cstring>
#include <vector>

#include "util/test/test.h"

TEST(SubstitutionBitsTest, FillVectorDeterministicOrdering) {
  SubstitutionBits bits;
  bits.used.insert(&SubstitutionTargetGenDir);
  bits.used.insert(&SubstitutionSource);
  bits.used.insert(&SubstitutionOutput);
  bits.used.insert(&SubstitutionLabel);
  bits.used.insert(&SubstitutionBundleRootDir);

  std::vector<const Substitution*> vect;
  bits.FillVector(&vect);

  ASSERT_EQ(5u, vect.size());
  EXPECT_STREQ("{{bundle_root_dir}}", vect[0]->name);
  EXPECT_STREQ("{{label}}", vect[1]->name);
  EXPECT_STREQ("{{output}}", vect[2]->name);
  EXPECT_STREQ("{{source}}", vect[3]->name);
  EXPECT_STREQ("{{target_gen_dir}}", vect[4]->name);

  for (size_t i = 1; i < vect.size(); ++i) {
    EXPECT_LT(std::strcmp(vect[i - 1]->name, vect[i]->name), 0);
  }
}
