use crate::engine::parser;
use crate::error::{AudioError, Result};
use std::sync::Arc;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// Decode FLAC file specifically
pub fn flac(path: &str) -> Result<Arc<Vec<f32>>> {
    let file = std::fs::File::open(path)
        .map_err(|e| AudioError::FlacDecodeError(path.to_string(), e.to_string()))?;

    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("flac");

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| AudioError::FlacDecodeError(path.to_string(), e.to_string()))?;

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .ok_or_else(|| {
            AudioError::FlacDecodeError(path.to_string(), "No audio track found".to_string())
        })?;

    let track_id = track.id;
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| AudioError::FlacDecodeError(path.to_string(), e.to_string()))?;

    let mut samples: Vec<f32> = Vec::new();
    let mut sample_buf: Option<SampleBuffer<f32>> = None;

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(e) => return Err(AudioError::FlacDecodeError(path.to_string(), e.to_string())),
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = decoder
            .decode(&packet)
            .map_err(|e| AudioError::FlacDecodeError(path.to_string(), e.to_string()))?;

        let buf = sample_buf.get_or_insert_with(|| {
            SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec())
        });

        buf.copy_interleaved_ref(decoded);
        samples.extend_from_slice(buf.samples());
    }

    if samples.is_empty() {
        return Err(AudioError::FlacDecodeError(
            path.to_string(),
            "No audio data decoded".to_string(),
        ));
    }

    Ok(Arc::new(samples))
}

/// Decode WAV file specifically
pub fn wav(path: &str) -> Result<Arc<Vec<f32>>> {
    let file = std::fs::File::open(path)
        .map_err(|e| AudioError::FlacDecodeError(path.to_string(), e.to_string()))?;

    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("wav");

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| AudioError::FlacDecodeError(path.to_string(), e.to_string()))?;

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .ok_or_else(|| {
            AudioError::FlacDecodeError(path.to_string(), "No audio track found".to_string())
        })?;

    let track_id = track.id;
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| AudioError::FlacDecodeError(path.to_string(), e.to_string()))?;

    let mut samples: Vec<f32> = Vec::new();
    let mut sample_buf: Option<SampleBuffer<f32>> = None;

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(e) => return Err(AudioError::FlacDecodeError(path.to_string(), e.to_string())),
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = decoder
            .decode(&packet)
            .map_err(|e| AudioError::FlacDecodeError(path.to_string(), e.to_string()))?;

        let buf = sample_buf.get_or_insert_with(|| {
            SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec())
        });

        buf.copy_interleaved_ref(decoded);
        samples.extend_from_slice(buf.samples());
    }

    if samples.is_empty() {
        return Err(AudioError::FlacDecodeError(
            path.to_string(),
            "No audio data decoded".to_string(),
        ));
    }

    Ok(Arc::new(samples))
}

/// Legacy decode function for backwards compatibility
/// Routes to appropriate decoder based on file extension
pub fn decode(path: &str) -> Result<Arc<Vec<f32>>> {
    let path_obj = std::path::Path::new(path);
    
    if let Some(ext) = path_obj.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "flac" => flac(path),
            "wav" | "wave" => wav(path),
            _ => Err(AudioError::FlacDecodeError(
                path.to_string(), 
                format!("Unsupported file extension: {}", ext)
            ))
        }
    } else {
        Err(AudioError::FlacDecodeError(
            path.to_string(), 
            "No file extension found".to_string()
        ))
    }
}

pub fn pitch_ratio(recorded_midi: u8, target_midi: u8) -> f32 {
    2.0f32.powf((target_midi as f32 - recorded_midi as f32) / 12.0)
}

pub fn pitch_to_midi(pitch: &str) -> Option<u8> {
    parser::note_name_to_midi(pitch)
}
