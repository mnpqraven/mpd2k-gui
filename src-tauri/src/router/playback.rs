use crate::{error::AppError, meta::types::AudioTrackIpc};
use rodio::{Decoder, Sink, Source};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, sync::Arc, time::Duration};
use tauri::AppHandle;

#[taurpc::procedures(path = "playback", event_trigger = PlaybackEvent)]
pub trait PlaybackApi {
    async fn play(
        track: AudioTrackIpc,
        app_handle: AppHandle<tauri::Wry>,
    ) -> Result<PlaybackState, AppError>;

    async fn pause_toggle(app_handle: AppHandle<tauri::Wry>);

    #[taurpc(event)]
    async fn ev_playback_state(app_handle: AppHandle<tauri::Wry>, playback_state: PlaybackState);
}

#[derive(Clone, Serialize, Deserialize, specta::Type, Default)]
pub enum PlayStatus {
    Play,
    Pause,
    #[default]
    Stopped,
}

#[taurpc::ipc_type]
pub struct PlaybackState {
    #[serde(skip)]
    pub sink: Option<Arc<Sink>>,
    pub now_playing: Option<AudioTrackIpc>,
    #[serde(skip)]
    pub now_playing_dur: Option<Duration>,
    #[serde(rename = "duration_secs")]
    pub now_playing_dur_secs: Option<u32>,
    pub status: PlayStatus,
}

#[taurpc::resolvers]
impl PlaybackApi for PlaybackState {
    async fn play(
        mut self,
        track: AudioTrackIpc,
        app_handle: AppHandle<tauri::Wry>,
    ) -> Result<PlaybackState, AppError> {
        let file = BufReader::new(File::open(&track.path).unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // Play the sound directly on the device
        let dur = source.total_duration();

        if let Some(ref sink) = self.sink {
            sink.clear();
            sink.append(source);
            sink.play();
        }
        self.now_playing = Some(track.clone());
        self.now_playing_dur = dur;
        self.now_playing_dur_secs = dur.and_then(|e| u32::try_from(e.as_secs()).ok());
        self.status = PlayStatus::Play;

        PlaybackEvent::new(app_handle)
            .ev_playback_state(self.clone())
            .unwrap();
        Ok(self.clone())
    }

    /// toggle the play state of now plyaing
    async fn pause_toggle(mut self, app_handle: AppHandle<tauri::Wry>) {
        if let Some(ref sink) = self.sink {
            match sink.is_paused() {
                true => {
                    sink.play();
                    self.status = PlayStatus::Play;
                }
                false => {
                    sink.pause();
                    self.status = PlayStatus::Pause;
                }
            }
        }

        PlaybackEvent::new(app_handle)
            .ev_playback_state(self.clone())
            .unwrap();
    }
}

impl PlaybackState {
    pub fn new(sink: Arc<Sink>) -> Self {
        Self {
            sink: Some(sink),
            now_playing: None,
            now_playing_dur: None,
            now_playing_dur_secs: None,
            status: PlayStatus::Stopped,
        }
    }
}
