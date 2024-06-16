use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::sync::Arc;

pub struct SinkArc(Arc<Sink>);

pub struct PlaybackServer {
    sink: SinkArc,
    _stream: OutputStream,
}

impl Default for PlaybackServer {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaybackServer {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = SinkArc::new(&stream_handle);
        Self { sink, _stream }
    }

    /// returns a handle to the playback server
    pub fn handle(&self) -> Arc<Sink> {
        self.sink.arced()
    }
}

impl SinkArc {
    fn new(stream: &OutputStreamHandle) -> Self {
        Self(Arc::new(
            Sink::try_new(stream).expect("should always have at least one device"),
        ))
    }
    pub fn arced(&self) -> Arc<Sink> {
        self.0.clone()
    }
}
