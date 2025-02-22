use rand::Rng;

#[cfg(feature = "v1")]
use crate::llama_cpp_2::sampling::params::LlamaSamplerChainParams;

use crate::llama_cpp_2::sampling::LlamaSampler;

use crate::InferencingParams;

#[cfg(feature = "v1")]
pub fn generate_sampler(
    params: &InferencingParams,
    n_vocab: i32,
    special_eos_id: i32,
    linefeed_id: i32,
) -> anyhow::Result<LlamaSampler> {
    let sampler_params = LlamaSamplerChainParams::default();

    let mut sampler = LlamaSampler::new(sampler_params)?;

    if let Some(temperature) = params.temperature {
        sampler = sampler.add_temp(temperature);
    }

    if let Some(top_k) = params.top_k {
        sampler = sampler.add_top_k(top_k);
    }

    if let Some(top_p) = params.top_p {
        let min_keep = params.min_keep.unwrap_or(1) as usize;

        sampler = sampler.add_top_p(top_p, min_keep);
    }

    if let Some(penalty_last_n) = params.penalty_last_n {
        let penalty_repeat = params.penalty_repeat.unwrap_or(1.);
        let penalty_freq = params.penalty_repeat.unwrap_or(0.);
        let penalty_present = params.penalty_repeat.unwrap_or(0.);

        sampler = sampler.add_penalties(
            n_vocab,
            special_eos_id,
            linefeed_id,
            penalty_last_n,
            penalty_repeat,
            penalty_freq,
            penalty_present,
            false,
            true,
        );
    }

    let mut rng = rand::thread_rng();
    let seed = params.seed.unwrap_or(rng.gen_range(u32::MIN..u32::MAX));
    sampler = sampler.add_dist(seed);

    sampler = sampler.add_greedy();

    Ok(sampler)
}

#[cfg(feature = "v2")]
pub fn generate_sampler(
    params: &InferencingParams,
    _n_vocab: i32,
    _special_eos_id: i32,
    _linefeed_id: i32,
) -> anyhow::Result<LlamaSampler> {
    let mut samplers: Vec<LlamaSampler> = Vec::new();

    if let Some(temperature) = params.temperature {
        samplers.push(LlamaSampler::temp(temperature));
    }

    if let Some(top_k) = params.top_k {
        samplers.push(LlamaSampler::top_k(top_k));
    }

    if let Some(top_p) = params.top_p {
        let min_keep = params.min_keep.unwrap_or(1) as usize;
        samplers.push(LlamaSampler::top_p(top_p, min_keep));
    }

    if let Some(penalty_last_n) = params.penalty_last_n {
        let penalty_repeat = params.penalty_repeat.unwrap_or(1.);
        let penalty_freq = params.penalty_repeat.unwrap_or(0.);
        let penalty_present = params.penalty_repeat.unwrap_or(0.);
        samplers.push(LlamaSampler::penalties(
            penalty_last_n,
            penalty_repeat,
            penalty_freq,
            penalty_present,
        ));
    }

    let mut rng = rand::thread_rng();
    let seed = params.seed.unwrap_or(rng.gen_range(u32::MIN..u32::MAX));
    samplers.push(LlamaSampler::dist(seed));

    samplers.push(LlamaSampler::greedy());

    let sampler = LlamaSampler::chain_simple(samplers);

    Ok(sampler)
}
