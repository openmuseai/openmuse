// Copyright 2026 The GN Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/edit_command.h"

#include <string>
#include <string_view>
#include <vector>

#include "base/files/file_util.h"
#include "base/files/scoped_temp_dir.h"
#include "gn/err.h"
#include "gn/filesystem_utils.h"
#include "gn/setup.h"
#include "gn/test_with_scheduler.h"
#include "util/test/test.h"

namespace commands {
namespace {
struct Edited;
std::string Pretty(const Edited& edited);

struct Edited {
  Edited(std::string_view contents, EditState edit_state = EditState())
      : contents_(contents.starts_with('\n') ? contents.substr(1) : contents),
        edit_state_(std::move(edit_state)) {}

  bool operator==(const Edited& other) const {
    return Pretty(*this) == Pretty(other);
  }

  std::string contents_;
  EditState edit_state_;
};

std::string Pretty(const Edited& edited) {
  std::string res = edited.contents_;
  if (!edited.edit_state_.needs_manual_review.empty()) {
    res += "\nNeeds manual review: " +
           testing::Pretty(edited.edit_state_.needs_manual_review);
  }
  if (!edited.edit_state_.warnings.empty()) {
    res += "\nWarnings: " + testing::Pretty(edited.edit_state_.warnings);
  }
  if (!edited.edit_state_.needs_fix_deps.empty()) {
    res += "\nNeeds check --fix: " +
           testing::Pretty(edited.edit_state_.needs_fix_deps);
  }
  return res;
}

// Runs an edit command on matching the given target
// patterns, and returns the formatted output file contents.
Result<Edited> DoEdit(std::string command,
                      std::vector<std::string> patterns,
                      const std::string& before) {
  base::ScopedTempDir temp_dir;
  if (!temp_dir.CreateUniqueTempDir()) {
    return Err(Location(), "Failed to create temp dir");
  }
  base::FilePath root_path = base::MakeAbsoluteFilePath(temp_dir.GetPath());

  base::FilePath build_gn_path = root_path.AppendASCII("BUILD.gn");
  if (!WriteFile(build_gn_path, before, nullptr)) {
    return Err(Location(), "Failed to write BUILD.gn");
  }
  base::FilePath dot_gn_path = root_path.AppendASCII(".gn");
  if (!WriteFile(dot_gn_path, "", nullptr)) {
    return Err(Location(), "Failed to write .gn");
  }

  Setup setup;
  setup.build_settings().SetRootPath(root_path);

  std::vector<std::string> args;
  args.push_back(std::move(command));
  for (auto& p : patterns) {
    args.push_back(std::move(p));
  }

  auto result = RunEditImpl(args, setup);
  if (result.has_error()) {
    return result.error();
  }

  std::string after;
  if (!base::ReadFileToString(build_gn_path, &after)) {
    return Err(Location(), "Failed to read BUILD.gn");
  }
  return Edited(after, std::move(result->second));
}

// Runs an edit command matching all targets in the root BUILD.gn ("//:*").
Result<Edited> DoEdit(std::string command, const std::string& before) {
  return DoEdit(std::move(command), {"//:*"}, before);
}

}  // namespace

using EditCommandTest = TestWithScheduler;

TEST_F(EditCommandTest, MultipleTargetsSubset) {
  EXPECT_SUCCESS(DoEdit("set testonly true", {"//:foo"},
                        R"(
executable("foo") {
  testonly = false
}
executable("bar") {
  testonly = false
}
)"),
                 Edited(R"(
executable("foo") {
  testonly = true
}

executable("bar") {
  testonly = false
}
)"));
}

TEST_F(EditCommandTest, PatternNeverMatched) {
  EXPECT_FAILURE(DoEdit("set testonly true", {"//:nonexistent"},
                        R"(
executable("foo") {
}
)"),
                 "Target(s) not found: //:nonexistent");
}

TEST_F(EditCommandTest, AddSubcommand) {
  EXPECT_SUCCESS(DoEdit("add deps //add1 //add2 //add3 :dep2",
                        R"(
executable("foo") {
  deps = [ "//dep1" ]
  deps += [ "//:dep2" ]
  if (is_linux) {
    deps += [ "//add3" ]
  }
}
)"),
                 Edited(R"(
executable("foo") {
  deps = [
    "//add1",
    "//add2",
    "//add3",
    "//dep1",
  ]
  deps += [ "//:dep2" ]
  if (is_linux) {
    deps += []
  }
}
)"));

  // Adding to a target where attribute is not defined should it at the end
  EXPECT_SUCCESS(DoEdit("add deps //base",
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
)"),
                 Edited(R"(
executable("foo") {
  sources = [ "foo.cc" ]
  deps = [ "//base" ]
}
)"));

  // Attribute defined only conditionally should hoist to start of block and
  // convert = to +=
  EXPECT_SUCCESS(DoEdit("add deps //base",
                        R"(
executable("foo") {
  if (is_linux) {
    deps = [ "//dep" ]
  }
}
)"),
                 Edited(R"(
executable("foo") {
  deps = [ "//base" ]
  if (is_linux) {
    deps += [ "//dep" ]
  }
}
)"));

  EXPECT_SUCCESS(DoEdit("add deps //base",
                        R"(
executable("foo") {
  deps = foo + bar + [ "//baz" ]
}
)"),
                 Edited(R"(
executable("foo") {
  deps = foo + bar + [
           "//base",
           "//baz",
         ]
}
)"));

  EXPECT_SUCCESS(DoEdit("add deps //base",
                        R"(
executable("foo") {
  deps = other_deps
}
)"),
                 Edited(R"(
executable("foo") {
  deps = [ "//base" ] + other_deps
}
)"));

  EXPECT_SUCCESS(DoEdit("add public_deps //base",
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
  deps = [ "//dep" ]
}
)"),
                 Edited(R"(
executable("foo") {
  sources = [ "foo.cc" ]
  public_deps = [ "//base" ]
  deps = [ "//dep" ]
}
)"));

  EXPECT_SUCCESS(DoEdit("add deps //base",
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
  if (is_linux) {
    deps = [ "//dep" ]
  }
}
)"),
                 Edited(R"(
executable("foo") {
  sources = [ "foo.cc" ]
  deps = [ "//base" ]
  if (is_linux) {
    deps += [ "//dep" ]
  }
}
)"));
}

TEST_F(EditCommandTest, DeleteSubcommand) {
  EXPECT_SUCCESS(DoEdit("delete", {"//:bar"},
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
executable("bar") {
  sources = [ "bar.cc" ]
}
)"),
                 Edited(R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
)"));

  EXPECT_SUCCESS(DoEdit("delete", {"//:bar"},
                        R"(
if (is_win) {
  executable("bar") {
    sources = [ "bar.cc" ]
  }
} else {
  executable("bar") {
    sources = [ "bar.cc" ]
  }
}
)"),
                 Edited(R"(
if (is_win) {
  # TODO(gn edit: delete): This would normally be deleted but is conditional.
  # Manual intervention is required to decide whether it should actually be
  # deleted.
  executable("bar") {
    sources = [ "bar.cc" ]
  }
} else {
  # TODO(gn edit: delete): This would normally be deleted but is conditional.
  # Manual intervention is required to decide whether it should actually be
  # deleted.
  executable("bar") {
    sources = [ "bar.cc" ]
  }
}
)",
                        EditState({Label(SourceDir("//"), "bar")})));
}

TEST_F(EditCommandTest, MoveSubcommand) {
  EXPECT_SUCCESS(
      DoEdit("move deps public_deps //a //b //nonexistent", {"//:foo"},
             R"(
executable("foo") {
  deps = [
    "//a",
    "//b",
    "//c",
  ]
  public_deps = [ "//d" ]
}
)"),
      Edited(R"(
executable("foo") {
  deps = [ "//c" ]
  public_deps = [
    "//a",
    "//b",
    "//d",
  ]
}
)",
             EditState{{},
                       {Err(Location(),
                            "Target \"//:foo\" does not contain the value "
                            "\"//nonexistent\" in attribute \"deps\".")}}));

  EXPECT_SUCCESS(DoEdit("move deps public_deps //a",
                        R"(
executable("foo") {
  deps = [ "//a" ]
}
)"),
                 Edited(R"(
executable("foo") {
  public_deps = [ "//a" ]
}
)"));
}

TEST_F(EditCommandTest, NewSubcommand) {
  // Append new target to the end of the file.
  EXPECT_SUCCESS(DoEdit("new source_set", {"//:bar"},
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
)"),
                 Edited(R"(
executable("foo") {
  sources = [ "foo.cc" ]
}

source_set("bar") {
}
)"));

  // Insert before a relative target.
  EXPECT_SUCCESS(DoEdit("new source_set before foo", {"//:bar"},
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
)"),
                 Edited(R"(
source_set("bar") {
}

executable("foo") {
  sources = [ "foo.cc" ]
}
)"));

  // Insert after a relative target.
  EXPECT_SUCCESS(DoEdit("new source_set after foo", {"//:bar"},
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
}

executable("baz") {
  sources = [ "baz.cc" ]
}
)"),
                 Edited(R"(
executable("foo") {
  sources = [ "foo.cc" ]
}

source_set("bar") {
}

executable("baz") {
  sources = [ "baz.cc" ]
}
)"));

  // Insert multiple new targets after a relative target.
  EXPECT_SUCCESS(DoEdit("new source_set after foo", {"//:bar", "//:qux"},
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
}

executable("baz") {
  sources = [ "baz.cc" ]
}
)"),
                 Edited(R"(
executable("foo") {
  sources = [ "foo.cc" ]
}

source_set("bar") {
}

source_set("qux") {
}

executable("baz") {
  sources = [ "baz.cc" ]
}
)"));

  // Error when target already exists.
  EXPECT_FAILURE(DoEdit("new source_set", {"//:foo"},
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
)"));

  // Error when relative target not found.
  EXPECT_FAILURE(DoEdit("new source_set before nonexistent", {"//:bar"},
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
)"));

  // Error when no explicit target name specified (e.g. glob pattern).
  EXPECT_FAILURE(DoEdit("new source_set", {"//*"},
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
)"));
}

TEST_F(EditCommandTest, RemoveAttributeSubcommand) {
  EXPECT_SUCCESS(DoEdit("remove testonly",
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
  testonly = true
}
)"),
                 Edited(R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
)"));

  EXPECT_SUCCESS(
      DoEdit("remove nonexistent_attribute", {"//:foo"},
             R"(
executable("foo") {
}
)"),
      Edited(R"(
executable("foo") {
}
)",
             EditState{{},
                       {Err(Location(),
                            "Target \"//:foo\" does not contain the "
                            "attribute \"nonexistent_attribute\".")}}));
}

TEST_F(EditCommandTest, RemoveFromAttributeSubcommand) {
  EXPECT_SUCCESS(DoEdit("remove deps //base :bar",
                        R"(
executable("foo") {
  deps = [
    "//base",
    "//:bar",
    "//other:bar",
  ]
}
)"),
                 Edited(R"(
executable("foo") {
  deps = [ "//other:bar" ]
}
)"));

  EXPECT_SUCCESS(DoEdit("remove deps //base //nonexistent:glob",
                        R"(
executable("foo") {
  deps = [ "//base" ] + [ "//foo:bar" ]
}
)"),
                 Edited(R"(
executable("foo") {
  deps = [ "//foo:bar" ]
}
)"));

  EXPECT_SUCCESS(DoEdit("remove deps //nonexistent", {"//:foo"},
                        R"(
executable("foo") {
  deps = [ "//base" ]
}
)"),
                 Edited(R"(
executable("foo") {
  deps = [ "//base" ]
}
)",
                        EditState{{},
                                  {Err(Location(),
                                       "Target \"//:foo\" does not contain the "
                                       "value \"//nonexistent\" in attribute "
                                       "\"deps\".")}}));

  EXPECT_SUCCESS(DoEdit("remove deps //base",
                        R"(
executable("foo") {
  deps = [ "//base" ]
}
)"),
                 Edited(R"(
executable("foo") {
}
)"));

  EXPECT_SUCCESS(DoEdit("remove deps //base",
                        R"(
executable("foo") {
  deps = [ "//base" ]
  if (is_linux) {
    deps += [ "//linux" ]
  }
}
)"),
                 Edited(R"(
executable("foo") {
  if (is_linux) {
    deps = [ "//linux" ]
  }
}
)"));

  EXPECT_SUCCESS(DoEdit("remove deps //a",
                        R"(
executable("foo") {
  deps = [ "//a" ]
  deps += [ "//b" ]
}
)"),
                 Edited(R"(
executable("foo") {
  deps = [ "//b" ]
}
)"));

  EXPECT_SUCCESS(DoEdit("remove deps //base",
                        R"(
executable("foo") {
  deps = [ "//base" ]
  if (is_linux) {
    deps += [ "//linux" ]
  }
  deps += [ "//other" ]
}
)"),
                 Edited(R"(
executable("foo") {
  deps = []
  if (is_linux) {
    deps += [ "//linux" ]
  }
  deps += [ "//other" ]
}
)"));

  // When multiple assignments exist, an empty assignment must not be removed,
  // and subsequent conditional += must not be converted to =.
  EXPECT_SUCCESS(DoEdit("remove deps //base",
                        R"(
executable("foo") {
  deps = [ "//base" ]
  if (is_linux) {
    deps += [ "//linux" ]
  }
  if (is_mac) {
    deps += [ "//mac" ]
  }
}
)"),
                 Edited(R"(
executable("foo") {
  deps = []
  if (is_linux) {
    deps += [ "//linux" ]
  }
  if (is_mac) {
    deps += [ "//mac" ]
  }
}
)"));
}

TEST_F(EditCommandTest, RenameSubcommand) {
  EXPECT_SUCCESS(DoEdit("rename srcs sources",
                        R"(
executable("foo") {
  # This comment should be preserved
  srcs = [ "foo.cc" ]
  if (is_linux) {
    srcs += [ "linux.cc" ]
  }
}
)"),
                 Edited(R"(
executable("foo") {
  # This comment should be preserved
  sources = [ "foo.cc" ]
  if (is_linux) {
    sources += [ "linux.cc" ]
  }
}
)"));

  EXPECT_SUCCESS(DoEdit("rename nonexistent new_attr", {"//:foo"},
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
)"),
                 Edited(R"(
executable("foo") {
  sources = [ "foo.cc" ]
}
)",
                        EditState{{},
                                  {Err(Location(),
                                       "Target \"//:foo\" does not contain the "
                                       "attribute \"nonexistent\".")}}));
}

TEST_F(EditCommandTest, SetSubcommand) {
  // New bool attribute
  EXPECT_SUCCESS(DoEdit("set testonly true",
                        R"(
executable("foo") {
}
)"),
                 Edited(R"(
executable("foo") {
  testonly = true
}
)"));

  EXPECT_SUCCESS(DoEdit("set testonly true",
                        R"(
executable("foo") {
  sources = [ "foo.cc" ]
  deps = [ "//dep" ]
}
)"),
                 Edited(R"(
executable("foo") {
  testonly = true
  sources = [ "foo.cc" ]
  deps = [ "//dep" ]
}
)"));

  // Replacing existing attribute
  EXPECT_SUCCESS(DoEdit("set testonly false",
                        R"(
executable("foo") {
  testonly = true
}
)"),
                 Edited(R"(
executable("foo") {
  testonly = false
}
)"));

  // String attribute
  EXPECT_SUCCESS(DoEdit("set label \"//foo:bar\"",
                        R"(
executable("foo") {
}
)"),
                 Edited(R"(
executable("foo") {
  label = "//foo:bar"
}
)"));

  // Int attribute
  EXPECT_SUCCESS(DoEdit("set assert_no_deps 42",
                        R"(
executable("foo") {
}
)"),
                 Edited(R"(
executable("foo") {
  assert_no_deps = 42
}
)"));

  // Multiple values setting a list (replaces first, deletes modification,
  // adds review for conditional)
  EXPECT_SUCCESS(DoEdit("set deps //foo //bar",
                        R"(
executable("foo") {
  deps = [ "//bar" ]
  deps += [ "//baz" ]
  if (is_linux) {
    deps += [ "//linux" ]
  }
}
)"),
                 Edited(
                     R"(
executable("foo") {
  deps = [
    "//bar",
    "//foo",
  ]

  if (is_linux) {
    # TODO(gn edit: set deps //foo //bar): This would normally be deleted but is
    # conditional. Manual intervention is required to decide whether it should
    # actually be deleted.
    deps += [ "//linux" ]
  }
}
)",
                     EditState({Label(SourceDir("//"), "foo")})));

  // Forced list attribute (appends new list, adds review for conditional)
  EXPECT_SUCCESS(DoEdit("set deps:list //foo",
                        R"(
executable("foo") {
  if (is_linux) {
    deps = [ "//linux" ]
  }
}
)"),
                 Edited(
                     R"(
executable("foo") {
  deps = [ "//foo" ]
  if (is_linux) {
    # TODO(gn edit: set deps:list //foo): This would normally be deleted but is
    # conditional. Manual intervention is required to decide whether it should
    # actually be deleted.
    deps = [ "//linux" ]
  }
}
)",
                     EditState({Label(SourceDir("//"), "foo")})));

  // Custom Expressions
  EXPECT_SUCCESS(DoEdit("set str:expr default + \"a b\"",
                        R"(
executable("foo") {
}
)"),
                 Edited(R"(
executable("foo") {
  str = default + "a b"
}
)"));

  EXPECT_FAILURE(DoEdit("set deps:unknown a b",
                        R"(
executable("foo") {
}
)"),
                 "Unknown type: :unknown");
}

TEST_F(EditCommandTest, ShardSubcommand) {
  // Shards target across sources and subdirectory files, preserving
  // conditionals, attributes, and visibility.
  EXPECT_SUCCESS(DoEdit("shard", {"//:foo"},
                        R"(
static_library("foo") {
  sources = [
    "a.cc",
    "a.h",
    "foo.h",
    "util/foo-bar.cc",
    "util/foo-bar.h",
  ]
  if (is_win) {
    sources += [ "c_win.cc" ]
  }
  defines = [ "ENABLE_FEATURE" ]
  deps = [ "//base" ]
  testonly = true
  visibility = [ "//..." ]
}
)"),
                 Edited(R"(
group("foo") {
  public_deps = [
    ":a",
    ":c_win",
    ":foo_foo",
    ":util_foo_bar",
  ]

  testonly = true
  visibility = [ "//..." ]
}

static_library("a") {
  sources = [
    "a.cc",
    "a.h",
  ]
  if (is_win) {
    sources += []
  }
  defines = [ "ENABLE_FEATURE" ]

  testonly = true
  visibility = [ "//..." ]
}

static_library("c_win") {
  sources = []
  if (is_win) {
    sources += [ "c_win.cc" ]
  }
  defines = [ "ENABLE_FEATURE" ]

  testonly = true
  visibility = [ "//..." ]
}

static_library("foo_foo") {
  sources = [ "foo.h" ]
  if (is_win) {
    sources += []
  }
  defines = [ "ENABLE_FEATURE" ]

  testonly = true
  visibility = [ "//..." ]
}

static_library("util_foo_bar") {
  sources = [
    "util/foo-bar.cc",
    "util/foo-bar.h",
  ]
  if (is_win) {
    sources += []
  }
  defines = [ "ENABLE_FEATURE" ]

  testonly = true
  visibility = [ "//..." ]
}
)",
                        EditState({}, {},
                                  {Label(SourceDir("//"), "a"),
                                   Label(SourceDir("//"), "c_win"),
                                   Label(SourceDir("//"), "foo_foo"),
                                   Label(SourceDir("//"), "util_foo_bar")})));

  // Sharding with explicit shard target type and custom group type.
  EXPECT_SUCCESS(DoEdit("shard source_set static_library", {"//:foo"},
                        R"(
static_library("foo") {
  sources = [
    "a.cc",
    "b.cc",
  ]
}
)"),
                 Edited(R"(
static_library("foo") {
  public_deps = [
    ":a",
    ":b",
  ]
}

source_set("a") {
  sources = [ "a.cc" ]
}

source_set("b") {
  sources = [ "b.cc" ]
}
)",
                        EditState({}, {},
                                  {Label(SourceDir("//"), "a"),
                                   Label(SourceDir("//"), "b")})));

  // Single source target emits a warning and is not sharded.
  EXPECT_SUCCESS(
      DoEdit("shard", {"//:foo"},
             R"(
static_library("foo") {
  sources = [ "foo.cc" ]
  deps = [ "//base" ]
}
)"),
      Edited(R"(
static_library("foo") {
  sources = [ "foo.cc" ]
  deps = [ "//base" ]
}
)",
             EditState(
                 {}, {Err(Location(),
                          "Target \"//:foo\" does not need to be sharded.")})));

  // Absolute source paths produce an error.
  EXPECT_FAILURE(DoEdit("shard", {"//:foo"},
                        R"(
static_library("foo") {
  sources = [
    "//base/a.cc",
    "b.cc",
  ]
}
)"));
}

}  // namespace commands
