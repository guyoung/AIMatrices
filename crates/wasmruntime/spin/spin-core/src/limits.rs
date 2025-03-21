use anyhow::Result;
use async_trait::async_trait;
use wasmtime::ResourceLimiterAsync;

/// Async implementation of wasmtime's `StoreLimits`: https://github.com/bytecodealliance/wasmtime/blob/main/crates/wasmtime/src/limits.rs
/// Used to limit the memory use and spin-resource-table size of each Instance
#[derive(Default)]
pub struct StoreLimitsAsync {
    max_memory_size: Option<usize>,
    max_table_elements: Option<u32>,
    memory_consumed: u64,
}

#[async_trait]
impl ResourceLimiterAsync for StoreLimitsAsync {
    async fn memory_growing(
        &mut self,
        current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> Result<bool> {
        let can_grow = if let Some(limit) = self.max_memory_size {
            desired <= limit
        } else {
            true
        };
        if can_grow {
            self.memory_consumed =
                (self.memory_consumed as i64 + (desired as i64 - current as i64)) as u64;
        }
        Ok(can_grow)
    }

    async fn table_growing(
        &mut self,
        _current: u32,
        desired: u32,
        _maximum: Option<u32>,
    ) -> Result<bool> {
        let can_grow = if let Some(limit) = self.max_table_elements {
            desired <= limit
        } else {
            true
        };
        Ok(can_grow)
    }
}

impl StoreLimitsAsync {
    pub fn new(max_memory_size: Option<usize>, max_table_elements: Option<u32>) -> Self {
        Self {
            max_memory_size,
            max_table_elements,
            memory_consumed: 0,
        }
    }

    /// How much memory has been consumed in bytes
    pub fn memory_consumed(&self) -> u64 {
        self.memory_consumed
    }
}
