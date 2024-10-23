use crate::config::CommonConfig;
use jgenesis_common::frontend::AudioOutput;
use sdl2::AudioSubsystem;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use thiserror::Error;

const SAMPLE_RATE: i32 = 48000;
const CHANNELS: u8 = 2;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("Error opening SDL2 audio device: {0}")]
    OpenDevice(String),
}

struct QueueAudioCallback(Arc<Mutex<VecDeque<f32>>>);

impl AudioCallback for QueueAudioCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        let mut audio_queue = self.0.lock().unwrap();
        for sample in out {
            *sample = audio_queue.pop_front().unwrap_or(0.0);
        }
    }
}

pub struct SdlAudioOutput {
    audio_device: AudioDevice<QueueAudioCallback>,
    audio_queue: Arc<Mutex<VecDeque<f32>>>,
    audio_buffer: Vec<f32>,
    audio_sync: bool,
    internal_audio_buffer_len: u32,
    audio_sync_threshold: u32,
    audio_gain_multiplier: f64,
    sample_count: u64,
    speed_multiplier: u64,
}

impl SdlAudioOutput {
    pub fn create_and_init<KC, JC>(
        audio: &AudioSubsystem,
        config: &CommonConfig<KC, JC>,
    ) -> Result<Self, AudioError> {
        let audio_queue =
            Arc::new(Mutex::new(VecDeque::with_capacity((4 * SAMPLE_RATE / 60) as usize)));
        let audio_device =
            open_audio_device(audio, Arc::clone(&audio_queue), config.audio_device_queue_size)?;

        Ok(Self {
            audio_queue,
            audio_device,
            audio_buffer: Vec::with_capacity(config.internal_audio_buffer_size as usize),
            audio_sync: config.audio_sync,
            internal_audio_buffer_len: config.internal_audio_buffer_size,
            audio_sync_threshold: config.audio_sync_threshold,
            audio_gain_multiplier: decibels_to_multiplier(config.audio_gain_db),
            sample_count: 0,
            speed_multiplier: 1,
        })
    }

    pub fn reload_config<KC, JC>(
        &mut self,
        config: &CommonConfig<KC, JC>,
    ) -> Result<(), AudioError> {
        self.audio_sync = config.audio_sync;
        self.internal_audio_buffer_len = config.internal_audio_buffer_size;
        self.audio_sync_threshold = config.audio_sync_threshold;
        self.audio_gain_multiplier = decibels_to_multiplier(config.audio_gain_db);

        if config.audio_device_queue_size != self.audio_device.spec().samples {
            log::info!("Recreating SDL audio queue with size {}", config.audio_device_queue_size);
            self.audio_device.pause();

            let new_audio_device = open_audio_device(
                self.audio_device.subsystem(),
                Arc::clone(&self.audio_queue),
                config.audio_device_queue_size,
            )?;
            self.audio_device = new_audio_device;
        }

        Ok(())
    }

    pub fn set_speed_multiplier(&mut self, speed_multiplier: u64) {
        self.speed_multiplier = speed_multiplier;
    }

    #[must_use]
    pub fn should_wait_for_audio(&self) -> bool {
        let audio_queue = self.audio_queue.lock().unwrap();
        self.audio_sync && audio_queue_len_bytes(&audio_queue) >= self.audio_sync_threshold
    }
}

fn open_audio_device(
    audio_subsystem: &AudioSubsystem,
    audio_queue: Arc<Mutex<VecDeque<f32>>>,
    audio_device_queue_size: u16,
) -> Result<AudioDevice<QueueAudioCallback>, AudioError> {
    let audio_device = audio_subsystem
        .open_playback(
            None,
            &AudioSpecDesired {
                freq: Some(SAMPLE_RATE),
                channels: Some(CHANNELS),
                samples: Some(audio_device_queue_size),
            },
            |_| QueueAudioCallback(audio_queue),
        )
        .map_err(AudioError::OpenDevice)?;
    audio_device.resume();

    Ok(audio_device)
}

fn decibels_to_multiplier(decibels: f64) -> f64 {
    10.0_f64.powf(decibels / 20.0)
}

impl AudioOutput for SdlAudioOutput {
    type Err = AudioError;

    #[inline]
    fn push_sample(&mut self, sample_l: f64, sample_r: f64) -> Result<(), Self::Err> {
        self.sample_count += 1;
        if self.sample_count % self.speed_multiplier != 0 {
            return Ok(());
        }

        self.audio_buffer.push((sample_l * self.audio_gain_multiplier) as f32);
        self.audio_buffer.push((sample_r * self.audio_gain_multiplier) as f32);

        if self.audio_buffer.len() >= self.internal_audio_buffer_len as usize {
            let mut audio_queue = self.audio_queue.lock().unwrap();
            if !self.audio_sync && audio_queue_len_bytes(&audio_queue) >= self.audio_sync_threshold
            {
                // Audio queue is full; drop samples
                self.audio_buffer.clear();
                return Ok(());
            }

            audio_queue.extend(self.audio_buffer.drain(..));
        }

        Ok(())
    }
}

fn audio_queue_len_bytes(audio_queue: &VecDeque<f32>) -> u32 {
    // f32 is 4 bytes
    4 * audio_queue.len() as u32
}
