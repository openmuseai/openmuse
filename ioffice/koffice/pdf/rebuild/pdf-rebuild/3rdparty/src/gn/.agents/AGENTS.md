# Rust Style Guidelines & Best Practices

Please adhere to the following conventions and best practices when writing or
modifying Rust code in this repository:

## Code Structure & Formatting

* **Code Ordering**: Be consistent about the layout within a file.
  * The structure should generally be:
    1. `mod` statements
    2. `use` statements
    3. Core code (structs, enums, impls, functions)
    4. Test module at the bottom: `#[cfg(test)]`
  * Structs should always come immediately before their impl and impl traits.
  * Full types should be defined before reference types (e.g. if we were
    writing the standard library, String before str)
* **Inlining**: If you only use a variable once, prefer inlining it rather
  than assigning it to a separate binding.
* **Collect**: Prefer using `.collect()` over manually iterating and
  adding/pushing to data structures when possible.
* **Unused Variables**: Variables starting with `_` (e.g. `_var`) should only
  be used if they are truly never used. If you begin using a variable that was
  prefixed with an underscore, rename it to remove the underscore prefix.
* **Unsafe Code**: All `unsafe` blocks must be prefixed with a
  `// Safety: <reason>` comment explaining why the usage is safe.
* **Warnings**: The code (including the tests) should compile with no warnings
  and no clippy lint errors.
* If you ever believe that #[allow] is required to bypass the linter:
  * You *must* get permission from the user.
  * You *must* write a comment above the allow saying why we need it.
  * You *must* put it in as specific a place as is feasible. Avoid, for
    example, allowing a lint for a whole file.

## Imports & Use Statements

* **Test-only Imports**: Imports that are only used inside tests should be
  moved into a module block guarded by `#[cfg(test)]`.

## Documentation

* Anything marked as pub should have a docstring (starts with `///`).

## Coding guidelines

* Data members should not be public without good reason
* **FFI Boundary**: No FFI function calls (such as `extern "C"` blocks or raw
  FFI calls) are allowed outside of the `ffi` submodule. The only exception is
  `intern_string`.
* **No Statics**: Do not use global static variables or `thread_local!`
  structures. They hinder concurrency, unit testing isolation, and
  multi-session safety. Always prefer passing state and context explicitly via
  arguments or trait objects.
* **Dynamic dispatch**: Never use dynamic dispatch (`dyn Trait`) in the hot
  path (any code that could be called an arbitrary number of times by a
  starlark rule).
  * Avoid using dynamic dispatch (`dyn Trait`) in general if you can, but
    you may use it to increase testability of code if it is not in the hot
    path.
    * Eg. Using `dyn Trait` to support a "real" version in production code and
      a fake one in test code.
* `collect_repr` and `collect_str`
  * collect_repr should be defined on starlark types iff `Debug` is
    available on a type. It should just `write!(collector, "{:?}").unwrap()`
  * collect_str should be defined on starlark types iff `Display` is
    available on a type. It should just `write!(collector, "{}").unwrap()`
* **Error Handling**: Prefer using custom error enums with `thiserror` for
  crate-level errors, rather than generic strings or custom hand-written
  Display implementations. Let exceptions and failures bubble up naturally.
* **Crate Encapsulation**: Keep modules, structs, and fields private or
  `pub(crate)` by default to maintain clean boundaries. Only use `pub` for APIs
  that are intended to be consumed by other crates.

# C++ Style guidelines and best practices

* Never use `#pragma once` - use header guards instead.
