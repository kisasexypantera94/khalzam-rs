use minimp3::{Decoder, Error, Frame};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;
use std::fs::File;
use std::i16;

const FFT_WINDOW_SIZE: usize = 4096;
const FREQ_BINS: &[usize] = &[40, 80, 120, 180, 300];
const FUZZ_FACTOR: usize = 2;

fn decode_mp3(filename: &str) -> Result<Vec<i16>, Error> {
    let mut decoder = Decoder::new(File::open(filename).unwrap());
    let mut frames = Vec::<i16>::new();

    loop {
        match decoder.next_frame() {
            Ok(Frame {
                data,
                sample_rate,
                channels,
                ..
            }) => frames.extend(data),
            Err(Error::Eof) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(frames)
}

fn stereo_i16_to_mono_f64(samples_i16: &Vec<i16>) -> Vec<f64> {
    let mut samples_f64 = Vec::new();

    for pair in samples_i16.chunks_exact(2) {
        samples_f64.push((pair[0] as f64 + pair[1] as f64) / 2 as f64 / i16::MAX as f64);
    }

    samples_f64
}

fn get_key_points(arr: &Vec<Complex<f64>>) -> usize {
    let mut high_scores: Vec<f64> = vec![0.0; FREQ_BINS.len()];
    let mut record_points: Vec<usize> = vec![0; FREQ_BINS.len()];

    for bin in FREQ_BINS[0]..FREQ_BINS[FREQ_BINS.len() - 1] {
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

fn hash(arr: &Vec<usize>) -> usize {
    (arr[3] - (arr[3] % FUZZ_FACTOR)) * usize::pow(10, 8)
        + (arr[2] - (arr[2] % FUZZ_FACTOR)) * usize::pow(10, 5)
        + (arr[1] - (arr[1] % FUZZ_FACTOR)) * usize::pow(10, 2)
        + (arr[0] - (arr[0] % FUZZ_FACTOR))
}

pub fn calc_fingerprint(filename: &str) -> Result<Vec<usize>, Error> {
    let pcm_f64 = stereo_i16_to_mono_f64(&decode_mp3(filename)?);

    let mut hash_array = Vec::<usize>::new();
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(FFT_WINDOW_SIZE);

    for chunk in pcm_f64.chunks_exact(FFT_WINDOW_SIZE) {
        let mut input: Vec<Complex<f64>> = chunk.iter().map(|x| Complex::from(x)).collect();
        let mut output: Vec<Complex<f64>> = vec![Complex::zero(); FFT_WINDOW_SIZE];
        fft.process(&mut input, &mut output);

        hash_array.push(get_key_points(&output));
    }

    Ok(hash_array)
}
