// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_FFI_ERR_H_
#define TOOLS_GN_FFI_ERR_H_

#include <memory>
#include <string>
#include <vector>

#include "cxx.h"
#include "gn/err.h"
#include "gn/input_file.h"
#include "gn/input_file_manager.h"
#include "gn/scheduler.h"
#include "gn/source_file.h"

inline const InputFile& NewInputFile(rust::Str name, rust::Str code) {
  // Yes, the same file can be created multiple times.
  // No, we don't really care, since this is only for error messages.
  SourceFile source_file{std::string(name)};
  InputFileManager* manager = g_scheduler->input_file_manager();

  InputFile* file = nullptr;
  std::vector<Token>* tokens = nullptr;
  std::unique_ptr<ParseNode>* parse_root = nullptr;
  manager->AddDynamicInput(source_file, &file, &tokens, &parse_root);
  file->SetContents(std::string(code));
  file->set_friendly_name(std::string(name));
  return *file;
}

inline void PopulateErrWithLocation(Err& err,
                                    rust::Str message,
                                    rust::Str help,
                                    const InputFile& file,
                                    int start_line,
                                    int start_column,
                                    int end_line,
                                    int end_column) {
  Location begin(&file, start_line, start_column);
  Location end(&file, end_line, end_column);
  LocationRange range(begin, end);
  err = Err(range, std::string(message), std::string(help));
}

inline void PopulateErrWithMessage(Err& err,
                                   rust::Str message,
                                   rust::Str help) {
  err = Err(Location(), std::string(message), std::string(help));
}

inline void AppendSubErr(Err& err,
                         rust::Str message,
                         const InputFile& file,
                         int start_line,
                         int start_column,
                         int end_line,
                         int end_column) {
  Location begin(&file, start_line, start_column);
  Location end(&file, end_line, end_column);
  LocationRange range(begin, end);
  err.AppendSubErr(Err(range, std::string(message)));
}

inline std::unique_ptr<Err> NewErr() {
  return std::make_unique<Err>();
}

inline rust::String ErrToString(const Err& err) {
  return rust::String(err.to_string());
}

#endif  // TOOLS_GN_FFI_ERR_H_
