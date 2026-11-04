// Copyright (c) 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/desc_builder.h"

#include "gn/test_with_scope.h"
#include "util/test/test.h"

TEST(DescBuilder, TargetWithValidations) {
  TestWithScope setup;
  Err err;

  Target validation_target(setup.settings(), Label(SourceDir("//foo/"), "val"));
  validation_target.set_output_type(Target::ACTION);
  validation_target.visibility().SetPublic();
  validation_target.SetToolchain(setup.toolchain());
  validation_target.action_values().set_script(SourceFile("//foo/script.py"));
  validation_target.action_values().outputs() =
      SubstitutionList::MakeForTest("//out/Debug/val.out");
  ASSERT_TRUE(validation_target.OnResolved(&err));

  Target target(setup.settings(), Label(SourceDir("//foo/"), "target"));
  target.set_output_type(Target::GROUP);
  target.visibility().SetPublic();
  target.SetToolchain(setup.toolchain());
  target.validations().push_back(LabelTargetPair(&validation_target));
  ASSERT_TRUE(target.OnResolved(&err));

  std::unique_ptr<base::DictionaryValue> desc =
      DescBuilder::DescriptionForTarget(&target, "", false, false, false);

  base::Value* validations = desc->FindKey("validations");
  ASSERT_TRUE(validations);
  ASSERT_TRUE(validations->is_list());
  ASSERT_EQ(1u, validations->GetList().size());
  EXPECT_EQ("//foo:val()", validations->GetList()[0].GetString());
}

TEST(DescBuilder, TargetWithPublicInputs) {
  TestWithScope setup;
  Err err;

  Target group_target(setup.settings(), Label(SourceDir("//foo/"), "group"));
  group_target.set_output_type(Target::GROUP);
  group_target.visibility().SetPublic();
  group_target.SetToolchain(setup.toolchain());
  group_target.public_inputs().push_back(SourceFile("//foo/bar.d.ts"));
  ASSERT_TRUE(group_target.OnResolved(&err));

  // 1. Overall description includes public_inputs.
  {
    std::unique_ptr<base::DictionaryValue> desc =
        DescBuilder::DescriptionForTarget(&group_target, "", false, false,
                                          false);
    base::Value* public_inputs = desc->FindKey("public_inputs");
    ASSERT_TRUE(public_inputs);
    ASSERT_TRUE(public_inputs->is_list());
    ASSERT_EQ(1u, public_inputs->GetList().size());
    EXPECT_EQ("//foo/bar.d.ts", public_inputs->GetList()[0].GetString());
  }

  // 2. Specific "public_inputs" query.
  {
    std::unique_ptr<base::DictionaryValue> desc =
        DescBuilder::DescriptionForTarget(&group_target, "public_inputs", false,
                                          false, false);
    EXPECT_EQ(1u, desc->size());
    base::Value* public_inputs = desc->FindKey("public_inputs");
    ASSERT_TRUE(public_inputs);
    ASSERT_TRUE(public_inputs->is_list());
    ASSERT_EQ(1u, public_inputs->GetList().size());
    EXPECT_EQ("//foo/bar.d.ts", public_inputs->GetList()[0].GetString());
  }

  // 3. Target without public_inputs does not have public_inputs in description.
  Target empty_group(setup.settings(), Label(SourceDir("//foo/"), "empty"));
  empty_group.set_output_type(Target::GROUP);
  empty_group.visibility().SetPublic();
  empty_group.SetToolchain(setup.toolchain());
  ASSERT_TRUE(empty_group.OnResolved(&err));

  {
    std::unique_ptr<base::DictionaryValue> desc =
        DescBuilder::DescriptionForTarget(&empty_group, "", false, false,
                                          false);
    EXPECT_FALSE(desc->FindKey("public_inputs"));
  }
}
