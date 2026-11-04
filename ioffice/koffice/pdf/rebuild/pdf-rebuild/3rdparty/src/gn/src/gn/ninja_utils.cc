// Copyright 2014 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/ninja_utils.h"

#include "gn/build_settings.h"
#include "gn/filesystem_utils.h"
#include "gn/output_file.h"
#include "gn/settings.h"
#include "gn/target.h"

SourceFile GetNinjaFileForTarget(const Target* target) {
  return SourceFile(GetSourceDir(*target, BuildDirType::OBJ).value() +
                    target->label().name() + ".ninja");
}

SourceFile GetNinjaFileForToolchain(const Settings* settings) {
  return SourceFile(
      GetSourceDir(BuildDirContext(settings), BuildDirType::TOOLCHAIN_ROOT)
          .value() +
      "toolchain.ninja");
}

std::string GetNinjaRulePrefixForToolchain(const Settings* settings) {
  // Don't prefix the default toolchain so it looks prettier, prefix everything
  // else.
  if (settings->is_default())
    return std::string();  // Default toolchain has no prefix.
  return settings->toolchain_label().name() + "_";
}

OutputFile GetPublicInputsOutputFile(const Target* target,
                                     const BuildSettings* build_settings) {
  if (build_settings->no_stamp_files()) {
    return GetOutputFile(*target, BuildDirType::PHONY, target->label().name(),
                         ".public_inputs");
  } else {
    return GetOutputFile(*target, BuildDirType::OBJ, target->label().name(),
                         ".public_inputs.stamp");
  }
}
