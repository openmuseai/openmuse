// Copyright (c) 2013 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_GN_ERR_H_
#define TOOLS_GN_ERR_H_

#include <expected>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "gn/label.h"
#include "gn/location.h"
#include "gn/token.h"

class ParseNode;
class Value;
struct ErrOutput;

// Result of doing some operation. Check has_error() to see if an error
// occurred.
//
// An error has a location and a message. Below that, is some optional help
// text to go with the annotation of the location.
//
// An error can also have sub-errors which are additionally printed out
// below. They can provide additional context.
class Err {
 private:
  struct ErrInfo {
    ErrInfo(const Location& loc,
            const std::string& msg,
            const std::string& help)
        : location(loc), message(msg), help_text(help) {}

    Location location;
    Label toolchain_label;

    std::vector<LocationRange> ranges;

    std::string message;
    std::string help_text;

    std::vector<Err> sub_errs;
  };

 public:
  using RangeList = std::vector<LocationRange>;

  // Indicates no error.
  Err() = default;

  // Error at a single point.
  Err(const Location& location,
      const std::string& msg,
      const std::string& help = std::string());

  // Error at a given range.
  Err(const LocationRange& range,
      const std::string& msg,
      const std::string& help = std::string());

  // Error at a given token.
  Err(const Token& token,
      const std::string& msg,
      const std::string& help_text = std::string());

  // Error at a given node.
  Err(const ParseNode* node,
      const std::string& msg,
      const std::string& help_text = std::string());

  // Error at a given value.
  Err(const Value& value,
      const std::string& msg,
      const std::string& help_text = std::string());

  Err(const Err& other);
  Err(Err&& other) = default;

  Err& operator=(const Err&);
  Err& operator=(Err&&) = default;

  bool has_error() const { return !!info_; }

  // All getters and setters below require has_error() returns true.
  const Location& location() const { return info_->location; }
  const std::string& message() const { return info_->message; }
  const std::string& help_text() const { return info_->help_text; }

  void AppendRange(const LocationRange& range) {
    info_->ranges.push_back(range);
  }
  const RangeList& ranges() const { return info_->ranges; }

  void set_toolchain_label(const Label& toolchain_label) {
    info_->toolchain_label = toolchain_label;
  }

  void AppendSubErr(const Err& err);

  // Prints the error to standard out.
  // Returns true if the error was actually printed, or false if it was
  // suppressed (e.g., due to reaching the error limit). Callers can use this
  // return value to determine whether to print additional formatting like
  // newlines or separators.
  bool PrintToStdout() const;

  // Converts the error to a string, formatted as it would be if calling
  // PrintToStdout with text decoration disabled.
  std::string to_string() const;

  // Prints to standard out but uses a "WARNING" messaging instead of the
  // normal "ERROR" messaging. This is a property of the printing system rather
  // than of the Err class because there is no expectation that code calling a
  // function that take an Err check that the error is nonfatal and continue.
  // Generally all Err objects with has_error() set are fatal.
  //
  // In some very specific cases code will detect a condition and print a
  // nonfatal error to the screen instead of returning it. In these cases, that
  // code can decide at printing time whether it will continue (and use this
  // method) or not (and use PrintToStdout()).
  //
  // Returns true if the warning was actually printed, or false if suppressed.
  bool PrintNonfatalToStdout() const;

 private:
  bool InternalPrintToStdout(bool is_sub_err, bool is_fatal) const;
  void InternalFormat(ErrOutput& output, bool is_sub_err, bool is_fatal) const;

  std::unique_ptr<ErrInfo> info_;  // Non-null indicates error.
};

// "return Ok()" is far more clear than "return Err()"
inline Err Ok() {
  return Err();
}

// A wrapper around std::expected<T, Err>.
// std::expected does not allow implicit conversions because you can have
// std::expected<T, T>. Since that isn't a problem for us, we add the
// implicit conversions.
template <typename T>
class Result : public std::expected<T, Err> {
  static_assert(!std::is_void_v<T>, "Use Err instead of Result<void>");
  static_assert(!std::is_same_v<T, Err>, "Err cannot be the success case");

 public:
  using std::expected<T, Err>::expected;

  // Implicit conversion from Err (always creates an error state)
  Result(Err err) : std::expected<T, Err>(std::unexpect, std::move(err)) {
    // Cannot create an error result that's actually an "Ok".
    DCHECK(this->error().has_error());
  }

  // Implicit conversion from T (always creates success state)
  Result(const T& val) : std::expected<T, Err>(val) {}
  Result(T&& val) : std::expected<T, Err>(std::move(val)) {}

  // We implement transform_error here because while it is in the C++23 spec
  // (which we use), the older macOS SDK used in CI doesn't support it.
  template <typename F>
  constexpr Result<T> transform_error(F&& f) const& {
    if (this->has_value()) {
      return Result<T>(**this);
    }
    return Result<T>(std::forward<F>(f)(this->error()));
  }

  template <typename F>
  constexpr Result<T> transform_error(F&& f) && {
    if (this->has_value()) {
      return Result<T>(std::move(**this));
    }
    return Result<T>(std::forward<F>(f)(std::move(this->error())));
  }

  // Implementation of has_error for compatibility with Err type.
  bool has_error() const { return !this->has_value(); }

  // Implementation of message for compatibility with Err type.
  const std::string& message() const {
    DCHECK(has_error());
    return this->error().message();
  }
};

namespace internal {

inline Err GetError(const Err& err) {
  return err;
}

inline Err GetError(Err&& err) {
  return std::move(err);
}

template <typename T, typename E>
inline E GetError(const std::expected<T, E>& exp) {
  return exp.error();
}

template <typename T, typename E>
inline E GetError(std::expected<T, E>&& exp) {
  return std::move(exp).error();
}

}  // namespace internal

// We will define the following terms:
// A "new-style" function is one which returns either Result<T> or Err (which is
// treated as Result<void>).
// A "legacy" function takes an Err* pointer as a parameter and returns a value.

#define _STATUS_CONCAT_INNER(a, b) a##b
#define _STATUS_CONCAT(a, b) _STATUS_CONCAT_INNER(a, b)

// Usage: In a new-style function, call
//   ASSIGN_OR_RETURN(auto foo, new_style_function())
#define ASSIGN_OR_RETURN(lhs, rexpr)                                        \
  _ASSIGN_OR_RETURN_IMPL(_STATUS_CONCAT(_expected_value, __COUNTER__), lhs, \
                         rexpr)

#define _ASSIGN_OR_RETURN_IMPL(expected_val, lhs, rexpr) \
  auto expected_val = (rexpr);                           \
  if (!expected_val) {                                   \
    return std::move(expected_val).error();              \
  }                                                      \
  lhs = std::move(*expected_val)

// Usage: In a new-style function, call
//   RETURN_IF_ERROR(new_style_function())
#define RETURN_IF_ERROR(expr) \
  _RETURN_IF_ERROR_IMPL(_STATUS_CONCAT(_status_value, __COUNTER__), expr)

#define _RETURN_IF_ERROR_IMPL(status_val, expr)         \
  do {                                                  \
    auto status_val = (expr);                           \
    if (status_val.has_error()) {                       \
      return internal::GetError(std::move(status_val)); \
    }                                                   \
  } while (0)

// Usage: In a legacy function returning void, call
//   ASSIGN_OR_RETURN_VOID(foo, err_ptr, new_style_function())
#define ASSIGN_OR_RETURN_VOID(lhs, err_ptr, rexpr)                          \
  _ASSIGN_OR_RETURN_VOID_IMPL(_STATUS_CONCAT(_expected_value, __COUNTER__), \
                              lhs, err_ptr, rexpr)

#define _ASSIGN_OR_RETURN_VOID_IMPL(expected_val, lhs, err_ptr, rexpr) \
  auto expected_val = (rexpr);                                         \
  if (!expected_val) {                                                 \
    *(err_ptr) = std::move(expected_val).error();                      \
    return;                                                            \
  }                                                                    \
  lhs = std::move(*expected_val)

// Usage: In a legacy function returning void, call
//   RETURN_IF_ERROR_VOID(err_ptr, new_style_function())
#define RETURN_IF_ERROR_VOID(err_ptr, expr)                              \
  _RETURN_IF_ERROR_VOID_IMPL(_STATUS_CONCAT(_status_value, __COUNTER__), \
                             err_ptr, expr)

#define _RETURN_IF_ERROR_VOID_IMPL(status_val, err_ptr, expr) \
  do {                                                        \
    auto status_val = (expr);                                 \
    if (status_val.has_error()) {                             \
      *(err_ptr) = internal::GetError(std::move(status_val)); \
      return;                                                 \
    }                                                         \
  } while (0)

// Usage: In a legacy function returning a pointer or bool, call
//   ASSIGN_OR_RETURN_PTR(foo, err_ptr, new_style_function())
#define ASSIGN_OR_RETURN_PTR(lhs, err_ptr, rexpr)                          \
  _ASSIGN_OR_RETURN_PTR_IMPL(_STATUS_CONCAT(_expected_value, __COUNTER__), \
                             lhs, err_ptr, rexpr)

#define _ASSIGN_OR_RETURN_PTR_IMPL(expected_val, lhs, err_ptr, rexpr) \
  auto expected_val = (rexpr);                                        \
  if (!expected_val) {                                                \
    *(err_ptr) = std::move(expected_val).error();                     \
    return {};                                                        \
  }                                                                   \
  lhs = std::move(*expected_val)

// Usage: In a legacy function returning a pointer or bool, call
//   RETURN_IF_ERROR_PTR(err_ptr, new_style_function())
#define RETURN_IF_ERROR_PTR(err_ptr, expr)                              \
  _RETURN_IF_ERROR_PTR_IMPL(_STATUS_CONCAT(_status_value, __COUNTER__), \
                            err_ptr, expr)

#define _RETURN_IF_ERROR_PTR_IMPL(status_val, err_ptr, expr)  \
  do {                                                        \
    auto status_val = (expr);                                 \
    if (status_val.has_error()) {                             \
      *(err_ptr) = internal::GetError(std::move(status_val)); \
      return {};                                              \
    }                                                         \
  } while (0)

#endif  // TOOLS_GN_ERR_H_
