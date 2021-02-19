use std::ops::DerefMut;

use super::music::Music;

pub mod context;

pub fn bind_world_music() {
    if cfg!(not(target_arch = "wasm32")) {
        std::thread::spawn( || {
            bind_music_fn();
        });
    } else {
        bind_music_fn();
    }
    
}

fn bind_music_fn() {
    if let Some(mut audio_context) = macroquad::prelude::collections::storage::get_mut::<self::context::AudioContext>() {
        audio_context.deref_mut().bind_music();     
    }
}

impl Music {

    pub fn included_bytes(&self) -> Option<&[u8]> { // To - do: Load dynamically from assets folder instead of specifying this
        match self {
            Music::IntroGamefreak => Some(include_bytes!("../../../build/assets/music/gamefreak.ogg")),
            Music::Title => Some(include_bytes!("../../../build/assets/music/title.ogg")),
            Music::Pallet => Some(include_bytes!("../../../build/assets/music/pallet.ogg")),
            Music::EncounterBoy => Some(include_bytes!("../../../build/assets/music/encounter_boy.ogg")),
            Music::BattleWild => Some(include_bytes!("../../../build/assets/music/vs_wild.ogg")),
            Music::BattleTrainer => Some(include_bytes!("../../../build/assets/music/vs_trainer.ogg")),
            Music::BattleGym => Some(include_bytes!("../../../build/assets/music/vs_gym.ogg")),
            _ => None,
        }
    }

}

pub fn from_ogg_bytes(bytes: &[u8], settings: kira::sound::SoundSettings) -> Result<kira::sound::Sound, kira::sound::error::SoundFromFileError> {
    use lewton::samples::Samples;
    let mut reader = lewton::inside_ogg::OggStreamReader::new(std::io::Cursor::new(bytes))?;
    let mut stereo_samples = vec![];
    while let Some(packet) = reader.read_dec_packet_generic::<Vec<Vec<f32>>>()? {
        let num_channels = packet.len();
        let num_samples = packet.num_samples();
        match num_channels {
            1 => {
                for i in 0..num_samples {
                    stereo_samples.push(kira::Frame::from_mono(packet[0][i]));
                }
            }
            2 => {
                for i in 0..num_samples {
                    stereo_samples.push(kira::Frame::new(packet[0][i], packet[1][i]));
                }
            }
            _ => return Err(kira::sound::error::SoundFromFileError::UnsupportedChannelConfiguration),
        }
    }
    Ok(kira::sound::Sound::from_frames(
        reader.ident_hdr.audio_sample_rate,
        stereo_samples,
        settings,
    ))
}