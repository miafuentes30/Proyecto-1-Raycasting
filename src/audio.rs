use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;

pub struct Audio {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    bgm_sink: Sink,
    sfx_volume: f32,
}

impl Audio {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().expect("Error al inicializar audio");
        let bgm_sink = Sink::try_new(&handle).expect("Error al crear el sink de música de fondo");
        
        let mut audio = Audio {
            _stream: stream,
            handle,
            bgm_sink,
            sfx_volume: 0.3,
        };
        
        audio.load_and_play_music();
        audio
    }

    fn load_and_play_music(&mut self) {
        if let Some(file) = try_open_any(&[
            "assets/audio/music.mp3",
            "assets/music.mp3",
        ]) {
            if let Ok(src) = rodio::Decoder::new(BufReader::new(file)) {
                self.bgm_sink.append(src.repeat_infinite());
                self.bgm_sink.set_volume(0.2);
                self.bgm_sink.play();
            }
        }
    }

    pub fn play_collect(&self) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            if let Some(file) = try_open_any(&[
                "assets/audio/collect.wav",
                "assets/collect.wav",
            ]) {
                if let Ok(src) = rodio::Decoder::new(BufReader::new(file)) {
                    sink.append(src);
                    sink.set_volume(self.sfx_volume);
                    sink.detach();
                }
            }
        }
    }

    pub fn play_damage(&self) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            if let Some(file) = try_open_any(&[
                "assets/audio/damage.wav",
                "assets/damage.wav",
            ]) {
                if let Ok(src) = rodio::Decoder::new(BufReader::new(file)) {
                    sink.append(src);
                    sink.set_volume(self.sfx_volume);
                    sink.detach();
                }
            }
        }
    }

    pub fn play_footstep(&self) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            if let Some(file) = try_open_any(&[
                "assets/audio/footstep.wav",
                "assets/footstep.wav",
            ]) {
                if let Ok(src) = rodio::Decoder::new(BufReader::new(file)) {
                    sink.append(src);
                    sink.set_volume(self.sfx_volume * 0.5); // quieter footsteps
                    sink.detach();
                }
            }
        }
    }

    pub fn play_hit(&self) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            if let Some(file) = try_open_any(&[
                "assets/audio/damage.wav",
                "assets/audio/sfx_hit.wav",
                "assets/sfx_hit.wav",
                "assets/audio/hit.wav",
            ]) {
                if let Ok(src) = rodio::Decoder::new(BufReader::new(file)) {
                    sink.append(src);
                    sink.set_volume(self.sfx_volume);
                    sink.detach();
                }
            }
        }
    }

    pub fn play_chest(&self) {
        if let Ok(sink) = Sink::try_new(&self.handle) {
            if let Some(file) = try_open_any(&[
                "assets/audio/collect.wav",
                "assets/audio/sfx_chest.wav",
                "assets/sfx_chest.wav",
                "assets/audio/chest.wav",
            ]) {
                if let Ok(src) = rodio::Decoder::new(BufReader::new(file)) {
                    sink.append(src);
                    sink.set_volume(self.sfx_volume);
                    sink.detach();
                }
            }
        }
    }
}

fn try_open_any(paths: &[&str]) -> Option<File> {
    for p in paths {
        if let Ok(f) = File::open(p) {
            return Some(f);
        }
    }
    None
}