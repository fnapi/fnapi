use std::{
    fmt::{self, Debug, Formatter},
    hash::BuildHasherDefault,
    path::Path,
    sync::Arc,
};

use anyhow::{Context, Result};
use dashmap::DashMap;
use rustc_hash::FxHasher;
use swc_common::{FileName, Mark, SourceMap};
use swc_ecma_transforms_base::resolver;
use swc_ecmascript::{
    ast::{EsVersion, Module},
    parser::{parse_file_as_module, Syntax, TsConfig},
    visit::VisitMutWith,
};

/// Storage for parsed modules.
///
/// Allocates `top_level_mark` for each modules on addition.
pub struct Modules {
    cm: Arc<SourceMap>,

    pub unresolved_mark: Mark,

    metadata: DashMap<Arc<FileName>, ModuleMetadata, BuildHasherDefault<FxHasher>>,

    data: DashMap<Arc<FileName>, ModuleData, BuildHasherDefault<FxHasher>>,
}

#[derive(Debug)]
struct ModuleMetadata {
    top_level_mark: Mark,
}

impl Default for ModuleMetadata {
    fn default() -> Self {
        Self {
            top_level_mark: Mark::fresh(Mark::root()),
        }
    }
}

#[derive(Debug)]
struct ModuleData {
    content: Arc<Module>,
}

impl Debug for Modules {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Modules").finish()
    }
}

impl Modules {
    pub fn new(cm: Arc<SourceMap>) -> Arc<Self> {
        Arc::new(Self {
            cm,
            unresolved_mark: Mark::new(),
            metadata: Default::default(),
            data: Default::default(),
        })
    }

    pub fn get_top_level_mark_for(&self, path: Arc<FileName>) -> Mark {
        self.metadata.entry(path).or_default().top_level_mark
    }

    pub fn get(&self, key: Arc<FileName>) -> Option<Arc<Module>> {
        self.data.get(&key).map(|v| v.content.clone())
    }

    pub fn load(&self, path: &Path) -> Result<Arc<Module>> {
        let fm = self
            .cm
            .load_file(path)
            .with_context(|| format!("failed to load file `{}`", path.display()))?;

        let key = Arc::new(fm.name.clone());
        let unresolved_mark = self.unresolved_mark;
        let top_level_mark = self.get_top_level_mark_for(key.clone());

        let e = self.data.entry(key).or_try_insert_with(|| -> Result<_> {
            let tsx = path.to_string_lossy().ends_with(".tsx");
            let dts = path.to_string_lossy().ends_with(".d.ts");
            let mut m = parse_file_as_module(
                &fm,
                Syntax::Typescript(TsConfig {
                    tsx,
                    dts,
                    decorators: true,
                    ..Default::default()
                }),
                EsVersion::latest(),
                None,
                &mut vec![],
            )
            .map_err(|err| anyhow::anyhow!("failed to parse `{}`: {:?}", path.display(), err))?;

            m.visit_mut_with(&mut resolver(unresolved_mark, top_level_mark, true));

            Ok(ModuleData {
                content: Arc::new(m),
            })
        })?;

        Ok(e.content.clone())
    }
}
