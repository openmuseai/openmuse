// Copyright 2026 The GN Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/edit_subcommands.h"

#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "base/containers/span.h"
#include "base/strings/string_number_conversions.h"
#include "base/strings/string_util.h"
#include "gn/build_file_editor.h"
#include "gn/err.h"
#include "gn/location.h"
#include "gn/parse_tree.h"
#include "gn/value.h"

namespace {

// Parses a single string argument into a primitive GN Value (bool, int, or
// string).
Result<Value> ParseValue(std::string_view val_string) {
  if (val_string == "true") {
    return Value(nullptr, true);
  }
  if (val_string == "false") {
    return Value(nullptr, false);
  }

  int64_t result_int;
  if (base::StringToInt64(val_string, &result_int)) {
    return Value(nullptr, result_int);
  }

  return Value(nullptr, std::string(val_string));
}

// Parses multiple string arguments into a vector of GN Values.
Result<std::vector<Value>> ParseValues(base::span<const std::string> values) {
  std::vector<Value> list_elements;
  list_elements.reserve(values.size());
  for (const std::string& val_str : values) {
    ASSIGN_OR_RETURN(Value val, ParseValue(val_str));
    list_elements.push_back(std::move(val));
  }
  return list_elements;
}

std::pair<std::string_view, std::optional<std::string_view>> SplitAttrType(
    std::string_view arg) {
  size_t colon_pos = arg.find(':');
  if (colon_pos == std::string_view::npos) {
    return {arg, std::nullopt};
  }
  return {arg.substr(0, colon_pos), arg.substr(colon_pos + 1)};
}

using ParseNodeGenerator =
    std::function<Result<std::unique_ptr<ParseNode>>(BuildFile& build_file)>;

Result<ParseNodeGenerator> CreateParseNodeGenerator(
    std::optional<std::string_view> kind,
    base::span<const std::string> values) {
  CHECK(!values.empty());

  if (kind == "expr") {
    std::string expr_string = base::JoinString(
        std::vector<std::string_view>(values.begin(), values.end()), " ");
    return [expr_string = std::move(expr_string)](BuildFile& build_file) {
      return build_file.parse_expression(expr_string);
    };
  } else if (kind == "list" || (!kind && values.size() > 1)) {
    ASSIGN_OR_RETURN(std::vector<Value> out, ParseValues(values));
    return [out = std::move(out)](
               BuildFile& build_file) -> Result<std::unique_ptr<ParseNode>> {
      return build_file.to_node(Value(nullptr, std::vector<Value>(out)));
    };
  } else if (!kind.has_value()) {
    ASSIGN_OR_RETURN(Value val, ParseValue(values[0]));
    return [val = std::move(val)](
               BuildFile& build_file) -> Result<std::unique_ptr<ParseNode>> {
      return build_file.to_node(val);
    };
  }

  return Err(Location(), "Unknown type: :" + std::string(*kind),
             "Supported types are :list and :expr.");
}

const TreeNode* FirstUnconditionalAssignment(
    const std::vector<TreeNode>& assignments) {
  for (const auto& assignment : assignments) {
    if (!assignment.is_conditional() && assignment.AsAssignment())
      return &assignment;
  }
  return nullptr;
}

// Helper to create an EditCommand that loops over all matched targets in a
// BuildFile.
EditCommand EditTargetCommand(
    std::function<Err(BuildFile&, const EditTarget&, EditState&)>
        apply_to_target) {
  return [apply_to_target = std::move(apply_to_target)](
             BuildFile& build_file, EditState& state) -> Err {
    for (const auto& target : build_file.targets()) {
      RETURN_IF_ERROR(apply_to_target(build_file, target, state));
    }
    return Ok();
  };
}

bool RemoveFromTarget(const EditTarget& target,
                      const std::string& attribute,
                      const Value& value,
                      EditState& state) {
  bool done = false;
  for (auto& assignment : target.assignments(attribute)) {
    auto matches = FindListElementInAssignment(target, assignment, value);

    for (const auto& match : matches) {
      match.RemoveSelf(state, target);
    }
    done |= !matches.empty();
  }

  if (done) {
    auto assignments = target.assignments(attribute);
    for (auto i = 0u; i < assignments.size(); ++i) {
      auto& assign = assignments[i];
      auto* op = assignments[i]->AsBinaryOpMut();
      CHECK(op);
      TreeNode* next =
          i + 1 < assignments.size() ? &assignments[i + 1] : nullptr;

      op->set_right(SimplifyExpression(op->take_right()));
      if (IsEmptyList(op->right())) {
        if (assign.is_modification() || assignments.size() == 1) {
          assign.RemoveSelfUnconditionally();
          // Transform a = []; a += ["..."] => a = ["..."]
          // This can be safely done if `a = []` is unconditional, and either
          // the += is unconditional, or we know it's the last assignment.
        } else if (next && !assign.is_conditional() &&
                   (!next->is_conditional() || assignments.size() == 2)) {
          auto* next_op = next->node()->AsBinaryOpMut();
          if (next_op->op().type() == Token::PLUS_EQUALS) {
            next_op->set_op(Token(next_op->op().location(), Token::EQUAL, "="));
            assign.RemoveSelfUnconditionally();
          } else if (next_op->op().type() == Token::EQUAL) {
            assign.RemoveSelfUnconditionally();
          }
        }
      }
    }
  } else if (target.is_explicit) {
    target.add_warning(state, "does not contain the value " +
                                  value.ToString(true) + " in attribute \"" +
                                  attribute + "\".");
  }
  return done;
}

void AddToTarget(BuildFile& build_file,
                 const EditTarget& target,
                 const std::string& attribute,
                 const std::vector<Value>& values) {
  auto assignments = target.assignments(attribute);
  std::vector<Value> to_add = values;

  // Iterate over a copy of values since we're mutating it.
  for (const auto& value : values) {
    for (auto& assignment : assignments) {
      auto matches = FindListElementInAssignment(target, assignment, value);
      for (const auto& match : matches) {
        if (assignment.is_conditional()) {
          // If it's assigned conditionally, remove it from the list first,
          // since we're going to assign it unconditionally.
          // Unlike usual we don't mark this with a comment, because this is
          // safe.
          match.RemoveSelfUnconditionally();
        } else {
          // If it's added unconditionally, we don't need to worry about
          // adding it anymore.
          std::erase(to_add, value);
        }
      }
    }
  }

  if (const auto* first = FirstUnconditionalAssignment(assignments); first) {
    // Case A: There exists an unconditional assignment -> add values to it.
    ListNode* target_list = nullptr;
    if (auto list = FindListInAssignment(*first)) {
      // The expression is something like `[ "a" ]` or `foo + [ "a" ]`
      // In this case we just add directly to the first list "literal" we
      // find.
      target_list = *list;
    } else {
      // The expression doesn't have a list literal (eg. `foo`)
      // Rewrite it as `[] + foo` so we can add to the empty list.
      auto* op = first->AsAssignment();
      auto empty_list_val =
          build_file.to_node(Value(nullptr, std::vector<Value>{}));
      target_list = empty_list_val->AsListMut();

      auto plus_node = std::make_unique<BinaryOpNode>();
      plus_node->set_op(Token(build_file.location(), Token::PLUS, "+"));
      plus_node->set_left(std::move(empty_list_val));
      plus_node->set_right(op->take_right());

      op->set_right(std::move(plus_node));
    }

    for (const auto& value : to_add) {
      target_list->append_item(build_file.to_node(value));
    }
  } else if (!assignments.empty()) {
    // Case B: attr is only defined conditionally -> add attr = [value] right
    // before the conditional statement, change all other assignments to "+=".
    for (auto& assignment : assignments) {
      if (auto* op = assignment->AsBinaryOpMut()) {
        if (op->op().type() == Token::EQUAL) {
          op->set_op(Token(op->op().location(), Token::PLUS_EQUALS, "+="));
        }
      }
    }

    auto stack = assignments[0].stack();
    while (stack.size() >= 2 && stack[stack.size() - 2] != target.block)
      stack.pop_back();

    build_file.assign_in_block(
        target.block,
        std::find_if(
            target.block->statements().begin(),
            target.block->statements().end(),
            [node = stack.back()](const auto& s) { return s.get() == node; }),
        attribute,
        build_file.to_node(Value(nullptr, std::vector<Value>(to_add))));
  } else {
    // Case C: attr is not defined -> insert attr = [value] at the canonically
    // sorted position.
    build_file.assign_in_block(
        target.block, attribute,
        build_file.to_node(Value(nullptr, std::vector<Value>(to_add))));
  }
}

EditCommand AddToAttributeCommand(std::string attribute,
                                  std::vector<Value> values) {
  return EditTargetCommand(
      [attribute = std::move(attribute), values = std::move(values)](
          BuildFile& build_file, const EditTarget& target,
          EditState& state) -> Err {
        AddToTarget(build_file, target, attribute, values);
        return Ok();
      });
}

EditCommand DeleteCommand() {
  return EditTargetCommand([](BuildFile& build_file, const EditTarget& target,
                              EditState& state) -> Err {
    target.node.RemoveSelf(state, target);
    return Ok();
  });
}

EditCommand MoveCommand(std::string from_attribute,
                        std::string to_attribute,
                        std::vector<Value> values) {
  return EditTargetCommand([from_attribute = std::move(from_attribute),
                            to_attribute = std::move(to_attribute),
                            values = std::move(values)](
                               BuildFile& build_file, const EditTarget& target,
                               EditState& state) -> Err {
    std::vector<Value> moved_values;
    for (const auto& value : values) {
      if (RemoveFromTarget(target, from_attribute, value, state)) {
        moved_values.push_back(value);
      }
    }
    if (!moved_values.empty()) {
      AddToTarget(build_file, target, to_attribute, moved_values);
    }
    return Ok();
  });
}

using LocationProvider =
    std::function<Result<TreeNode::NodeLocation>(BuildFile&)>;

EditCommand NewCommand(std::string rule_kind,
                       LocationProvider location_provider) {
  return [rule_kind = std::move(rule_kind),
          location_provider = std::move(location_provider)](
             BuildFile& build_file, EditState& state) -> Err {
    ASSIGN_OR_RETURN(auto rule_names,
                     build_file.label_matcher().explicit_target_names());
    DCHECK(!rule_names.empty());

    ASSIGN_OR_RETURN(auto loc, location_provider(build_file));
    auto& [container, it] = loc;

    for (const auto& rule_name : rule_names) {
      if (auto target = build_file.find_target(rule_name);
          target && !target->node.is_conditional()) {
        return Err(Location(), "Target \"" + rule_name +
                                   "\" already exists in " +
                                   build_file.source_file().value() + ".");
      }

      // Reassign and advance the iterator returned by insert() to ensure valid
      // iterators across vector reallocations and preserve insertion order
      // when adding multiple targets.
      it = container.insert(
          it, build_file.create_target(rule_kind, rule_name,
                                       build_file.create_block()));
      ++it;
    }
    return Ok();
  };
}

EditCommand RemoveAttributeCommand(std::string attribute) {
  return EditTargetCommand([attribute = std::move(attribute)](
                               BuildFile& build_file, const EditTarget& target,
                               EditState& state) -> Err {
    auto assignments = target.assignments(attribute);
    for (auto& assignment : assignments) {
      assignment.RemoveSelf(state, target);
    }
    if (assignments.empty() && target.is_explicit) {
      target.add_warning(
          state, "does not contain the attribute \"" + attribute + "\".");
    }
    return Ok();
  });
}

EditCommand RemoveFromAttributeCommand(std::string attribute,
                                       std::vector<Value> values) {
  return EditTargetCommand(
      [attribute = std::move(attribute), values = std::move(values)](
          BuildFile& build_file, const EditTarget& target,
          EditState& state) -> Err {
        for (const auto& value : values) {
          RemoveFromTarget(target, attribute, value, state);
        }
        return Ok();
      });
}

EditCommand RenameAttributeCommand(std::string_view from_attribute,
                                   std::string_view to_attribute) {
  return EditTargetCommand([from_attribute = std::string(from_attribute),
                            to_attribute = std::string(to_attribute)](
                               BuildFile& build_file, const EditTarget& target,
                               EditState& state) -> Err {
    auto assignments = target.assignments(from_attribute);
    for (auto& assignment : assignments) {
      assignment->AsBinaryOpMut()->set_left(
          build_file.create_identifier(to_attribute));
    }
    if (assignments.empty() && target.is_explicit) {
      target.add_warning(
          state, "does not contain the attribute \"" + from_attribute + "\".");
    }
    return Ok();
  });
}

// Sets an attribute to an expression
EditCommand SetCommand(std::string attribute, ParseNodeGenerator generator) {
  return EditTargetCommand(
      [attribute = std::move(attribute), generator = std::move(generator)](
          BuildFile& build_file, const EditTarget& target,
          EditState& state) -> Err {
        ASSIGN_OR_RETURN(auto node, generator(build_file));
        auto assignments = target.assignments(attribute);
        const auto* first = FirstUnconditionalAssignment(assignments);
        for (const auto& assignment : assignments) {
          if (&assignment != first) {
            assignment.RemoveSelf(state, target);
          }
        }

        if (first) {
          (*first)->AsBinaryOpMut()->set_right(std::move(node));
        } else {
          build_file.assign_in_block(target.block, attribute, std::move(node));
        }

        return Ok();
      });
}

Result<std::string> GetShardName(const LocationRange& location,
                                 std::string_view path) {
  if (path.empty() || path.starts_with("/")) {
    return Err(
        location,
        "Cannot shard target with non-relative source path: " +
            std::string(path),
        "Determining a shard name is not supported for non-relative paths.");
  }
  if (path.starts_with("./")) {
    path.remove_prefix(2);
  }

  size_t last_slash = path.rfind('/');
  size_t last_dot = path.rfind('.');
  if (last_dot != std::string_view::npos &&
      (last_slash == std::string_view::npos || last_dot > last_slash)) {
    path = path.substr(0, last_dot);
  }

  std::string shard_name;
  shard_name.reserve(path.size());
  for (char c : path) {
    if (c == '/' || c == '\\' || c == '.' || c == '-') {
      shard_name.push_back('_');
    } else {
      shard_name.push_back(c);
    }
  }
  return shard_name;
}

EditCommand ShardCommand(
    std::optional<std::string> shard_target_type = std::nullopt,
    std::string group_type = "group") {
  return EditTargetCommand([shard_target_type, group_type](
                               BuildFile& build_file, const EditTarget& target,
                               EditState& state) -> Err {
    std::set<std::string> shards;
    for (auto assign : target.assignments({"sources", "public"})) {
      for (const auto& item : FindAllListElements(assign)) {
        if (auto lit = AsStringLiteral(item.node()); lit) {
          ASSIGN_OR_RETURN(std::string shard_name,
                           GetShardName(item->GetRange(), *lit));
          shards.insert(std::move(shard_name));
        }
      }
    }

    if (shards.size() <= 1) {
      target.add_warning(state, "does not need to be sharded.");
      return Ok();
    }

    for (auto assign : target.assignments({"public_deps", "deps"})) {
      assign.RemoveSelfUnconditionally();
    }

    std::vector<Value> target_names;
    auto [container, it] = target.node.node_location();
    // Ensure we insert the shard after the group.
    ++it;

    for (const auto& shard : shards) {
      // Note: If a target "foo" contains both "foo.cc" and "foo_foo.cc", both
      // would map to "foo_foo". This is a rare edge case that is not worth
      // complex disambiguation logic.
      std::string target_name = (shard == target.label.name())
                                    ? (target.label.name() + "_" + shard)
                                    : shard;
      target_names.push_back(Value(nullptr, ":" + target_name));
      state.needs_fix_deps.insert(Label(target.label.dir(), target_name));

      auto node = target.node.node()->Clone();
      auto* func = node->AsFunctionCallMut();
      if (shard_target_type) {
        func->set_function(Token(func->function().location(), Token::IDENTIFIER,
                                 *shard_target_type));
      }
      it = container.insert(it, std::move(node));
      ++it;

      func->args()->contents()[0] =
          build_file.to_node(Value(nullptr, target_name));

      for (auto assign :
           TreeNode({func->block()}).assignments({"sources", "public"})) {
        for (const auto& item : FindAllListElements(assign)) {
          if (auto lit = AsStringLiteral(item.node()); lit) {
            auto res = GetShardName(item->GetRange(), *lit);
            DCHECK(!res.has_error()) << "should have failed above";
            if (*res != shard) {
              item.RemoveSelfUnconditionally();
            }
          }
        }
      }
    }

    auto* orig_func = target.node.node()->AsFunctionCallMut();
    orig_func->set_function(
        Token(orig_func->function().location(), Token::IDENTIFIER, group_type));

    std::vector<std::unique_ptr<ParseNode>> group_stmts;
    group_stmts.push_back(build_file.create_assignment(
        "public_deps",
        build_file.to_node(Value(nullptr, std::move(target_names)))));
    for (const auto& assign : target.assignments({"testonly", "visibility"})) {
      auto node = assign.node()->Clone();
      if (assign.is_conditional()) {
        TreeNode({node.get()})
            .add_todo(
                state, target,
                "This was conditional in the original target. Manual review is "
                "required to decide if it applies to this group target.");
      }
      group_stmts.push_back(std::move(node));
    }

    target.block->statements() = std::move(group_stmts);
    return Ok();
  });
}

}  // namespace

Result<EditCommand> ParseCommand(std::vector<std::string> args) {
  if (args.empty()) {
    return Err(Location(), "Empty command.");
  }

  if (args[0] == "add") {
    if (args.size() < 3) {
      return Err(Location(), "Invalid add command.",
                 "Usage: add <attribute> <value(s)>");
    }
    ASSIGN_OR_RETURN(std::vector<Value> values,
                     ParseValues(base::make_span(args).subspan(2)));
    return AddToAttributeCommand(args[1], std::move(values));
  } else if (args[0] == "delete") {
    if (args.size() != 1) {
      return Err(Location(), "Invalid delete command.", "Usage: delete");
    }
    return DeleteCommand();
  } else if (args[0] == "move") {
    if (args.size() < 4) {
      return Err(Location(), "Invalid move command.",
                 "Usage: move <from_attribute> <to_attribute> <value(s)>");
    }
    ASSIGN_OR_RETURN(std::vector<Value> values,
                     ParseValues(base::make_span(args).subspan(3)));
    return MoveCommand(args[1], args[2], std::move(values));
  } else if (args[0] == "new") {
    if (args.size() == 2) {
      return NewCommand(
          args[1], [](BuildFile& build_file) -> Result<TreeNode::NodeLocation> {
            auto* root = build_file.root()->AsBlockMut();
            return std::make_pair(std::ref(root->statements()),
                                  root->statements().end());
          });
    } else if (args.size() == 4 &&
               (args[2] == "before" || args[2] == "after")) {
      return NewCommand(
          args[1],
          [rel = args[3], after = args[2] == "after"](
              BuildFile& build_file) -> Result<TreeNode::NodeLocation> {
            auto target = build_file.find_target(rel);
            if (!target) {
              return Err(Location(), "Target \"" + rel + "\" not found in " +
                                         build_file.source_file().value() +
                                         ".");
            }
            auto [container, it] = target->node.node_location();
            if (after)
              it++;
            return std::make_pair(std::ref(container), it);
          });
    } else {
      return Err(Location(), "Invalid new command.",
                 "Usage: new <rule_kind> [(before|after) "
                 "<relative_rule_name>]");
    }
  } else if (args[0] == "remove") {
    if (args.size() < 2) {
      return Err(Location(), "Invalid remove command.",
                 "Usage: remove <attribute> [<value(s)>]");
    } else if (args.size() == 2) {
      return RemoveAttributeCommand(args[1]);
    }
    ASSIGN_OR_RETURN(std::vector<Value> values,
                     ParseValues(base::make_span(args).subspan(2)));
    return RemoveFromAttributeCommand(args[1], std::move(values));
  } else if (args[0] == "rename") {
    if (args.size() != 3) {
      return Err(Location(), "Invalid rename command.",
                 "Usage: rename <from_attribute> <to_attribute>");
    }
    return RenameAttributeCommand(args[1], args[2]);
  } else if (args[0] == "set") {
    if (args.size() < 3) {
      return Err(Location(),
                 "Invalid set command: missing attribute or value.\n"
                 "Usage: set <attribute>[:type] <value...>");
    }

    auto [attr, kind] = SplitAttrType(args[1]);
    ASSIGN_OR_RETURN(
        auto generator,
        CreateParseNodeGenerator(kind, base::make_span(args).subspan(2)));
    return SetCommand(std::string(attr), std::move(generator));
  } else if (args[0] == "shard") {
    if (args.size() == 1) {
      return ShardCommand();
    } else if (args.size() == 2) {
      return ShardCommand(args[1]);
    } else if (args.size() == 3) {
      return ShardCommand(args[1], args[2]);
    } else {
      return Err(Location(), "Invalid shard command.",
                 "Usage: shard [sharded target type] [group type]");
    }
  } else {
    return Err(Location(),
               "Unknown edit command: " + std::string(args[0]) +
                   "\n"
                   "See `gn help edit` for list of supported commands.");
  }
}
