// Copyright 2026 The GN Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_EDIT_SUBCOMMANDS_H_
#define TOOLS_GN_EDIT_SUBCOMMANDS_H_

#include <functional>
#include <set>
#include <string>
#include <vector>

#include "gn/err.h"
#include "gn/label.h"

class BuildFile;

struct EditState {
  explicit EditState(std::string context) : context(std::move(context)) {}

  EditState() = default;
  EditState(std::set<Label> review,
            std::vector<Err> warn = {},
            std::set<Label> needs_fix_deps = {})
      : needs_manual_review(std::move(review)),
        warnings(std::move(warn)),
        needs_fix_deps(std::move(needs_fix_deps)) {}

  // When something needs manual review, gn will output
  // "# TODO(gn edit: <context>)"
  std::string context;

  // Each label in this list needs manual review. # TODO(gn edit) comments
  // will have been added to the build file to give the user more precise
  // instructions.
  std::set<Label> needs_manual_review;
  // Extra information the user should be wary of. For example, if the user
  // runs: `gn edit "remove deps //bar" //foo`, but //bar was not a dependency
  // of //foo.
  std::vector<Err> warnings;
  // Targets that have had dependencies stripped and need dependency resolution
  // via `gn check <out_dir> --fix`.
  std::set<Label> needs_fix_deps;
};

// EditCommand is a function that modifies a build file.
using EditCommand = std::function<Err(BuildFile& build_file, EditState& state)>;

// Parses a command such as ["set", "testonly", "true"] into an EditCommand.
Result<EditCommand> ParseCommand(std::vector<std::string> args);

#endif  // TOOLS_GN_EDIT_SUBCOMMANDS_H_
