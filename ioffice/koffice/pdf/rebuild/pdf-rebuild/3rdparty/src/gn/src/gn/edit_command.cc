// Copyright 2026 The GN Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/edit_command.h"

#include <iomanip>
#include <sstream>
#include <utility>
#include <vector>

#include "gn/build_file_editor.h"
#include "gn/commands.h"
#include "gn/edit_subcommands.h"
#include "gn/filesystem_utils.h"
#include "gn/setup.h"
#include "gn/source_file.h"
#include "gn/standard_out.h"
#include "gn/value.h"

namespace commands {

const char kEdit[] = "edit";
const char kEdit_HelpShort[] =
    "edit: Edit BUILD.gn files from the command line.";
const char kEdit_Help[] =
    "gn edit <command> <labels/patterns...>\n"
    "\n"
    "  Executes a command to modify a set of targets\n"
    "\n"
    "  Note: Because GN is an imperative language, it's not always entirely\n"
    "  clear what the \"correct\" thing is to do.\n"
    "\n"
    "  In cases of ambiguity (eg. conditionals), `gn edit` will leave notes\n"
    "  in your build files instructing you what to do.\n"
    "\n"
    "Commands:\n"
    "  add <attribute> <value(s)>\n"
    "      Adds <value(s)> to the list attribute <attribute>.\n"
    "      If the attribute does not exist, it is created.\n"
    "\n"
    "      Example:\n"
    "        gn edit \"add deps //base //src/tools:utils\" //src/tools:*\n"
    "\n"
    "  delete\n"
    "      Deletes the matched targets entirely.\n"
    "\n"
    "      Example:\n"
    "        gn edit \"delete\" //src/tools:old_target\n"
    "\n"
    "  move <from_attribute> <to_attribute> <value(s)>\n"
    "      Moves <value(s)> from the list <from_attribute> to <to_attribute>.\n"
    "      If <to_attribute> does not exist, it is created.\n"
    "\n"
    "      Example:\n"
    "        gn edit \"move deps public_deps //base\" //src/tools:*\n"
    "\n"
    "  new <rule_kind> [(before|after) <relative_rule_name>]\n"
    "      Adds a new rule at the end of the BUILD file (or before/after\n"
    "      <relative_rule_name>). The rule name is determined by the target "
    "label.\n"
    "\n"
    "      Examples:\n"
    "        gn edit \"new source_set\" //src/tools:my_target\n"
    "        gn edit \"new static_library before old_target\" "
    "//src/tools:helper\n"
    "\n"
    "  remove <attribute>\n"
    "      Removes <attribute> entirely.\n"
    "\n"
    "      Example:\n"
    "        gn edit \"remove testonly\" //src/tools:*\n"
    "\n"
    "  remove <attribute> <value(s)>\n"
    "      Removes <value(s)> from the list attribute <attribute>.\n"
    "\n"
    "      Example:\n"
    "        gn edit \"remove deps //base\" //src/tools:*\n"
    "\n"
    "  rename <from_attribute> <to_attribute>\n"
    "      Renames <from_attribute> to <to_attribute>.\n"
    "\n"
    "      Example:\n"
    "        gn edit \"rename srcs sources\" //src/tools:*\n"
    "\n"
    "  set <attribute>[:list|:expr] <value(s)>\n"
    "      Sets or overwrites the target's <attribute> to <value(s)>.\n"
    "      If multiple values are provided, or if the \":list\" suffix is\n"
    "      appended to the attribute, <value(s)> is interpreted as a list.\n"
    "      If the \":expr\" suffix is appended to the attribute, <value(s)>\n"
    "      is parsed as a raw GN expression (e.g. variable, list of "
    "variables,\n"
    "      or expression).\n"
    "\n"
    "      Examples:\n"
    "        gn edit \"set testonly true\" //src/tools:*\n"
    "          => testonly = true\n"
    "        gn edit \"set output_name my_tool\" //src/tools:*\n"
    "          => output_name = \"my_tool\"\n"
    "        gn edit \"set deps:expr default_deps\" //:foo\n"
    "          => deps = default_deps\n"
    "        gn edit \"set srcs:list foo.cc\" //:foo\n"
    "          => srcs = [ \"foo.cc\" ]\n"
    "        gn edit \"set deps :bar :baz\" //:foo\n"
    "          => deps = [ \":bar\", \":baz\" ]\n"
    "        gn edit \"set deps:expr a + b\" //:foo\n"
    "          => deps = a + b\n"
    "\n"
    "  shard [sharded_target_type] [group_type]\n"
    "      Splits the target's sources into fine-grained shard targets,\n"
    "      inheriting compilation flags, and updates the parent\n"
    "      target to depend on the newly created shards.\n"
    "      Note: Sharding strips existing deps and clones conditional blocks\n"
    "      into each shard. For conditional sources (e.g. `if (is_win)`),\n"
    "      manual cleanup is often needed, such as:\n"
    "        * Moving the condition to wrap the shard instead of the sources\n"
    "        * Stripping empty conditions\n"
    "\n"
    "      Examples:\n"
    "        gn edit \"shard\" //:large_target\n"
    "        gn edit \"shard source_set static_library\" //:large_target\n";

Result<std::pair<std::vector<SourceFile>, EditState>> RunEditImpl(
    const std::vector<std::string>& args,
    Setup& setup) {
  if (args.size() < 2) {
    return Err(Location(), "Insufficient arguments.",
               "Usage: gn edit <command> <labels...>\n"
               "Example: gn edit \"set testonly true\" //foo:*");
  }

  // We use std::quoted to tokenize the command.
  // eg. set foo "bar baz" -> ["set", "foo", "bar baz"].
  std::stringstream ss(args[0]);
  std::vector<std::string> command_tokens;
  std::string token;
  while (ss >> std::ws && !ss.eof()) {
    if (!(ss >> std::quoted(token))) {
      return Err(Location(), "Unclosed quote in command string.");
    }
    command_tokens.push_back(std::move(token));

    // When calling set on an expression, we don't want to use std::quoted,
    // we want to load the string raw.
    if (command_tokens.size() == 2 && command_tokens[0] == "set" &&
        command_tokens[1].ends_with(":expr")) {
      if (!(ss >> std::ws).eof()) {
        command_tokens.emplace_back(args[0].substr(ss.tellg()));
      }
      break;
    }
  }
  if (command_tokens.empty()) {
    return Err(Location(), "Empty command string.");
  }

  ASSIGN_OR_RETURN(EditCommand command,
                   ParseCommand(std::move(command_tokens)));
  const SourceDir current_dir =
      SourceDirForCurrentDirectory(setup.build_settings().root_path());
  const std::string source_root = setup.build_settings().root_path_utf8();

  std::vector<LabelPattern> patterns;
  for (size_t i = 1; i < args.size(); ++i) {
    Value val(nullptr, args[i]);
    Err err;
    LabelPattern pattern =
        LabelPattern::GetPattern(current_dir, source_root, val, &err);
    if (err.has_error()) {
      return err;
    }
    patterns.push_back(std::move(pattern));
  }

  ASSIGN_OR_RETURN(std::vector<BuildFile> build_files,
                   ::ResolvePatternsToBuildFiles(&setup.build_settings(),
                                                 setup.loader(), patterns));

  EditState state(args[0]);
  for (auto& build_file : build_files) {
    RETURN_IF_ERROR(command(build_file, state));
    RETURN_IF_ERROR(build_file.label_matcher().done());
  }

  std::vector<SourceFile> modified_files;
  for (auto& build_file : build_files) {
    ASSIGN_OR_RETURN(bool wrote, build_file.Write());
    if (wrote) {
      modified_files.push_back(build_file.source_file());
    }
  }

  return std::make_pair(std::move(modified_files), std::move(state));
}

int RunEdit(const std::vector<std::string>& args) {
  Setup setup;
  if (!setup.DoSetupForEditing()) {
    return 1;
  }
  auto result = RunEditImpl(args, setup);
  if (result.has_error()) {
    result.error().PrintToStdout();
    return 1;
  }
  for (const auto& file : result->first) {
    OutputString("Wrote '" + file.value() + "'.\n");
  }
  for (const auto& warning : result->second.warnings) {
    warning.PrintNonfatalToStdout();
  }
  const auto& review_needed = result->second.needs_manual_review;
  if (!review_needed.empty()) {
    OutputString("\nThe following targets need manual review:\n",
                 DECORATION_YELLOW);
    for (const Label& label : review_needed) {
      OutputString("* ");
      OutputString(label.GetUserVisibleName(false) + "\n", DECORATION_GREEN);
    }

    OutputString(
        "\nWhere manual review is required, comments have been added to the "
        "build file of the form:\n'# TODO(gn edit: <command>): ...'\n",
        DECORATION_DIM);
  }
  const auto& needs_fix_deps = result->second.needs_fix_deps;
  if (!needs_fix_deps.empty()) {
    OutputString(
        "\nTo automatically resolve and populate dependencies for sharded "
        "targets, run:\n",
        DECORATION_YELLOW);
    std::string check_cmd = "gn check <out_dir>";
    for (const Label& label : needs_fix_deps) {
      check_cmd += " " + label.GetUserVisibleName(false);
    }
    check_cmd += " --fix\n";
    OutputString(check_cmd, DECORATION_GREEN);
  }
  return 0;
}

}  // namespace commands
