// Copyright (c) 2013 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_OUTPUT_FILE_H_
#define TOOLS_GN_OUTPUT_FILE_H_

#include <stddef.h>

#include <string>
#include <string_view>

#include "gn/string_atom.h"

class BuildSettings;
class SourceDir;
class SourceFile;

// A simple wrapper around StringAtom that indicates the path
// relative to the output directory.
class OutputFile {
 public:
  OutputFile() = default;

  explicit OutputFile(std::string_view v) : value_(v) {}

  OutputFile(const BuildSettings* build_settings,
             const SourceFile& source_file);

  std::string_view value() const { return value_; }

  // Converts to a SourceFile by prepending the build directory to the file.
  // The *Dir version requires that the current OutputFile ends in a slash, and
  // the *File version is the opposite.
  SourceFile AsSourceFile(const BuildSettings* build_settings) const;
  SourceDir AsSourceDir(const BuildSettings* build_settings) const;

  bool operator==(const OutputFile& other) const = default;
  std::strong_ordering operator<=>(const OutputFile& other) const = default;

 private:
  StringAtom value_;
};

std::string Pretty(const OutputFile& file);

namespace std {

template <>
struct hash<OutputFile> {
  std::size_t operator()(const OutputFile& v) const {
    return hash<std::string_view>()(v.value());
  }
};

}  // namespace std

#endif  // TOOLS_GN_OUTPUT_FILE_H_
