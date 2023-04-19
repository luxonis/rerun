use re_log_types::{
    component_types::{Tensor, TensorId, TensorImageLoadError},
    DecodedTensor,
};

// ----------------------------------------------------------------------------

struct DecodedTensorResult {
    /// Cached `Result` from decoding the `Tensor`
    tensor_result: Result<DecodedTensor, TensorImageLoadError>,

    /// Total memory used by this `Tensor`.
    memory_used: u64,

    /// Which [`DecodeCache::generation`] was this `Tensor` last used?
    last_use_generation: u64,
}

/// A cache of decoded [`Tensor`] entities, indexed by `TensorId`.
#[derive(Default)]
pub struct DecodeCache {
    cache: nohash_hasher::IntMap<TensorId, DecodedTensorResult>,
    memory_used: u64,
    generation: u64,
}

#[allow(clippy::map_err_ignore)]
impl DecodeCache {
    /// Decode a [`Tensor`] if necessary and cache the result.
    ///
    /// This is a no-op for Tensors that are not compressed.
    ///
    /// Currently supports JPEG encoded tensors.
    pub fn try_decode_tensor_if_necessary(
        &mut self,
        maybe_encoded_tensor: Tensor,
    ) -> Result<DecodedTensor, TensorImageLoadError> {
        crate::profile_function!();

        match DecodedTensor::try_from(maybe_encoded_tensor) {
            Ok(decoded_tensor) => Ok(decoded_tensor),

            Err(encoded_tensor) => {
                let lookup = self.cache.entry(encoded_tensor.id()).or_insert_with(|| {
                    let tensor_result = DecodedTensor::try_decode(encoded_tensor);
                    let memory_used = match &tensor_result {
                        Ok(tensor) => tensor.size_in_bytes() as u64,
                        Err(_) => 0,
                    };
                    self.memory_used += memory_used;
                    let last_use_generation = 0;
                    DecodedTensorResult {
                        tensor_result,
                        memory_used,
                        last_use_generation,
                    }
                });
                lookup.last_use_generation = self.generation;
                lookup.tensor_result.clone()
            }
        }
    }

    /// Call once per frame to (potentially) flush the cache.
    pub fn begin_frame(&mut self, max_memory_use: u64) {
        // TODO(jleibs): a more incremental purging mechanism, maybe switching to an LRU Cache
        // would likely improve the behavior.

        if self.memory_used > max_memory_use {
            self.purge_memory();
        }

        self.generation += 1;
    }

    /// Attempt to free up memory.
    pub fn purge_memory(&mut self) {
        crate::profile_function!();

        // Very aggressively flush everything not used in this frame

        let before = self.memory_used;

        self.cache.retain(|_, ci| {
            let retain = ci.last_use_generation == self.generation;
            if !retain {
                self.memory_used -= ci.memory_used;
            }
            retain
        });

        re_log::debug!(
            "Flushed tensor decode cache. Before: {:.2} GB. After: {:.2} GB",
            before as f64 / 1e9,
            self.memory_used as f64 / 1e9,
        );
    }
}
