// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
    pin::Pin,
    sync::{Arc, Condvar, Mutex, RwLock},
};

use starlark::{
    collections::SmallMap,
    environment::{FrozenModule, Globals, Module},
    eval::{Evaluator, FileLoader as StarlarkFileLoader},
    syntax::{AstModule, Dialect},
    values::FrozenHeapName,
};
use types::{EvalContext, EvaluatorContextExt as _, LabelRef, PackageRef, PathResolver};

use crate::Error;

/// The Starlark compilation dialect used for `.bzl` configuration files.
pub const BZL_FILE_DIALECT: Dialect = Dialect {
    enable_lambda: false,
    enable_load_reexport: false,
    ..Dialect::Standard
};

enum FileStatus {
    Loading {
        // A CondVar to wait on for the filestatus to resolve to Loaded.
        wait: Arc<Condvar>,
        // What needs to finish evaluating before this can be evaluated.
        // This is used purely for cycle detection.
        needs: Option<String>,
    },
    Loaded(starlark::Result<Pin<Box<FrozenModule>>>),
}

impl Default for FileStatus {
    fn default() -> Self {
        Self::Loading {
            wait: Arc::new(Condvar::new()),
            needs: None,
        }
    }
}

/// A thread-safe loader for reading, compiling, and caching Starlark modules
/// (`.bzl` files).
#[derive(Default)]
pub struct FileLoader {
    files: RwLock<HashMap<String, Arc<Mutex<FileStatus>>>>,
}

impl FileLoader {
    /// Preloads a frozen module into the loader's cache.
    /// This should be called once for each builtin module.
    /// By convention, builtin modules will be named "//builtins:$NAME.scl"
    pub fn preload(&self, module: FrozenModule) {
        let mut files = self.files.write().unwrap();
        files.insert(
            module.frozen_heap().name().unwrap().to_string(),
            Arc::new(Mutex::new(FileStatus::Loaded(Ok(Box::pin(module))))),
        );
    }

    fn wait_for_load(
        &self,
        file_status: &Arc<Mutex<FileStatus>>,
    ) -> starlark::Result<FrozenModule> {
        let mut status = file_status.lock().unwrap();
        while let FileStatus::Loading { wait, .. } = &*status {
            let wait = wait.clone();
            // Note: this temporarily releases the status mutex, so we don't
            // prevent other threads from accessing it.
            status = wait.wait(status).unwrap();
        }
        self.get_loaded(&status)
    }

    fn set_complete(
        &self,
        file_status: &Arc<Mutex<FileStatus>>,
        result: starlark::Result<FrozenModule>,
    ) -> starlark::Result<FrozenModule> {
        let mut status = file_status.lock().unwrap();
        if let FileStatus::Loading { wait, .. } = &*status {
            wait.notify_all();
        }
        *status = FileStatus::Loaded(result.map(Box::pin));
        self.get_loaded(&status)
    }

    /// Gets an already loaded file.
    fn get_loaded(&self, status: &FileStatus) -> starlark::Result<FrozenModule> {
        match status {
            FileStatus::Loading { .. } => unreachable!(),
            // Since `FrozenModule` is a cheap-to-clone handle around an Arc'd frozen heap,
            // we clone and return it by value instead of using unsafe pointer casts.
            FileStatus::Loaded(Ok(m)) => Ok(m.as_ref().get_ref().clone()),
            // starlark::Error doesn't implement Clone, so we do a poor man's clone.
            FileStatus::Loaded(Err(e)) => Err(starlark::Error::new_other(anyhow::anyhow!("{e}"))),
        }
    }

    /// Loads, parses, compiles, and evaluates a Starlark module, resolving
    /// dependencies recursively and caching the result.
    pub fn load<'b, C: EvalContext, F: Fn(&PackageRef) -> C>(
        &self,
        label: LabelRef<'b>,
        path_resolver: &PathResolver,
        globals: &Globals,
        make_eval_context: &F,
    ) -> starlark::Result<FrozenModule> {
        // Starlark-rs requires module identifiers to be strings.
        let label_str = label.to_string();
        let file_status = {
            let mut loader = self.files.write().unwrap();
            match loader.entry(label_str.clone()) {
                Entry::Occupied(entry) => {
                    let file_status = entry.get().clone();
                    drop(loader);
                    return self.wait_for_load(&file_status);
                },
                // We're about to start evaluating it.
                Entry::Vacant(entry) => entry.insert(Default::default()).clone(),
            }
        };

        let result = self.load_and_evaluate(
            label,
            &label_str,
            &file_status,
            path_resolver,
            globals,
            make_eval_context,
        );

        self.set_complete(&file_status, result)
    }

    fn load_and_evaluate<'b, C: EvalContext, F: Fn(&PackageRef) -> C>(
        &self,
        label: LabelRef<'b>,
        label_str: &str,
        file_status: &Arc<Mutex<FileStatus>>,
        path_resolver: &PathResolver,
        globals: &Globals,
        make_eval_context: &F,
    ) -> starlark::Result<FrozenModule> {
        // Read and parse the file to get its dependencies.
        let absolute_path = path_resolver.absolute_path(label.package(), label.name());
        let content = fs::read_to_string(&absolute_path)
            .map_err(|_| Error::ReadFailed(label_str.to_owned()))?;
        let ast = AstModule::parse(label_str, content, &BZL_FILE_DIALECT)?;

        let mut deps: Vec<(String, FrozenModule)> = Default::default();
        if !ast.loads().is_empty() {
            for load in ast.loads() {
                let dep_label = types::Label::parse(load.module_id, label.package())?;
                let dep_label_str = dep_label.to_string();
                if let Some(cycle) = self.find_cycle_path(file_status, &dep_label_str) {
                    return Err(Error::CycleDetected(cycle).into());
                }

                deps.push((
                    load.module_id.to_owned(),
                    self.load(
                        dep_label.as_ref(),
                        path_resolver,
                        globals,
                        make_eval_context,
                    )?,
                ));
            }
        }

        let deps_map: SmallMap<&str, &FrozenModule> =
            deps.iter().map(|(k, v)| (k.as_str(), v)).collect();

        let loader = PreloadedLoader { modules: &deps_map };
        Module::with_temp_heap(|module| {
            let extra = make_eval_context(label.package());
            {
                let mut eval = Evaluator::new(&module);
                eval.set_context(&extra);
                eval.set_loader(&loader);
                eval.eval_module(ast, globals)?;
            }
            Ok(module.freeze_named(FrozenHeapName::User(Box::new(label_str.to_owned())))?)
        })
    }

    fn find_cycle_path(
        &self,
        current: &Arc<Mutex<FileStatus>>,
        target: &str,
    ) -> Option<Vec<String>> {
        // Set the dependency before we start doing cycle detection.
        // This prevents multiple threads simultaneously calling find_cycle_path
        // not seeing a cycle on each other, then adding a dependency on each
        // other after the fact.
        {
            let mut status = current.lock().unwrap();
            let FileStatus::Loading { ref mut needs, .. } = &mut *status else {
                // find_cycle_path should only be called from the thing you're
                // currently trying to load.
                unreachable!();
            };
            *needs = Some(target.to_owned());
        }
        let loader = self.files.read().unwrap();
        let mut cur = target.to_owned();
        let mut cycle = vec![cur.clone()];
        while let Some(status_mutex) = loader.get(&cur) {
            let status = status_mutex.lock().unwrap();
            if let FileStatus::Loading {
                needs: Some(need), ..
            } = &*status
            {
                cycle.push(need.clone());
                cur = need.clone();
            } else {
                break;
            }
            if Arc::ptr_eq(current, status_mutex) {
                return Some(cycle);
            }
        }
        None
    }
}

/// Helper loader to load preloaded dependencies during evaluator execution.
struct PreloadedLoader<'a> {
    /// A map of pre-loaded Starlark modules indexed by their module path.
    modules: &'a SmallMap<&'a str, &'a FrozenModule>,
}

impl StarlarkFileLoader for PreloadedLoader<'_> {
    fn load(&self, path: &str) -> starlark::Result<FrozenModule> {
        match self.modules.get(path) {
            Some(m) => Ok((*m).clone()),
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use testutils::{FakeEvalContext, FakeSession};
    use types::Label;

    use super::*;

    fn make_eval_context(_pkg: &PackageRef) -> FakeEvalContext {
        FakeEvalContext::default_rule_impl(Rc::new(FakeSession::default()))
    }

    fn load(loader: &FileLoader, label_str: &str) -> starlark::Result<FrozenModule> {
        loader.load(
            Label::parse(label_str, PackageRef::root())
                .unwrap()
                .as_ref(),
            &PathResolver::new_for_testing(),
            &Globals::standard(),
            &make_eval_context,
        )
    }

    #[test]
    fn test_simple_load() {
        let loader = FileLoader::default();
        let module = load(&loader, "//load:absolute.scl").unwrap();
        assert_eq!(
            module.get("absolute").unwrap().unpack_str(),
            Some("absolute")
        );
    }

    #[test]
    fn test_transitive_load() {
        let loader = FileLoader::default();
        let module = load(&loader, "//load:root.scl").unwrap();
        assert_eq!(
            module.get("absolute_value").unwrap().unpack_str(),
            Some("absolute")
        );
        assert_eq!(
            module.get("relative_value").unwrap().unpack_str(),
            Some("relative")
        );
        assert_eq!(
            module
                .get("relative_as_absolute_value")
                .unwrap()
                .unpack_str(),
            Some("relative")
        );
        // Any symbol imported via load should be inaccessible to transitive users.
        assert!(module.get("absolute").is_err());
    }

    #[test]
    fn test_cycle_detection() {
        let loader = FileLoader::default();
        let err_msg = load(&loader, "//cycle:a.bzl").unwrap_err().to_string();
        assert!(err_msg.contains("cycle detected"));
    }

    #[test]
    fn test_load_error_caching() {
        let loader = FileLoader::default();
        let first_err = load(&loader, "//load:does_not_exist.scl").unwrap_err();
        let second_err = load(&loader, "//load:does_not_exist.scl").unwrap_err();

        assert!(first_err.to_string().contains("Failed to read"));
        assert!(second_err.to_string().contains("Failed to read"));
    }
}
