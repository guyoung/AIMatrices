use rand::Rng;

use llama_cpp_2::sampling::LlamaSampler;

use crate::InferencingParams;

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
