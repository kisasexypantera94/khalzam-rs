use minimp3::{Decoder, Frame};
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFT;
use std::error::Error;
use std::fs::File;

const FFT_WINDOW_SIZE: usize = 4096;
const FREQ_BINS: &[usize] = &[40, 80, 120, 180, 300];
const FREQ_BIN_FIRST: usize = 40;
const FREQ_BIN_LAST: usize = 300;
const FUZZ_FACTOR: usize = 2;

/// Helper struct for calculating acoustic fingerprint
pub struct FingerprintHandle {
    /// FFT algorithm
    fft: Radix4<f64>,
}

impl FingerprintHandle {
    pub fn new() -> FingerprintHandle {
        FingerprintHandle {
            fft: Radix4::new(FFT_WINDOW_SIZE, false),
        }
    }

    pub fn calc_fingerprint(&self, filename: &str) -> Result<Vec<usize>, Box<Error>> {
        let pcm_f64 = decode_mp3(filename)?;
        let mut hash_array = Vec::<usize>::new();

        for chunk in pcm_f64.chunks_exact(FFT_WINDOW_SIZE) {
            let mut input: Vec<Complex<f64>> = chunk.iter().map(Complex::from).collect();
            let mut output: Vec<Complex<f64>> = vec![Complex::zero(); FFT_WINDOW_SIZE];
            self.fft.process(&mut input, &mut output);

            hash_array.push(get_key_points(&output));
        }

        Ok(hash_array)
    }
}

/// Mp3 decoding function.
/// Samples are read frame by frame and pushed to the vector.
/// Conversion to mono is done by simply taking the mean of left and right channels.
fn decode_mp3(filename: &str) -> Result<Vec<f64>, Box<Error>> {
    let mut decoder = Decoder::new(File::open(filename)?);
    let mut frames = Vec::new();

    loop {
        match decoder.next_frame() {
            Ok(Frame { data, channels, .. }) => match channels {
                2 => {
                    for pair in data.chunks_exact(2) {
                        frames.push(f64::from(pair[0] / 2 + pair[1] / 2));
                    }
                }
                1 => {
                    for &sample in data.iter() {
                        frames.push(f64::from(sample));
                    }
                }
                _ => return Err(Box::from("Invalid number of channels")),
            },
            Err(minimp3::Error::Eof) => break,
            Err(e) => return Err(Box::from(e)),
        }
    }

    Ok(frames)
}

/// Find points with max magnitude in each of the bins
fn get_key_points(arr: &[Complex<f64>]) -> usize {
    let mut high_scores: Vec<f64> = vec![0.0; FREQ_BINS.len()];
    let mut record_points: Vec<usize> = vec![0; FREQ_BINS.len()];

    for bin in FREQ_BIN_FIRST..=FREQ_BIN_LAST {
        let magnitude = arr[bin].re.hypot(arr[bin].im);

        let mut bin_idx = 0;
        while FREQ_BINS[bin_idx] < bin {
            bin_idx += 1;
        }

        if magnitude > high_scores[bin_idx] {
            high_scores[bin_idx] = magnitude;
            record_points[bin_idx] = bin;
        }
    }

    hash(&record_points)
}

fn hash(arr: &[usize]) -> usize {
    (arr[3] - (arr[3] % FUZZ_FACTOR)) * usize::pow(10, 8)
        + (arr[2] - (arr[2] % FUZZ_FACTOR)) * usize::pow(10, 5)
        + (arr[1] - (arr[1] % FUZZ_FACTOR)) * usize::pow(10, 2)
        + (arr[0] - (arr[0] % FUZZ_FACTOR))
}
