use audiotags::TimestampTag;
use chrono::{Datelike, NaiveDate};
use csv::StringRecord;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::api::dir::read_dir;
use tracing::info;

use crate::{error::AppError, meta::empty_to_option};

use super::read_tag;

// NOTE: keep expanding this or migrate to album(outer struct) > tracks(inner struct)
#[taurpc::ipc_type]
#[derive(Debug, PartialEq, Eq)]
pub struct AudioTrackIpc {
    pub name: String,
    pub path: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub track_no: Option<u16>,
    pub date: SomeAlbumDate,
    pub binary_hash: Option<String>,
}

#[taurpc::ipc_type]
#[derive(Debug, PartialEq, Eq)]
pub struct AlbumMetaIpc {
    pub album_artist: Option<String>,
    pub date: SomeAlbumDate,
    pub name: Option<String>,
}

#[taurpc::ipc_type]
pub struct AlbumIpc {
    pub meta: AlbumMetaIpc,
    pub tracks: Vec<AudioTrackIpc>,
}

pub struct Album {
    pub meta: AlbumMeta,
    pub tracks: Vec<AudioTrack>,
}

// NOTE: keep expanding this or migrate to album(outer struct) > tracks(inner struct)
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AudioTrack {
    pub name: Arc<str>,
    pub path: Arc<str>,
    pub artist: Option<Arc<str>>,
    pub album: Option<Arc<str>>,
    pub album_artist: Option<Arc<str>>,
    pub track_no: Option<u16>,
    pub date: SomeAlbumDate,
    pub binary_hash: Option<Arc<str>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AlbumMeta {
    pub album_artist: Option<Arc<str>>,
    pub date: SomeAlbumDate,
    pub name: Option<Arc<str>>,
}

#[taurpc::ipc_type]
#[derive(Debug, PartialEq, Eq, Copy)]
pub struct SomeAlbumDate(pub Option<AlbumDate>);

#[taurpc::ipc_type]
#[derive(Debug, PartialEq, Eq, Copy)]
pub struct AlbumDate {
    // at least year is always `Some`, if we can't parse year then the whole
    // struct is safe to be `None`
    pub year: u32,
    pub month: Option<u8>,
    pub day: Option<u8>,
}

impl AudioTrack {
    const CSV_COLS: usize = 8;

    // TODO: perf + unicode check
    pub fn new<P: AsRef<Path> + Into<Arc<str>>>(path: P) -> Self {
        let name = path
            .as_ref()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        Self {
            name: name.into(),
            path: path.into(),
            artist: None,
            album: None,
            album_artist: None,
            track_no: None,
            date: SomeAlbumDate(None),
            binary_hash: None,
        }
    }

    pub fn to_record(&self) -> StringRecord {
        let as_vec: &[String; Self::CSV_COLS] = &[
            self.name.to_string(),
            self.path.to_string(),
            self.track_no.map(|no| no.to_string()).unwrap_or_default(),
            self.artist.as_ref().map(Arc::to_string).unwrap_or_default(),
            self.album.as_ref().map(Arc::to_string).unwrap_or_default(),
            self.album_artist
                .as_ref()
                .map(Arc::to_string)
                .unwrap_or_default(),
            self.date
                .0
                .as_ref()
                .map(AlbumDate::to_string)
                .unwrap_or_default(),
            // ----
            // NOTE: ALWAYS PUT THIS LAST FOR `record_hash`
            self.binary_hash
                .as_ref()
                .map(Arc::to_string)
                .unwrap_or_default(),
        ];
        StringRecord::from(as_vec.as_slice())
    }

    pub fn from_record(record: StringRecord) -> Result<Self, AppError> {
        if record.len() != Self::CSV_COLS {
            return Err(AppError::CsvParse);
        }
        let track = AudioTrack {
            name: record[0].into(),
            path: record[1].into(),
            track_no: empty_to_option(&record[2]),
            artist: empty_to_option::<String>(&record[3]).map(Into::into),
            album: empty_to_option::<String>(&record[4]).map(Into::into),
            album_artist: empty_to_option::<String>(&record[5]).map(Into::into),
            date: SomeAlbumDate(AlbumDate::parse(TimestampTag::Unknown(
                record[6].to_string(),
            ))),
            binary_hash: empty_to_option::<String>(&record[7]).map(Into::into),
        };
        info!(?track);

        Ok(track)
    }

    pub fn update_tag(&mut self) -> Result<(), AppError> {
        let new_trk = read_tag(self.path.as_ref())?;
        self.clone_from(&new_trk);
        Ok(())
    }

    pub fn try_cover_path(&self) -> Option<PathBuf> {
        let track_path = PathBuf::from(self.path.as_ref());
        let dir = track_path.parent();
        match dir {
            Some(dir) => {
                let img_paths: Vec<PathBuf> = read_dir(dir, false)
                    .unwrap()
                    .into_iter()
                    .filter(|e| {
                        ["png", "jpg"]
                            .into_iter()
                            .any(|ext| ext == e.path.extension().unwrap())
                    })
                    .map(|e| e.path)
                    .collect();
                return img_paths.first().cloned();
            }
            _ => None,
        }
    }
}

impl AlbumDate {
    pub fn parse(text: TimestampTag) -> Option<Self> {
        match text {
            TimestampTag::Id3(_) => todo!(),
            TimestampTag::Unknown(text) => {
                // TODO: more formats
                match NaiveDate::parse_from_str(&text, "%Y.%m.%d") {
                    Ok(s) => Some(Self {
                        year: s.year() as u32,
                        month: Some((s.month0() + 1) as u8),
                        day: Some((s.day0() + 1) as u8),
                    }),
                    Err(_) => None,
                }
            }
        }
    }
}
