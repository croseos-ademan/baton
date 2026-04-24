use crate::error::BatonError;
use crate::lifecycle::LifecyclePhase;
use std::collections::HashMap;

type HookFn = Box<dyn Fn() -> Result<(), BatonError> + Send + Sync>;

pub struct LifecycleHook {
    hooks: HashMap<String, Vec<HookFn>>,
}

impl LifecycleHook {
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, phase: &LifecyclePhase, f: F)
    where
        F: Fn() -> Result<(), BatonError> + Send + Sync + 'static,
    {
        let key = format!("{:?}", phase);
        self.hooks.entry(key).or_default().push(Box::new(f));
    }

    pub fn run(&self, phase: &LifecyclePhase) -> Result<(), BatonError> {
        let key = format!("{:?}", phase);
        if let Some(fns) = self.hooks.get(&key) {
            for f in fns {
                f()?;
            }
        }
        Ok(())
    }

    pub fn hook_count(&self, phase: &LifecyclePhase) -> usize {
        let key = format!("{:?}", phase);
        self.hooks.get(&key).map(|v| v.len()).unwrap_or(0)
    }
}

impl Default for LifecycleHook {
    fn default() -> Self {
        Self::new()
    }
}
