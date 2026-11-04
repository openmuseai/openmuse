// Copyright 2026 The GN Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "gn/build_file_editor.h"

#include <algorithm>

#include "base/files/file_enumerator.h"
#include "base/files/file_util.h"
#include "gn/build_settings.h"
#include "gn/command_format.h"
#include "gn/edit_subcommands.h"
#include "gn/filesystem_utils.h"
#include "gn/input_file.h"
#include "gn/label.h"
#include "gn/loader.h"
#include "gn/parse_tree.h"
#include "gn/parser.h"
#include "gn/scope.h"
#include "gn/string_atom.h"
#include "gn/tokenizer.h"
#include "gn/value.h"

namespace {

std::optional<Value> AsLiteralValue(const ParseNode* node) {
  auto* literal = node->AsLiteral();
  if (!literal) {
    return std::nullopt;
  }
  Scope scope(static_cast<const Settings*>(nullptr));
  Err err;
  Value v = literal->Execute(&scope, &err);
  // Literals should *usually* not error out, but there are some cases they do.
  // Eg. the string literal "${foo}" with no variable foo in scope.
  // When this happens, just treat them as if they're opaque things we don't
  // know about.
  if (err.has_error()) {
    return std::nullopt;
  }
  return v;
}

// Returns true if a node in the tree is a literal node matching the user's
// request.
bool Matches(const EditTarget& target,
             const ParseNode* node,
             const Value& value) {
  auto got_value = AsLiteralValue(node);
  if (!got_value) {
    return false;
  }
  if (*got_value == value) {
    return true;
  }
  if (got_value->type() == Value::STRING && value.type() == Value::STRING) {
    // If the user requests "remove deps //foo:bar" //foo:baz, and //foo:baz
    // contains the literal ":bar", that should match.
    Err err;
    Label got_label =
        Label::Resolve(target.label.dir(), "", target.label.GetToolchainLabel(),
                       *got_value, &err);
    Label want_label = Label::Resolve(
        target.label.dir(), "", target.label.GetToolchainLabel(), value, &err);
    return !err.has_error() && got_label == want_label;
  }
  return false;
}

// Finds matching nodes in an expression.
template <typename T>
void FindExpressionRecursive(
    ParseNode* node,
    std::vector<ParseNode*>& stack,
    const std::function<std::optional<T>(TreeNode&)>& transform,
    std::vector<T>* results) {
  if (!node)
    return;

  stack.push_back(node);

  TreeNode node_ref(stack);
  if (auto mapped = transform(node_ref)) {
    results->push_back(std::move(*mapped));
  }

  if (auto* list = node->AsListMut()) {
    for (auto& item : list->contents()) {
      FindExpressionRecursive(item.get(), stack, transform, results);
    }
  } else if (auto* op = node->AsBinaryOpMut()) {
    if (op->op().type() == Token::PLUS) {
      FindExpressionRecursive(op->left(), stack, transform, results);
      FindExpressionRecursive(op->right(), stack, transform, results);
    }
  }

  stack.pop_back();
}

// Finds matching nodes in an expression.
template <typename T>
std::vector<T> FindExpression(
    const TreeNode& root,
    const std::function<std::optional<T>(TreeNode&)>& transform) {
  std::vector<T> results;
  std::vector<ParseNode*> stack = root.stack();
  stack.pop_back();
  FindExpressionRecursive<T>(root.node(), stack, transform, &results);
  return results;
}

// Resolves a single LabelPattern to matching SourceFiles.
Result<std::vector<SourceFile>> ResolvePatternToFiles(
    const BuildSettings* build_settings,
    const Loader* loader,
    const LabelPattern& pattern) {
  std::vector<SourceFile> matched_files;
  auto add_dir = [&](const SourceDir& dir) {
    auto build_file = loader->BuildFileForLabel(Label(dir, "dummy"));
    if (base::PathExists(build_settings->GetFullPath(build_file))) {
      matched_files.push_back(build_file);
    }
  };

  add_dir(pattern.dir());

  if (pattern.type() == LabelPattern::MATCH ||
      pattern.type() == LabelPattern::DIRECTORY) {
    if (matched_files.empty()) {
      return Err(
          Location(),
          "Build file does not exist: " +
              loader->BuildFileForLabel(Label(pattern.dir(), "dummy")).value());
    }
    return matched_files;
  }

  base::FilePath disk_path = build_settings->GetFullPath(pattern.dir());
  if (!base::DirectoryExists(disk_path)) {
    return Err(Location(),
               "Directory does not exist: " + pattern.dir().value());
  }

  base::FileEnumerator traverser(disk_path, /*recursive=*/true,
                                 base::FileEnumerator::DIRECTORIES);
  for (base::FilePath current = traverser.Next(); !current.empty();
       current = traverser.Next()) {
    base::FilePath relative;
    if (build_settings->root_path().AppendRelativePath(current, &relative)) {
      std::string source_path = "//" + FilePathToUTF8(relative) + "/";
      NormalizePath(&source_path);
      add_dir(SourceDir(source_path));
    }
  }

  return matched_files;
}

// See style_guide.md, "Ordering within a target"
std::optional<int> GetAttributeOrder(std::string_view attribute) {
  // 100 => Target metadata
  if (attribute == "output_name" || attribute == "output_prefix_override" ||
      attribute == "output_dir" || attribute == "output_extension") {
    return 100;
  } else if (attribute == "visibility") {
    return 110;
  } else if (attribute == "testonly") {
    return 120;
  } else if (attribute == "friend") {
    return 130;
    // 200 => inputs / outputs
  } else if (attribute == "script") {
    return 200;
  } else if (attribute == "args") {
    return 210;
  } else if (attribute == "sources") {
    return 220;
  } else if (attribute == "public") {
    return 230;
  } else if (attribute == "inputs") {
    return 240;
  } else if (attribute == "outputs") {
    return 250;
  } else if (attribute == "depfile" || attribute == "response_file_contents") {
    return 260;
    // 300 => config
  } else if (attribute == "defines" || attribute == "include_dirs" ||
             attribute.starts_with("cflags") || attribute == "asmflags" ||
             attribute == "ldflags" || attribute == "arflags" ||
             attribute == "data" || attribute == "rustflags" ||
             attribute.ends_with("configs")) {
    return 300;
    // 400 => deps
  } else if (attribute == "public_deps") {
    return 400;
  } else if (attribute == "deps") {
    return 410;
  } else if (attribute == "data_deps") {
    return 420;
  } else {
    return std::nullopt;
  }
}

std::optional<int> GetOrder(const ParseNode* node);

std::vector<std::optional<int>> GetOrders(
    const std::vector<std::unique_ptr<ParseNode>>& nodes) {
  std::vector<std::optional<int>> orders;
  orders.reserve(nodes.size());
  for (const auto& node : nodes) {
    orders.push_back(GetOrder(node.get()));
  }
  return orders;
}

std::optional<int> GetOrder(const ParseNode* node) {
  if (!node)
    return std::nullopt;

  if (const auto* op = node->AsBinaryOp()) {
    if (op->op().type() == Token::EQUAL ||
        op->op().type() == Token::PLUS_EQUALS ||
        op->op().type() == Token::MINUS_EQUALS) {
      if (const auto* id = op->left()->AsIdentifier()) {
        return GetAttributeOrder(id->value().value());
      }
    }
  } else if (const auto* condition = node->AsCondition()) {
    std::vector<std::optional<int>> orders;
    if (condition->if_true()) {
      auto true_orders = GetOrders(condition->if_true()->statements());
      orders.insert(orders.end(), true_orders.begin(), true_orders.end());
    }
    if (const auto* if_false = condition->if_false()) {
      if (const auto* block = if_false->AsBlock()) {
        auto false_orders = GetOrders(block->statements());
        orders.insert(orders.end(), false_orders.begin(), false_orders.end());
      } else if (auto false_order = GetOrder(if_false)) {
        orders.push_back(false_order);
      }
    }
    // The style guide says:
    // Simple conditions affecting just one variable (e.g. adding a single
    // source or adding a flag for one particular OS) can go beneath the
    // variable they affect. More complicated conditions affecting more than
    // one thing should go at the bottom.
    std::optional<int> result = std::nullopt;
    for (const auto& order : orders) {
      if (order && result && order != result) {
        // Affects more than one thing, goes to the bottom
        return std::numeric_limits<int>::max();
      } else if (!result) {
        result = order;
      }
    }
    if (result) {
      // Affects a single variable.
      return *result;
    }
  }
  return std::nullopt;
}

}  // namespace

std::optional<std::string> AsStringLiteral(const ParseNode* node) {
  auto val = AsLiteralValue(node);
  if (val && val->type() == Value::STRING) {
    return std::move(val->string_value());
  }
  return std::nullopt;
}

std::vector<TreeNode> FindAllListElements(const TreeNode& assignment) {
  auto* op = assignment.AsAssignment();
  if (!op)
    return {};
  return FindExpression<TreeNode>(
      assignment.Descend(op->right()),
      [](TreeNode& node_ref) -> std::optional<TreeNode> {
        if (node_ref.parent() && node_ref.parent()->AsList()) {
          return node_ref;
        }
        return std::nullopt;
      });
}

std::vector<TreeNode> FindListElementInAssignment(const EditTarget& target,
                                                  const TreeNode& root,
                                                  const Value& value) {
  auto* node = root.AsAssignment();
  if (!node)
    return {};
  return FindExpression<TreeNode>(
      root.Descend(node->right()),
      [&](TreeNode& node_ref) -> std::optional<TreeNode> {
        if (node_ref.parent() && node_ref.parent()->AsList() &&
            Matches(target, node_ref.node(), value)) {
          return node_ref;
        }
        return std::nullopt;
      });
}

std::optional<ListNode*> FindListInAssignment(const TreeNode& assignment) {
  auto* op = assignment.AsAssignment();
  if (!op)
    return std::nullopt;
  auto results = FindExpression<ListNode*>(
      assignment.Descend(op->right()),
      [](TreeNode& node_ref) -> std::optional<ListNode*> {
        if (auto* list = node_ref->AsListMut()) {
          return list;
        }
        return std::nullopt;
      });
  if (results.empty())
    return std::nullopt;
  return results.front();
}

bool IsEmptyList(const ParseNode* node) {
  auto* list = node->AsList();
  return list && list->contents().empty();
}

std::unique_ptr<ParseNode> SimplifyExpression(std::unique_ptr<ParseNode> expr) {
  // We could recurse into minuses, but `gn edit` never modifies anything in a
  // minus, so no need to simplify.
  if (auto op = expr->AsBinaryOpMut(); op && op->op().type() == Token::PLUS) {
    op->set_left(SimplifyExpression(op->take_left()));
    op->set_right(SimplifyExpression(op->take_right()));
    if (IsEmptyList(op->left())) {
      return op->take_right();
    } else if (IsEmptyList(op->right())) {
      return op->take_left();
    }
  }
  return expr;
}

TreeNode TreeNode::Descend(ParseNode* child) const {
  std::vector<ParseNode*> s = stack_;
  s.push_back(child);
  return TreeNode(std::move(s));
}

BinaryOpNode* TreeNode::AsAssignment() const {
  if (auto* op = node()->AsBinaryOpMut()) {
    if (op->op().type() == Token::EQUAL ||
        op->op().type() == Token::PLUS_EQUALS) {
      return op;
    }
  }
  return nullptr;
}

bool TreeNode::is_conditional() const {
  DCHECK(!stack_.empty()) << "stack should never be empty";
  for (auto it = stack_.rbegin() + 1; it != stack_.rend(); ++it) {
    if ((*it)->AsCondition()) {
      return true;
    }
    // Stop at the target boundary.
    if ((*it)->AsFunctionCall()) {
      break;
    }
  }
  return false;
}

bool TreeNode::is_modification() const {
  if (const auto* op = node()->AsBinaryOp()) {
    return op->op().type() == Token::PLUS_EQUALS ||
           op->op().type() == Token::MINUS_EQUALS;
  }
  return false;
}

void TreeNode::add_todo(EditState& state,
                        const EditTarget& target,
                        std::string_view message) const {
  std::string line =
      "# TODO(gn edit: " + state.context + "): " + std::string(message);
  StringAtom atom(line);
  Token comment_token(node()->GetRange().begin(), Token::LINE_COMMENT,
                      atom.str());
  node()->comments_mutable()->append_before(std::move(comment_token));
  state.needs_manual_review.insert(target.label);
}

void TreeNode::RemoveSelf(EditState& state, const EditTarget& target) const {
  if (is_conditional()) {
    add_todo(state, target,
             "This would normally be deleted but is conditional. Manual "
             "intervention is required to decide whether it should actually be "
             "deleted.");
  } else {
    RemoveSelfUnconditionally();
  }
}

TreeNode::NodeList& TreeNode::container() const {
  DCHECK(parent());
  if (auto* block = parent()->AsBlockMut()) {
    return block->statements();
  } else if (auto* list = parent()->AsListMut()) {
    return list->contents();
  } else {
    NOTREACHED() << "Unsupported parent type in container";
  }
}

TreeNode::NodeLocation TreeNode::node_location() const {
  auto& c = container();
  auto it = std::find_if(c.begin(), c.end(),
                         [this](const auto& p) { return p.get() == node(); });
  CHECK(it != c.end()) << "child node not found in parent container";
  return {c, it};
}

void TreeNode::RemoveSelfUnconditionally() const {
  auto [container, it] = node_location();
  container.erase(it);
}

LabelMatcher::LabelMatcher(SourceDir source_dir,
                           const std::vector<LabelPattern>& patterns)
    : source_dir_(std::move(source_dir)), globbed_(false) {
  for (const auto& pattern : patterns) {
    if (pattern.type() == LabelPattern::RECURSIVE_DIRECTORY &&
        source_dir_.value().starts_with(pattern.dir().value())) {
      globbed_ = true;
    } else if (pattern.type() == LabelPattern::DIRECTORY &&
               source_dir_ == pattern.dir()) {
      globbed_ = true;
    } else if (pattern.type() == LabelPattern::MATCH &&
               pattern.dir() == source_dir_) {
      if (!used_.contains(pattern.name())) {
        explicit_names_.push_back(pattern.name());
        used_[pattern.name()] = false;
      }
    }
  }
}

LabelMatcher::MatchType LabelMatcher::matches(const std::string& name) {
  if (auto it = used_.find(name); it != used_.end()) {
    it->second = true;  // Mark as used.
    return EXACT;
  }
  return globbed_ ? GLOB : NONE;
}

Result<std::vector<std::string>> LabelMatcher::explicit_target_names() {
  if (globbed_) {
    return Err(Location(),
               "Explicit target label required (wildcard patterns are not "
               "supported for this command).");
  }
  for (const auto& name : explicit_names_) {
    used_[name] = true;
  }
  return explicit_names_;
}

Err LabelMatcher::done() const {
  std::vector<std::string> unused;
  for (const auto& [name, used] : used_) {
    if (!used) {
      unused.push_back(name);
    }
  }
  if (!unused.empty()) {
    std::sort(unused.begin(), unused.end());
    std::string msg = "Target(s) not found: ";
    for (size_t i = 0; i < unused.size(); ++i) {
      if (i > 0)
        msg += ", ";
      msg += Label(source_dir_, unused[i]).GetUserVisibleName(false);
    }
    return Err(Location(), msg);
  }
  return Ok();
}

std::vector<TreeNode> TreeNode::assignments(
    std::initializer_list<std::string_view> attrs) const {
  return FindStatement<TreeNode>(
      node(), [attrs](TreeNode& node_ref) -> std::optional<TreeNode> {
        if (const auto* op = node_ref->AsBinaryOp()) {
          if (op->op().type() == Token::EQUAL ||
              op->op().type() == Token::PLUS_EQUALS ||
              op->op().type() == Token::MINUS_EQUALS) {
            if (const auto* left = op->left()->AsIdentifier()) {
              for (auto attr : attrs) {
                if (left->value().value() == attr) {
                  return node_ref;
                }
              }
            }
          }
        }
        return std::nullopt;
      });
}

std::vector<TreeNode> TreeNode::assignments(std::string_view attr) const {
  return assignments({attr});
}

std::vector<TreeNode> EditTarget::assignments(
    std::initializer_list<std::string_view> attrs) const {
  return node.Descend(block).assignments(attrs);
}

std::vector<TreeNode> EditTarget::assignments(std::string_view attr) const {
  return assignments({attr});
}

void EditTarget::add_warning(EditState& state, std::string_view message) const {
  std::string full_message = "Target \"" + label.GetUserVisibleName(false) +
                             "\" " + std::string(message);
  state.warnings.push_back(Err(node.node()->GetRange().begin(), full_message));
}

Result<BuildFile> BuildFile::Create(const BuildSettings* build_settings,
                                    const SourceFile& source_file,
                                    const std::vector<LabelPattern>& patterns) {
  auto input_file = std::make_unique<InputFile>(source_file);
  base::FilePath full_path = build_settings->GetFullPath(source_file);
  if (!input_file->Load(full_path)) {
    return Err(Location(), "Could not load file: " + source_file.value());
  }

  Err err;
  std::vector<Token> tokens = Tokenizer::Tokenize(input_file.get(), &err);
  RETURN_IF_ERROR(err);

  std::unique_ptr<ParseNode> tree_root = Parser::Parse(tokens, &err);
  RETURN_IF_ERROR(err);

  LabelMatcher label_matcher(source_file.GetDir(), patterns);
  return BuildFile(build_settings, source_file, std::move(input_file),
                   std::move(tree_root), std::move(label_matcher));
}

Location BuildFile::location() const {
  return Location(input_file_.get(), 1, 1);
}

std::vector<EditTarget> BuildFile::targets(
    std::function<bool(EditTarget&)> filter) {
  if (!filter) {
    filter = [this](EditTarget& t) {
      switch (label_matcher_.matches(t.label.name())) {
        case LabelMatcher::NONE:
          return false;
        case LabelMatcher::EXACT:
          return true;
        case LabelMatcher::GLOB:
          t.is_explicit = false;
          return true;
      }
      NOTREACHED();
    };
  }
  return FindStatement<EditTarget>(
      tree_root_.get(),
      [this, &filter](TreeNode& node_ref) -> std::optional<EditTarget> {
        if (auto* func = node_ref->AsFunctionCallMut()) {
          if (func->block() && func->args() &&
              func->args()->contents().size() == 1) {
            if (auto name =
                    AsStringLiteral(func->args()->contents()[0].get())) {
              EditTarget target{
                  .is_explicit = true,
                  .label = Label(source_file_.GetDir(), *name),
                  .node = node_ref,
                  .block = func->block(),
              };
              if (filter(target)) {
                return target;
              }
            }
          }
        }
        return std::nullopt;
      });
}

std::optional<EditTarget> BuildFile::find_target(std::string_view target_name) {
  auto found = targets([target_name](const EditTarget& t) {
    return t.label.name() == target_name;
  });
  if (!found.empty()) {
    return std::move(found.front());
  }
  return std::nullopt;
}

Result<std::unique_ptr<ParseNode>> BuildFile::parse_expression(
    std::string_view expr_string) {
  auto file = std::make_unique<InputFile>(SourceFile("//dummy"));
  file->SetContents(std::string(expr_string));

  Err err;
  std::vector<Token> tokens = Tokenizer::Tokenize(file.get(), &err);
  RETURN_IF_ERROR(err);
  for (auto& token : tokens) {
    token.set_location(this->location());
  }
  auto parsed = Parser::ParseExpression(tokens, &err);
  RETURN_IF_ERROR(err);
  extra_files_.push_back(std::move(file));
  return std::move(parsed);
}

std::unique_ptr<ParseNode> BuildFile::to_node(const Value& value) {
  auto parsed = parse_expression(value.ToString(true));
  // value.ToString() must return something parsable as input to GN.
  DCHECK(!parsed.has_error());
  return std::move(*parsed);
}

std::unique_ptr<IdentifierNode> BuildFile::create_identifier(
    std::string_view value) {
  StringAtom atom(value);
  return std::make_unique<IdentifierNode>(
      Token(location(), Token::IDENTIFIER, atom.str()));
}

std::unique_ptr<BinaryOpNode> BuildFile::create_assignment(
    std::string_view name,
    std::unique_ptr<ParseNode> value,
    Location loc) {
  StringAtom atom(name);
  auto left = std::make_unique<IdentifierNode>(
      Token(loc.is_null() ? location() : loc, Token::IDENTIFIER, atom.str()));

  auto assign = std::make_unique<BinaryOpNode>();
  assign->set_op(Token(location(), Token::EQUAL, "="));
  assign->set_left(std::move(left));
  assign->set_right(std::move(value));

  return assign;
}

void BuildFile::assign_in_block(
    BlockNode* block,
    std::vector<std::unique_ptr<ParseNode>>::const_iterator it,
    std::string_view name,
    std::unique_ptr<ParseNode> value) {
  CHECK(block);
  Location loc;
  // If GN sees:
  // a = 1 (line 10)
  // b = 2 (line 1 - defaulted)
  // c = 3 (line 11)
  // The formatter will decide to insert a blank line between b and c because
  // there's a gap of more than one line. Thus, we attach it up to an adjacent
  // element to prevent blank lines being inserted.
  if (block->statements().empty()) {
    loc = block->GetRange().begin();
  } else if (it == block->statements().end()) {
    loc = block->statements().back()->GetRange().end();
  } else {
    loc = (*it)->GetRange().begin();
  }
  block->statements().insert(it,
                             create_assignment(name, std::move(value), loc));
}

void BuildFile::assign_in_block(BlockNode* block,
                                std::string_view name,
                                std::unique_ptr<ParseNode> value) {
  CHECK(block);
  auto target_order = GetAttributeOrder(name);
  auto it = block->statements().end();
  if (target_order) {
    auto orders = GetOrders(block->statements());
    for (size_t i = 0; i < orders.size(); ++i) {
      if (orders[i] && *orders[i] >= *target_order) {
        it = block->statements().begin() + i;
        break;
      }
    }
  }
  assign_in_block(block, it, name, std::move(value));
}

std::unique_ptr<BlockNode> BuildFile::create_block(
    std::vector<std::unique_ptr<ParseNode>> statements) {
  auto block = std::make_unique<BlockNode>(BlockNode::DISCARDS_RESULT);
  block->set_begin_token(Token(location(), Token::LEFT_BRACE, "{"));
  block->set_end(
      std::make_unique<EndNode>(Token(location(), Token::RIGHT_BRACE, "}")));
  block->statements() = std::move(statements);
  return block;
}

std::unique_ptr<FunctionCallNode> BuildFile::create_target(
    std::string_view type,
    std::string_view name,
    std::unique_ptr<BlockNode> block) {
  auto func = std::make_unique<FunctionCallNode>();
  func->set_function(
      Token(location(), Token::IDENTIFIER, StringAtom(type).str()));

  auto args = std::make_unique<ListNode>();
  args->set_begin_token(Token(location(), Token::LEFT_PAREN, "("));
  args->set_end(
      std::make_unique<EndNode>(Token(location(), Token::RIGHT_PAREN, ")")));
  args->append_item(to_node(Value(nullptr, std::string(name))));
  func->set_args(std::move(args));

  func->set_block(std::move(block));
  return func;
}

Result<bool> BuildFile::Write() {
  ASSIGN_OR_RETURN(std::string formatted, commands::FormatNodeToString(root()));
  if (input_file_->contents() == formatted) {
    return false;
  }
  base::FilePath file_path = build_settings_->GetFullPath(source_file());
  if (base::WriteFile(file_path, formatted.data(),
                      static_cast<int>(formatted.size())) == -1) {
    return Err(Location(),
               "Failed to write to file: " + FilePathToUTF8(file_path));
  }
  return true;
}

BuildFile::BuildFile(const BuildSettings* build_settings,
                     SourceFile source_file,
                     std::unique_ptr<InputFile> input_file,
                     std::unique_ptr<ParseNode> tree_root,
                     LabelMatcher label_matcher)
    : build_settings_(build_settings),
      source_file_(std::move(source_file)),
      input_file_(std::move(input_file)),
      tree_root_(std::move(tree_root)),
      label_matcher_(std::move(label_matcher)) {}

Result<std::vector<BuildFile>> ResolvePatternsToBuildFiles(
    const BuildSettings* build_settings,
    const Loader* loader,
    const std::vector<LabelPattern>& patterns) {
  std::set<SourceFile> seen;
  std::vector<BuildFile> result;

  for (const LabelPattern& pattern : patterns) {
    ASSIGN_OR_RETURN(auto files,
                     ResolvePatternToFiles(build_settings, loader, pattern));

    for (const SourceFile& file : files) {
      if (seen.insert(file).second) {
        ASSIGN_OR_RETURN(auto parsed,
                         BuildFile::Create(build_settings, file, patterns));
        result.push_back(std::move(parsed));
      }
    }
  }
  return result;
}
