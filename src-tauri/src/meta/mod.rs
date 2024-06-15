#![allow(clippy::upper_case_acronyms)]

pub mod traits;
pub mod types;

use audiotags::Tag;
use rodio::Decoder;
use std::{
    cmp::Ordering, fmt::Debug, fs::File, io::BufReader, path::Path, str::FromStr, sync::Arc,
};
use strum::{Display, EnumString};
use tracing::instrument;
use types::{AlbumDate, AudioTrack, SomeAlbumDate};

use crate::error::AppError;

#[derive(Debug, EnumString, Display, PartialEq, Eq)]
enum SupportedAudio {
    #[strum(ascii_case_insensitive, to_string = "mp3")]
    MP3,
    // MP4,
    #[strum(ascii_case_insensitive)]
    WAV,
    #[strum(ascii_case_insensitive)]
    VORBIS,
    #[strum(ascii_case_insensitive)]
    FLAC,
    #[strum(ascii_case_insensitive)]
    AAC,
}

/// Returns true if the file is a valid audio file with supported codec
pub fn is_supported_audio<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    match path.extension() {
        Some(ext) => {
            let ext = ext.to_string_lossy();
            SupportedAudio::from_str(&ext).is_ok()
        }
        None => false,
    }
}

/// this function converts empty string to None
pub fn empty_to_option<T>(text: &str) -> Option<T>
where
    T: FromStr + Default,
{
    match text.is_empty() {
        true => None,
        false => Some(text.parse::<T>().unwrap_or_default()),
    }
}

pub fn reverse_ord_option(ord: Option<Ordering>) -> Option<Ordering> {
    if let Some(o) = ord {
        return match o {
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => Some(Ordering::Equal),
            Ordering::Greater => Some(Ordering::Less),
        };
    }
    None
}

pub fn reverse_ord(ord: Ordering) -> Ordering {
    match ord {
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => Ordering::Equal,
        Ordering::Greater => Ordering::Less,
    }
}

/// This function is not cheap, running in parallel is recommended
#[instrument]
pub fn read_tag<P: AsRef<Path> + Debug + Into<Arc<str>>>(path: P) -> Result<AudioTrack, AppError> {
    // TODO: unwrap
    let tag = Tag::new().read_from_path(&path).unwrap();
    let name = tag.title().unwrap_or_default().to_string();
    let album: Option<Arc<str>> = tag.album_title().map(Into::into);
    let artist = tag.artist().map(Into::into);
    let date = tag.date_raw().and_then(AlbumDate::parse);
    let album_artist: Option<Arc<str>> = tag.album_artist().map(Into::into);
    let track_no = tag.track_number();

    let track = AudioTrack {
        name: name.into(),
        path: path.into(),
        track_no,
        artist,
        date: SomeAlbumDate(date),
        album,
        album_artist,
        binary_hash: None,
    };
    Ok(track)
}

pub fn create_source<P: AsRef<Path>>(path: P) -> Result<Decoder<BufReader<File>>, AppError> {
    let file = BufReader::new(File::open(path)?);
    // TODO: Decoder error
    let source = Decoder::new(file).map_err(|_| AppError::GenericError("bad encode".into()))?;
    Ok(source)
}
