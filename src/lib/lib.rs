// lib.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, JsCast};
use js_sys::{Array, Float32Array};
use core::f64::consts::PI;

pub struct IIRFilter {
    g: f64,
    q: f64,
    fc: f64,
    rate: f64,
    filter_type: FilterType,
    b: Vec<f64>,
    a: Vec<f64>,
    // Buffers to keep track of the last two input and output samples
    x: [f64; 2],
    y: [f64; 2],
}

#[derive(Clone, Copy)]
pub enum FilterType {
    HighShelfDeMan,
    HighPass,
}

impl IIRFilter {
    pub fn new(g: f64, q: f64, fc: f64, rate: f64, filter_type: FilterType) -> Self {
        let mut filter = IIRFilter {
            g,
            q,
            fc,
            rate,
            filter_type,
            b: vec![0.0; 3],
            a: vec![0.0; 3],
            x: [0.0; 2],
            y: [0.0; 2],
        };
        filter.update_coefficients();
        filter
    }

    pub fn update_coefficients(&mut self) {
        let k = (PI * self.fc / self.rate).tan();
        match self.filter_type {
            FilterType::HighShelfDeMan => {
                let vh = 10f64.powf(self.g / 20.0);
                let vb = vh.powf(0.499666774155);
                let a0_ = 1.0 + k / self.q + k * k;

                self.b[0] = (vh + vb * k / self.q + k * k) / a0_;
                self.b[1] = 2.0 * (k * k - vh) / a0_;
                self.b[2] = (vh - vb * k / self.q + k * k) / a0_;

                self.a[0] = 1.0;
                self.a[1] = 2.0 * (k * k - 1.0) / a0_;
                self.a[2] = (1.0 - k / self.q + k * k) / a0_;
            },
            FilterType::HighPass => {
                self.b[0] = (1.0 + k.cos()) / 2.0;
                self.b[1] = -(1.0 + k.cos());
                self.b[2] = (1.0 + k.cos()) / 2.0;

                let a0 = 1.0 + k / self.q + k * k;

                self.a[0] = 1.0;
                self.a[1] = -2.0 * k.cos() / a0;
                self.a[2] = (1.0 - k / self.q + k * k) / a0;
            },
        }
    }

    pub fn process(&mut self, input: f64) -> f64 {
        let output = self.b[0] * input
            + self.b[1] * self.x[0]
            + self.b[2] * self.x[1]
            - self.a[1] * self.y[0]
            - self.a[2] * self.y[1];
    
        // Update the buffers for the next iteration
        self.x[1] = self.x[0];
        self.x[0] = input;
        self.y[1] = self.y[0];
        self.y[0] = output;
    
        output
    }
}

#[wasm_bindgen]
pub fn process(js_audio_data: JsValue, sample_rate: f64) -> Result<f32, JsValue> {
    let outer_array: Array = js_audio_data.into();
    let mut audio_data = Vec::new();
    if outer_array.length() > 2 {return Err(JsValue::from_str(&format!("Unsupported for {:?} channels .", outer_array.length())));}
    for i in 0..outer_array.length() {
        let inner_js_value: JsValue = outer_array.get(i);

        if let Ok(float32_array) = inner_js_value.dyn_into::<Float32Array>() {
            let channel_data: Vec<f32> = float32_array.to_vec();
            audio_data.push(channel_data);
        } else {
            return Err(JsValue::from_str(&format!("Failed to convert to Float32Array for channel {:?}.", i)));
        }
    }
    let loudness = process_audio(audio_data,sample_rate);
    Ok(loudness)
}

fn process_audio(audio_data: Vec<Vec<f32>>, sample_rate: f64) -> f32 {
    const BLOCK_SIZE_MS: usize = 400;
    const OVERLAP: f32 = 0.75;
    const CHANNEL_WEIGHTS: [f32; 5] = [1.0, 1.0, 1.0, 1.41, 1.41];
    const THRESHOLD_LUFS: f32 = -70.0;

    let block_size_samples = (sample_rate as f32 * BLOCK_SIZE_MS as f32 / 1000.0) as usize;
    let overlap_samples = (block_size_samples as f32 * OVERLAP) as usize;
    let step_samples = block_size_samples - overlap_samples;

    let mut pre_filter = IIRFilter::new(
        3.999843853973347,
        0.7071752369554193,
        1681.9744509555319,
        sample_rate,
        FilterType::HighShelfDeMan
    );

    let mut rlb_filter = IIRFilter::new(
        0.0,
        0.5003270373253953,
        38.13547087613982,
        sample_rate,
        FilterType::HighPass
    );
    let mut blockwise_mean_squares: Vec<Vec<f32>> = Vec::new();

    for (channel_index, channel_samples) in audio_data.iter().enumerate() {
        blockwise_mean_squares.push(Vec::new());
        let prefiltered_samples: Vec<f32> = channel_samples.iter().map(|&sample|{
            let after_pre_filter = pre_filter.process(sample as f64);
            let after_rlb_filter = rlb_filter.process(after_pre_filter);
            after_rlb_filter as f32
        }).collect();
        let squared_samples: Vec<f32> = prefiltered_samples
            .iter()
            .map(|&sample| sample * sample )
            .collect();

        for start in (0..squared_samples.len()).step_by(step_samples) {
            if start + block_size_samples <= squared_samples.len() {
                let block = &squared_samples[start..start+block_size_samples];
                let block_mean_square: f32 = block.iter().sum::<f32>() / block_size_samples as f32;
                blockwise_mean_squares[channel_index].push(block_mean_square);
            }
        }
    }

    let mut blockwise_loudness: Vec<f32> = Vec::new();

    // Assuming number of blocks for each channel is the same
    let num_blocks = blockwise_mean_squares[0].len();

    for j in 0..num_blocks {
        let mut sum = 0.0;
        for i in 0..audio_data.len() {
            sum += CHANNEL_WEIGHTS[i] * blockwise_mean_squares[i][j];
        }
        let loudness: f32 = -0.691 + 10.0 * sum.log10();
        if loudness >= THRESHOLD_LUFS {
            blockwise_loudness.push(loudness);
        }
    }
    let avg_gated_all_channels: f32 = {
        let mut gated: Vec<f32> = Vec::new();
        for (i, _) in blockwise_mean_squares.iter().enumerate() {
            let sum = blockwise_mean_squares[i].iter().sum::<f32>();
            let avg = sum / blockwise_loudness.len() as f32;
            gated.push(avg * CHANNEL_WEIGHTS[i]);
        }
        gated.iter().sum::<f32>()
    };
    let relative_threshold: f32 = -0.691 + 10.0 * avg_gated_all_channels.log10() - 10.0 as f32;

    let gated_blocks: Vec<usize> = blockwise_loudness.iter().enumerate()
    .filter(|&(_, &l_j)| l_j > relative_threshold && l_j > THRESHOLD_LUFS)
    .map(|(j, _)| j)
    .collect();

    let mut avg_gated_channels: Vec<f32> = Vec::new();

    for mean_squares in blockwise_mean_squares.iter() {
        let mut sum = 0.0;
        for &i in &gated_blocks {
            sum += mean_squares[i];
        }
        let avg = if !gated_blocks.is_empty() {
            sum / gated_blocks.len() as f32
        } else {
            0.0
        };
        avg_gated_channels.push(avg);
    }

    let mut lufs: f32 = 0.0;
    for (i, &avg) in avg_gated_channels.iter().enumerate() {
        lufs += CHANNEL_WEIGHTS[i] * avg;
    }
    lufs = -0.691 + 10.0 * lufs.log10();
    lufs
}

fn calculate_true_peak(audio_data: Vec<Vec<f32>>, sample_rate: f64) -> f32 {
    let true_peak: f32 = 0.0;
    let inserted_zero = insert_zeros(audio_data, 4);
    true_peak
}
fn insert_zeros(input: Vec<Vec<f32>>, factor: usize) -> Vec<Vec<f32>> {
    let mut inserted: Vec<Vec<f32>> = Vec::new();
    for (channel_index, channel_samples) in input.iter().enumerate() {
        for &sample in channel_samples {
            inserted[channel_index].push(sample);
            for _ in 1..factor {
                inserted[channel_index].push(0.0);
            }
        }
    }
    inserted
}
fn sinc(x: f32) -> f32 {
    if x == 0.0 {
        1.0
    } else {
        let pi_x = std::f32::consts::PI * x;
        pi_x.sin() / pi_x
    }
}