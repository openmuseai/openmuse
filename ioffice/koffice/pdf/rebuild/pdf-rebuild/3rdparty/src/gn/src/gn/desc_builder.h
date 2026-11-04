// Copyright (c) 2016 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_DESC_BUILDER_H_
#define TOOLS_GN_DESC_BUILDER_H_

#include "base/values.h"
#include "gn/target.h"

class ResolvedTargetData;

class DescBuilder {
 public:
  // Creates Dictionary representation for given target.
  //
  // If |resolved| is non-null it is used to compute (and memoize) inherited
  // lib/framework information. Callers that describe many targets in a row
  // should pass a single shared instance to avoid recomputing the transitive
  // dependency walk for every target (which is quadratic otherwise).
  static std::unique_ptr<base::DictionaryValue> DescriptionForTarget(
      const Target* target,
      const std::string& what,
      bool all,
      bool tree,
      bool blame,
      ResolvedTargetData* resolved = nullptr);

  // Creates Dictionary representation for given config
  static std::unique_ptr<base::DictionaryValue> DescriptionForConfig(
      const Config* config,
      const std::string& what);
};

#endif
