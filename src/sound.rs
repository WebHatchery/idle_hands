//! Small procedural feedback sounds selected by the profile's sound set.

use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use macroquad_toolkit::synth::{render_wav, SynthConfig, Voice, Wave};

#[derive(Debug, Clone, Copy)]
pub enum SoundCue {
    Tap,
    Success,
}

pub struct SoundBank {
    sets: Vec<Vec<Sound>>,
}

impl SoundBank {
    pub async fn load() -> Self {
        let config = SynthConfig {
            sample_rate: 22_050,
            master_gain: 0.22,
        };
        let mut sets = Vec::new();
        for set in 0..3 {
            let tap = render_wav(&[tap_voice(set)], &config, 0x1D1E_7000 + set as u64);
            let success = render_wav(&success_voices(set), &config, 0x1D1E_7100 + set as u64);
            let mut sounds = Vec::new();
            if let (Ok(tap), Ok(success)) = (
                load_sound_from_bytes(&tap).await,
                load_sound_from_bytes(&success).await,
            ) {
                sounds.push(tap);
                sounds.push(success);
            }
            sets.push(sounds);
        }
        Self { sets }
    }

    pub fn play(&self, set: u8, cue: SoundCue) {
        let Some(sounds) = self.sets.get(set as usize % self.sets.len().max(1)) else {
            return;
        };
        let index = match cue {
            SoundCue::Tap => 0,
            SoundCue::Success => 1,
        };
        if let Some(sound) = sounds.get(index) {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: 0.7,
                },
            );
        }
    }
}

fn tap_voice(set: usize) -> Voice {
    let frequencies = [560., 440., 680.];
    let waves = [Wave::Sine, Wave::Triangle, Wave::Square];
    Voice::tone(0., 0.075, frequencies[set], 0.42).wave(waves[set])
}

fn success_voices(set: usize) -> [Voice; 2] {
    let roots = [440., 392., 523.];
    [
        Voice::tone(0., 0.10, roots[set], 0.30).wave(Wave::Triangle),
        Voice::tone(0.07, 0.16, roots[set] * 1.5, 0.27).wave(Wave::Sine),
    ]
}
