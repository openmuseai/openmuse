// Copyright (c) 2013 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/builder_record.h"

#include "gn/item.h"

BuilderRecord::BuilderRecord(ItemType type,
                             const Label& label,
                             const ParseNode* originally_referenced_from)
    : type_(type),
      label_(label),
      originally_referenced_from_(originally_referenced_from) {}

// static
const char* BuilderRecord::GetNameForType(ItemType type) {
  switch (type) {
    case ITEM_TARGET:
      return "target";
    case ITEM_CONFIG:
      return "config";
    case ITEM_TOOLCHAIN:
      return "toolchain";
    case ITEM_POOL:
      return "pool";
    case ITEM_UNKNOWN:
    default:
      return "unknown";
  }
}

// static
bool BuilderRecord::IsItemOfType(const Item* item, ItemType type) {
  switch (type) {
    case ITEM_TARGET:
      return !!item->AsTarget();
    case ITEM_CONFIG:
      return !!item->AsConfig();
    case ITEM_TOOLCHAIN:
      return !!item->AsToolchain();
    case ITEM_POOL:
      return !!item->AsPool();
    case ITEM_UNKNOWN:
    default:
      return false;
  }
}

// static
BuilderRecord::ItemType BuilderRecord::TypeOfItem(const Item* item) {
  if (item->AsTarget())
    return ITEM_TARGET;
  if (item->AsConfig())
    return ITEM_CONFIG;
  if (item->AsToolchain())
    return ITEM_TOOLCHAIN;
  if (item->AsPool())
    return ITEM_POOL;

  NOTREACHED();
  return ITEM_UNKNOWN;
}

#if DEBUG_BUILDER_RECORD
// static
const char* BuilderRecord::ToDebugCString(RecordState state) {
  switch (state) {
    case STATE_INIT:
      return "INIT";
    case STATE_DEFINED:
      return "DEFINED";
    case STATE_RESOLVED:
      return "RESOLVED";
    case STATE_FINALIZED:
      return "FINALIZED";
    default:
      return "<<<<UNKNOWN STATE>>>>>>";
  }
}
#endif  // DEBUG_BUILDER_RECORD

void BuilderRecord::SetDefined(std::unique_ptr<Item> item) {
  DCHECK(state_ == STATE_INIT);
  state_ = STATE_DEFINED;
  item_ = std::move(item);
}

void BuilderRecord::AddDep(BuilderRecord* dep) {
  DCHECK(state_ == STATE_DEFINED);
  all_deps_.add(dep);
  WaitInfo* info = nullptr;
  if (!dep->is_resolved()) {
    // This cannot be resolved yet if any of its standard dependencies is not
    // resolved.
    if (!info)
      info = &dep->waiting_map_[this];
    if (!info->wait_resolved) {
      info->wait_resolved = true;
      unresolved_count_++;
      DEBUG_BUILDER_RECORD_LOG("-- AddDep %s -wait_resolved-> %s\n",
                               this->ToDebugString().c_str(),
                               dep->ToDebugString().c_str());
    }
  }
  if (!dep->is_finalized()) {
    // Finalization of the current record is blocked until the
    // dependency is finalized.
    if (!info)
      info = &dep->waiting_map_[this];
    if (!info->wait_finalized) {
      info->wait_finalized = true;
      unfinalized_count_++;
      DEBUG_BUILDER_RECORD_LOG("-- AddDep %s -wait_finalized-> %s\n",
                               this->ToDebugString().c_str(),
                               dep->ToDebugString().c_str());
    }
  }
}

void BuilderRecord::AddGenDep(BuilderRecord* dep) {
  DCHECK(state_ == STATE_DEFINED);
  all_deps_.insert(dep);
}

void BuilderRecord::AddValidationDep(BuilderRecord* dep) {
  DCHECK(state_ == STATE_DEFINED);
  // Validation dependencies are treated differently than normal dependencies:
  // 1. They are added to all_deps_ for graph traversal (should_generate
  //    propagation) and error checking.
  // 2. They block resolution ONLY if they are undefined (!item). This allows
  //    cycles (A is validated by B, B depends on A) to resolve.
  // 3. They block writing if they are unresolved. This prevents race conditions
  //    where we might write the ninja file before the validation output path
  //    is computed.
  all_deps_.add(dep);
  WaitInfo* info = nullptr;
  if (!dep->is_defined()) {
    // The record cannot be resolved yet if any of its validation dependencies
    // is not defined.
    if (!info)
      info = &dep->waiting_map_[this];
    if (!info->wait_validation_defined) {
      info->wait_validation_defined = true;
      unresolved_count_++;
      DEBUG_BUILDER_RECORD_LOG(
          "-- AddValidationDep %s -wait_validation_defined-> %s\n",
          this->ToDebugString().c_str(), dep->ToDebugString().c_str());
    }
  }
  if (!dep->is_resolved()) {
    // This record cannot be finalized if any of its validation deps is not
    // resolved.
    if (!info)
      info = &dep->waiting_map_[this];
    if (!info->wait_validation_resolved) {
      info->wait_validation_resolved = true;
      unfinalized_count_++;
      DEBUG_BUILDER_RECORD_LOG(
          "-- AddValidationDep %s -wait_validation_resolved-> %s\n",
          this->ToDebugString().c_str(), dep->ToDebugString().c_str());
    }
  }
}

bool BuilderRecord::OnDepStateChange(BuilderRecord* dep,
                                     RecordState from_state,
                                     RecordState to_state,
                                     BuilderRecord::WaitInfo& info) {
  bool result = false;

  DCHECK(all_deps_.contains(dep));

  DEBUG_BUILDER_RECORD_LOG(
      "-- OnDepStateChange %s -> %s (%s->%s)\n", this->ToDebugString().c_str(),
      dep->ToDebugString().c_str(), ToDebugCString(from_state),
      ToDebugCString(to_state));
  if (to_state >= STATE_DEFINED && info.wait_validation_defined) {
    info.wait_validation_defined = false;
    DCHECK(state_ < STATE_RESOLVED);
    DCHECK(unresolved_count_ > 0);
    if (--unresolved_count_ == 0) {
      DEBUG_BUILDER_RECORD_LOG("    CAN RESOLVE (validation)\n");
      result = true;
    }
  }
  if (to_state >= STATE_RESOLVED && info.wait_resolved) {
    info.wait_resolved = false;
    DCHECK(state_ < STATE_RESOLVED);
    DCHECK(unresolved_count_ > 0);
    if (--unresolved_count_ == 0) {
      DEBUG_BUILDER_RECORD_LOG("    CAN RESOLVE\n");
      result = true;
    }
  }
  if (to_state >= STATE_RESOLVED && info.wait_validation_resolved) {
    info.wait_validation_resolved = false;
    DCHECK(state_ < STATE_FINALIZED);
    DCHECK(unfinalized_count_ > 0);
    if (--unfinalized_count_ == 0) {
      DEBUG_BUILDER_RECORD_LOG("    CAN FINALIZE (validation)\n");
      result = true;
    }
  }
  if (to_state >= STATE_FINALIZED && info.wait_finalized) {
    info.wait_finalized = false;
    DCHECK(state_ < STATE_FINALIZED);
    DCHECK(unfinalized_count_ > 0);
    if (--unfinalized_count_ == 0) {
      DEBUG_BUILDER_RECORD_LOG("    CAN FINALIZE\n");
      result = true;
    }
  }
  return result;
}

void BuilderRecord::SetResolved() {
  DCHECK(can_resolve());
  state_ = STATE_RESOLVED;
}

void BuilderRecord::SetFinalized() {
  DCHECK(can_finalize());
  state_ = STATE_FINALIZED;
}

std::vector<const BuilderRecord*> BuilderRecord::GetSortedUnresolvedDeps()
    const {
  std::vector<const BuilderRecord*> result;
  for (auto it = all_deps_.begin(); it.valid(); ++it) {
    const BuilderRecord* dep = *it;
    auto wait_it = dep->waiting_map_.find(this);
    if (wait_it == dep->waiting_map_.end())
      continue;
    if (wait_it->second.wait_resolved ||
        wait_it->second.wait_validation_defined) {
      result.push_back(dep);
    }
  }
  std::sort(result.begin(), result.end(), LabelCompare);
  return result;
}

#if DEBUG_BUILDER_RECORD
std::string BuilderRecord::ToDebugString() const {
  std::string result = label_.GetUserVisibleName(false);
  result += " (";
  switch (state_) {
    case STATE_INIT:
      result += "INIT";
      break;
    case STATE_DEFINED:
      result += "DEFINED";
      break;
    case STATE_RESOLVED:
      result += "RESOLVED";
      break;
    case STATE_FINALIZED:
      result += "FINALIZED";
      break;
    default:
      result += "UNKNOWN";
      break;
  }
  result += " unresolved=" + std::to_string(unresolved_count_);
  result += " unfinalized=" + std::to_string(unfinalized_count_);
  if (should_generate_)
    result += " generated";
  result += ")";
  return result;
}
#endif  // DEBUG_BUILDER_RECORD
