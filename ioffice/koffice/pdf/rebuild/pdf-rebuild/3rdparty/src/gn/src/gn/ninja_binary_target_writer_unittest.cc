// Copyright (c) 2013 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/ninja_binary_target_writer.h"

#include "gn/test_with_scheduler.h"
#include "gn/test_with_scope.h"
#include "util/test/test.h"

using NinjaBinaryTargetWriterTest = TestWithScheduler;

TEST_F(NinjaBinaryTargetWriterTest, CSources) {
  Err err;
  TestWithScope setup;

  Target target(setup.settings(), Label(SourceDir("//foo/"), "bar"));
  target.set_output_type(Target::SOURCE_SET);
  target.visibility().SetPublic();
  target.sources().push_back(SourceFile("//foo/input1.cc"));
  target.sources().push_back(SourceFile("//foo/input2.cc"));
  // Also test object files, which should be just passed through to the
  // dependents to link.
  target.sources().push_back(SourceFile("//foo/input3.o"));
  target.sources().push_back(SourceFile("//foo/input4.obj"));
  target.source_types_used().Set(SourceFile::SOURCE_CPP);
  target.source_types_used().Set(SourceFile::SOURCE_O);
  target.SetToolchain(setup.toolchain());
  ASSERT_TRUE(target.OnResolved(&err));

  std::ostringstream out;
  NinjaBinaryTargetWriter writer(&target, out);
  writer.Run();

  const char expected[] =
      "defines =\n"
      "include_dirs =\n"
      "cflags =\n"
      "cflags_cc =\n"
      "root_out_dir = .\n"
      "target_gen_dir = gen/foo\n"
      "target_out_dir = obj/foo\n"
      "target_output_name = bar\n"
      "\n"
      "build obj/foo/bar.input1.o: cxx ../../foo/input1.cc\n"
      "  source_file_part = input1.cc\n"
      "  source_name_part = input1\n"
      "build obj/foo/bar.input2.o: cxx ../../foo/input2.cc\n"
      "  source_file_part = input2.cc\n"
      "  source_name_part = input2\n"
      "\n"
      "build phony/foo/bar.linkdeps: phony obj/foo/bar.input1.o "
      "obj/foo/bar.input2.o ../../foo/input3.o ../../foo/input4.obj\n"
      "build phony/foo/bar: phony phony/foo/bar.linkdeps\n";
  std::string out_str = out.str();
  EXPECT_EQ(expected, out_str);
}

TEST_F(NinjaBinaryTargetWriterTest, NoSourcesSourceSet) {
  Err err;
  TestWithScope setup;

  Target target(setup.settings(), Label(SourceDir("//foo/"), "bar"));
  target.set_output_type(Target::SOURCE_SET);
  target.visibility().SetPublic();
  target.SetToolchain(setup.toolchain());
  ASSERT_TRUE(target.OnResolved(&err));

  std::ostringstream out;
  NinjaBinaryTargetWriter writer(&target, out);
  writer.Run();

  const char expected[] =
      "defines =\n"
      "include_dirs =\n"
      "root_out_dir = .\n"
      "target_gen_dir = gen/foo\n"
      "target_out_dir = obj/foo\n"
      "target_output_name = bar\n"
      "\n"
      "\n";
  std::string out_str = out.str();
  EXPECT_EQ(expected, out_str);
}

TEST_F(NinjaBinaryTargetWriterTest, NoSourcesStaticLib) {
  Err err;
  TestWithScope setup;

  Target target(setup.settings(), Label(SourceDir("//foo/"), "bar"));
  target.set_output_type(Target::STATIC_LIBRARY);
  target.visibility().SetPublic();
  target.SetToolchain(setup.toolchain());
  ASSERT_TRUE(target.OnResolved(&err));

  std::ostringstream out;
  NinjaBinaryTargetWriter writer(&target, out);
  writer.Run();

  const char expected[] =
      "defines =\n"
      "include_dirs =\n"
      "root_out_dir = .\n"
      "target_gen_dir = gen/foo\n"
      "target_out_dir = obj/foo\n"
      "target_output_name = libbar\n"
      "\n"
      "\n"
      "build obj/foo/libbar.a: alink\n"
      "  arflags =\n"
      "  output_extension =\n"
      "  output_dir =\n";
  std::string out_str = out.str();
  EXPECT_EQ(expected, out_str);
}

TEST_F(NinjaBinaryTargetWriterTest, Inputs) {
  Err err;
  TestWithScope setup;

  {
    Target target(setup.settings(), Label(SourceDir("//foo/"), "bar"));
    target.set_output_type(Target::SOURCE_SET);
    target.visibility().SetPublic();
    target.sources().push_back(SourceFile("//foo/source1.cc"));
    target.config_values().inputs().push_back(SourceFile("//foo/input1"));
    target.config_values().inputs().push_back(SourceFile("//foo/input2"));
    target.source_types_used().Set(SourceFile::SOURCE_CPP);
    target.SetToolchain(setup.toolchain());
    ASSERT_TRUE(target.OnResolved(&err));

    std::ostringstream out;
    NinjaBinaryTargetWriter writer(&target, out);
    writer.Run();

    const char expected[] =
        "defines =\n"
        "include_dirs =\n"
        "cflags =\n"
        "cflags_cc =\n"
        "root_out_dir = .\n"
        "target_gen_dir = gen/foo\n"
        "target_out_dir = obj/foo\n"
        "target_output_name = bar\n"
        "\n"
        "build obj/foo/bar.source1.o: cxx ../../foo/source1.cc | "
        "../../foo/input1 ../../foo/input2\n"
        "  source_file_part = source1.cc\n"
        "  source_name_part = source1\n"
        "\n"
        "build phony/foo/bar.linkdeps: phony obj/foo/bar.source1.o\n"
        "build phony/foo/bar: phony phony/foo/bar.linkdeps\n";
    std::string out_str = out.str();
    EXPECT_EQ(expected, out_str);
  }

  {
    Target target(setup.settings(), Label(SourceDir("//foo/"), "bar"));
    target.set_output_type(Target::SOURCE_SET);
    target.visibility().SetPublic();
    target.sources().push_back(SourceFile("//foo/source1.cc"));
    target.sources().push_back(SourceFile("//foo/source2.cc"));
    target.config_values().inputs().push_back(SourceFile("//foo/input1"));
    target.config_values().inputs().push_back(SourceFile("//foo/input2"));
    target.source_types_used().Set(SourceFile::SOURCE_CPP);
    target.SetToolchain(setup.toolchain());
    ASSERT_TRUE(target.OnResolved(&err));

    std::ostringstream out;
    NinjaBinaryTargetWriter writer(&target, out);
    writer.Run();

    const char expected[] =
        "defines =\n"
        "include_dirs =\n"
        "cflags =\n"
        "cflags_cc =\n"
        "root_out_dir = .\n"
        "target_gen_dir = gen/foo\n"
        "target_out_dir = obj/foo\n"
        "target_output_name = bar\n"
        "\n"
        "build phony/foo/bar.inputs: phony "
        "../../foo/input1 ../../foo/input2\n"
        "build obj/foo/bar.source1.o: cxx ../../foo/source1.cc | "
        "phony/foo/bar.inputs\n"
        "  source_file_part = source1.cc\n"
        "  source_name_part = source1\n"
        "build obj/foo/bar.source2.o: cxx ../../foo/source2.cc | "
        "phony/foo/bar.inputs\n"
        "  source_file_part = source2.cc\n"
        "  source_name_part = source2\n"
        "\n"
        "build phony/foo/bar.linkdeps: phony obj/foo/bar.source1.o "
        "obj/foo/bar.source2.o\n"
        "build phony/foo/bar: phony phony/foo/bar.linkdeps\n";
    std::string out_str = out.str();
    EXPECT_EQ(expected, out_str);
  }
}

TEST_F(NinjaBinaryTargetWriterTest, PublicInputs) {
  Err err;
  TestWithScope setup;

  // Action A has public_inputs.
  Target a(setup.settings(), Label(SourceDir("//foo/"), "a"));
  a.set_output_type(Target::ACTION);
  a.visibility().SetPublic();
  a.SetToolchain(setup.toolchain());
  a.action_values().set_script(SourceFile("//foo/script.py"));
  a.public_inputs().push_back(SourceFile("//foo/a.in"));

  // Source set B depends on A.
  Target b(setup.settings(), Label(SourceDir("//foo/"), "b"));
  b.set_output_type(Target::SOURCE_SET);
  b.visibility().SetPublic();
  b.sources().push_back(SourceFile("//foo/source1.cc"));
  b.source_types_used().Set(SourceFile::SOURCE_CPP);
  b.SetToolchain(setup.toolchain());
  b.private_deps().push_back(LabelTargetPair(&a));

  ASSERT_TRUE(a.OnResolved(&err));
  ASSERT_TRUE(b.OnResolved(&err));

  std::ostringstream out;
  NinjaBinaryTargetWriter writer(&b, out);
  writer.Run();

  // B's compile lines should depend on a.in (since it inherits it from A).
  // Because it only has one input (a.in), it is depended on directly instead
  // of via a phony target.
  const char expected[] =
      "defines =\n"
      "include_dirs =\n"
      "cflags =\n"
      "cflags_cc =\n"
      "root_out_dir = .\n"
      "target_gen_dir = gen/foo\n"
      "target_out_dir = obj/foo\n"
      "target_output_name = b\n"
      "\n"
      "build obj/foo/b.source1.o: cxx ../../foo/source1.cc | "
      "phony/foo/a.public_inputs || phony/foo/a\n"
      "  source_file_part = source1.cc\n"
      "  source_name_part = source1\n"
      "\n"
      "build phony/foo/b.linkdeps: phony obj/foo/b.source1.o || phony/foo/a\n"
      "build phony/foo/b: phony phony/foo/b.linkdeps\n";
  std::string out_str = out.str();
  EXPECT_EQ(expected, out_str);
}
