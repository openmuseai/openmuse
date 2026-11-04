// Copyright 2014 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/output_file.h"

#include "gn/filesystem_utils.h"
#include "gn/source_file.h"

OutputFile::OutputFile(const BuildSettings* build_settings,
                       const SourceFile& source_file) {
  value_ =
      StringAtom(RebasePath(source_file.value(), build_settings->build_dir(),
                            build_settings->root_path_utf8()));
}

SourceFile OutputFile::AsSourceFile(const BuildSettings* build_settings) const {
  std::string_view val = value();
  DCHECK(!val.empty());
  DCHECK(val.back() != '/');

  std::string path = build_settings->build_dir().value();
  path.append(val);
  return SourceFile(std::move(path));
}

SourceDir OutputFile::AsSourceDir(const BuildSettings* build_settings) const {
  std::string_view val = value();
  if (!val.empty()) {
    // Empty means the root build dir. Otherwise, we expect it to end in a
    // slash.
    DCHECK(val.back() == '/');
  }
  std::string path = build_settings->build_dir().value();
  path.append(val);
  NormalizePath(&path);
  return SourceDir(std::move(path));
}

std::string Pretty(const OutputFile& file) {
  return std::string(file.value());
}
