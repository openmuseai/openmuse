// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/ffi/session.h"

#include "gn/ffi/bridge.h"
#include "gn/scope.h"
#include "util/test/test.h"

TEST(SessionTest, SessionLoad) {
  TestWithScope setup;
  rust::Box<Session> session = Session::new_for_testing();

  std::vector<Value> keys = {Value(nullptr, "absolute_value"),
                             Value(nullptr, "relative_value")};

  setup.scope()->set_source_dir(SourceDir("//load/"));
  Err err;
  bool success =
      session_load(*session, Value(nullptr, ":root.scl"), keys, *setup.scope(),
                   ParseNodePtr{.ptr = nullptr}, err);
  EXPECT_TRUE(success);
  EXPECT_FALSE(err.has_error());

  const Value* absolute_val = setup.scope()->GetValue("absolute_value");
  ASSERT_TRUE(absolute_val);
  EXPECT_EQ(absolute_val->type(), Value::STRING);
  EXPECT_EQ(absolute_val->string_value(), "absolute");

  const Value* relative_val = setup.scope()->GetValue("relative_value");
  ASSERT_TRUE(relative_val);
  EXPECT_EQ(relative_val->type(), Value::STRING);
  EXPECT_EQ(relative_val->string_value(), "relative");
}
