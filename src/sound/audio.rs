use raylib::audio::RaylibAudio;

pub fn init_audio(master_volume: f32) -> RaylibAudio {
    let audio = RaylibAudio::init_audio_device().expect("Failed to initialize audio");
    audio.set_master_volume(master_volume);

    audio
}
