// Copyright 2016 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_COMAND_FORMAT_H_
#define TOOLS_GN_COMAND_FORMAT_H_

#include <string>

#include "gn/err.h"

class ParseNode;
class Setup;
class SourceFile;

namespace commands {

enum class TreeDumpMode {
  // Normal operation mode. Format the input file.
  kInactive,

  // Output the token tree with indented plain text. For debugging.
  kPlainText,

  // Output the token tree in JSON format. Used for exporting a tree to another
  // program.
  kJSON
};

// The default format width.
extern const size_t kDefaultFormatWidth;

bool FormatJsonToString(const std::string& input,
                        size_t format_width,
                        std::string* output);

bool FormatStringToString(const std::string& input,
                          TreeDumpMode dump_tree,
                          size_t format_width,
                          std::string* output,
                          std::string* dump_output);

Result<std::string> FormatNodeToString(const ParseNode* root);

}  // namespace commands

#endif  // TOOLS_GN_COMAND_FORMAT_H_
