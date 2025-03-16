use std::io::Cursor;
use std::path::Path;

use anyhow::{anyhow, Context };

use hound::{SampleFormat, WavReader};

use whisper_rs::convert_integer_to_float_audio;
use whisper_rs::convert_stereo_to_mono_audio;

pub fn parse_wav(data: Vec<u8>) -> anyhow::Result<Vec<f32>> {
    let cursor = Cursor::new(data);

    let reader = WavReader::new(cursor)?;

    let hound::WavSpec {
        channels,
        sample_format,
        sample_rate,
        bits_per_sample,
        ..
    } = reader.spec();

    let original_samples: Vec<i16> = reader
        .into_samples::<i16>()
        .into_iter()
        .map(|x| x.context("Failed to read sample"))
        .collect::<anyhow::Result<Vec<_>>>()?;

    // Convert the audio to floating point samples.
    let mut samples = vec![0.0f32; original_samples.len()];
    convert_integer_to_float_audio(&original_samples, &mut samples)
        .map_err(|_| anyhow!("Failed to convert samples"))?;


    // Convert audio to 16KHz mono f32 samples, as required by the model.
    // These utilities are provided for convenience, but can be replaced with custom conversion logic.
    // SIMD variants of these functions are also available on nightly Rust (see the docs).
    if channels == 2 {
        samples =
            convert_stereo_to_mono_audio(&samples).map_err(|_| anyhow!("Conversion error"))?;
    } else if channels != 1 {
        return Err(anyhow!(">2 channels unsupported"));
    }

    if sample_format != SampleFormat::Int {
        return Err(anyhow!("Expected integer sample format"));
    }

    if sample_rate != 16000 {
        return Err(anyhow!("expected 16KHz sample rate"));
    }
    if bits_per_sample != 16 {
        return Err(anyhow!("expected 16 bits per sample"));
    }

    Ok(samples)
}

pub fn parse_wav_file(path: &Path) -> anyhow::Result<Vec<f32>> {
    let data = std::fs::read(path)?;

    parse_wav(data)
}