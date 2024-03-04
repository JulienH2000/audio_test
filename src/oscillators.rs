use core::f32::consts::PI;
use cpal::SampleRate;

use crate::{audiolib::*};

#[derive(Copy, Clone)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}

#[derive(Copy, Clone)]
pub struct Oscillator {
    pub waveform: Waveform,
    pub current_sample_index: f32,
    pub frequency_hz: f32,
    pub amplitude: f32,
    pub phase_shift: f32,
}

impl Oscillator {
    pub fn new_oscillator (wave: Waveform, freq: f32, amp: f32) -> Oscillator {
        return Oscillator {
            waveform: wave,
            current_sample_index: 0f32,
            frequency_hz: freq,
            amplitude: amp,
            phase_shift: 1f32,
        }
    }

    fn next_sample_index (&mut self, engine_sample_rate: &f32) {
        let sample_rate = engine_sample_rate;
        self.current_sample_index = (self.current_sample_index + 1.0) % sample_rate;
    }

    fn calculate_sine_output_from_freq(&self, freq: f32, engine_sample_rate: &f32) -> f32 {
        let sample_rate = engine_sample_rate;
        self.amplitude * ((self.current_sample_index * freq * 2.0 * PI / sample_rate) + self.phase_shift).sin()
    }

    fn calculate_square_output_from_freq(&self, engine_sample_rate: &f32) -> f32 {
        let mut output = 0.0;
        let sample_rate = engine_sample_rate;
        let freq = self.frequency_hz;
        let phase = self.current_sample_index * freq * 2.0 * PI / sample_rate;
        let period = sample_rate / freq;
        //let t = phase / period;
        //let half_phase = self.sample_rate / 2.0;

        // Naive Square gen

        output = self.amplitude * ((phase).sin()).signum();

        // PolyBLEP Substraction 

        //output = output + self.calc_poly_blep(t);
        //output = output - self.calc_poly_blep((t + 0.5) % 1.0);
        output
       
  }

    fn calculate_saw_output_from_freq(&mut self, engine_sample_rate: &f32) -> f32 {
        let sample_rate = engine_sample_rate;
        let freq = self.frequency_hz;
        let index = self.current_sample_index;
        let phase = self.current_sample_index * freq * 2.0 * PI / sample_rate;
        let period = sample_rate / freq;
        let t = phase / period;

        // Naive sawtooth gen
        let mut output = (2.0 * (index % period) / period) - 1.0;

        //let mut output = (2.0 * t) - 1.0;
        output = output * self.amplitude;

        // PolyBLEP Substraction

        output = output - self.calc_poly_blep(t);
        output
    }

    fn calculate_triangle_output_from_freq(&self) -> f32 {
        todo!()
    }

    fn _is_multiple_of_freq_above_nyquist(&self, multiple: f32, engine_sample_rate: &f32) -> bool {
        let sample_rate = engine_sample_rate;
        self.frequency_hz * multiple > sample_rate / 2.0
        
    }

    fn calc_poly_blep(&self, t: f32) -> f32 {
        /* t = phase / 2 PI */
        let mut t = t;
        //let dt = self.current_sample_index / 2.0 * PI;
        let dt = self.current_sample_index / 2.0 * PI;

        if t < dt {
            t = t / dt;
            return t + t - t * t - 1.0;
        } else if t > 1.0 - dt {
            t = t - 1.0 / dt;
            return t * t + t + t + 1.0;
        } else {
            return 0.0;
        }
    }

    pub fn sine_wave(&mut self, engine_sample_rate: &f32) -> f32 {
        self.next_sample_index(engine_sample_rate);
        self.calculate_sine_output_from_freq(self.frequency_hz, engine_sample_rate)
    }
    pub fn square_wave(&mut self, engine_sample_rate: &f32) -> f32 {
        self.next_sample_index(engine_sample_rate);
        self.calculate_square_output_from_freq(engine_sample_rate)
    }
    pub fn saw_wave(&mut self, engine_sample_rate: &f32) -> f32 {
        self.next_sample_index(engine_sample_rate);
        self.calculate_saw_output_from_freq(engine_sample_rate)
    }
    pub fn triangle_wave(&mut self, engine_sample_rate: &f32) -> f32 {
        self.next_sample_index(engine_sample_rate);
        self.calculate_triangle_output_from_freq()
    }

    pub fn update_freq(&mut self, nf: f32) {
        self.frequency_hz = nf;
    }
}