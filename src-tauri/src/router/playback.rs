use crate::{error::AppError, meta::types::AudioTrackIpc};
use rodio::{Decoder, Sink, Source};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, sync::Arc, time::Duration};
use tauri::AppHandle;
use tokio::sync::Mutex;

#[taurpc::procedures(path = "playback", event_trigger = PlaybackEvent)]
pub trait PlaybackApi {
    async fn play(track: AudioTrackIpc, app_handle: AppHandle) -> Result<PlaybackState, AppError>;

    async fn play_pause() -> Result<PlaybackState, AppError>;
    async fn set_shuffle(to: bool) -> Result<bool, AppError>;
    async fn cycle_repeat() -> Result<RepeatStatus, AppError>;

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

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, Default)]
pub enum RepeatStatus {
    #[default]
    Off,
    Repeat,
    RepeatOne,
}

#[derive(Clone)]
pub struct PlaybackStateImpl(Arc<Mutex<PlaybackState>>);

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

    pub shuffle: bool,
    pub repeat: RepeatStatus,
}

#[taurpc::resolvers]
impl PlaybackApi for PlaybackStateImpl {
    async fn play(
        self,
        track: AudioTrackIpc,
        app_handle: AppHandle,
    ) -> Result<PlaybackState, AppError> {
        let mut data = self.0.lock().await;

        let file = BufReader::new(File::open(&track.path).unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // Play the sound directly on the device
        let dur = source.total_duration();

        if let Some(ref sink) = data.sink {
            sink.clear();
            sink.append(source);
            sink.play();
        }
        data.now_playing = Some(track.clone());
        data.now_playing_dur = dur;
        data.now_playing_dur_secs = dur.and_then(|e| u32::try_from(e.as_secs()).ok());
        data.status = PlayStatus::Play;

        // TODO: mutate queue list here

        PlaybackEvent::new(app_handle)
            .ev_playback_state(data.clone())
            .unwrap();
        Ok(data.clone())
    }

    async fn set_shuffle(self, to: bool) -> Result<bool, AppError> {
        let mut data = self.0.lock().await;
        data.shuffle = to;
        Ok(to)
    }

    async fn cycle_repeat(self) -> Result<RepeatStatus, AppError> {
        let mut data = self.0.lock().await;
        data.repeat = next_repeat(&data.repeat);

        Ok(data.repeat.clone())
    }

    /// toggle the play state of now plyaing
    async fn play_pause(self) -> Result<PlaybackState, AppError> {
        let mut data = self.0.lock().await;
        if let Some(ref sink) = data.sink {
            match sink.is_paused() {
                true => {
                    sink.play();
                    data.status = PlayStatus::Play;
                }
                false => {
                    sink.pause();
                    data.status = PlayStatus::Pause;
                }
            }
        }

        Ok(data.clone())
    }
}

impl PlaybackStateImpl {
    pub fn new(pb_state: PlaybackState) -> Self {
        Self(Arc::new(Mutex::new(pb_state)))
    }
}

impl PlaybackState {
    pub fn new(sink: Arc<Sink>) -> Self {
        println!("running new()");
        Self {
            sink: Some(sink),
            now_playing: None,
            now_playing_dur: None,
            now_playing_dur_secs: None,
            status: PlayStatus::Stopped,
            shuffle: false,
            repeat: RepeatStatus::Off,
        }
    }
}

fn next_repeat(val: &RepeatStatus) -> RepeatStatus {
    match val {
        RepeatStatus::Off => RepeatStatus::Repeat,
        RepeatStatus::Repeat => RepeatStatus::RepeatOne,
        RepeatStatus::RepeatOne => RepeatStatus::Off,
    }
}
