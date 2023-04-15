use std::{
    fs::File,
    io::{BufReader, BufWriter, Read},
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::Result;
use arrow2::{
    array::{
        ListArray, MutableArray, MutableDictionaryArray, MutablePrimitiveArray, MutableUtf8Array,
        StructArray, TryPush,
    },
    chunk::Chunk,
    datatypes::{DataType, Field, Schema},
};
use serde::Deserialize;
use zip::ZipArchive;

#[derive(Debug, Clone, Deserialize)]
struct Container {
    playlists: Vec<Playlist>,
}

#[derive(Default, Debug, Clone, Deserialize)]
struct Playlist {
    name: String,
    collaborative: String,
    pid: i64,
    modified_at: i64,
    num_tracks: u16,
    num_albums: u16,
    num_followers: i64,
    tracks: Vec<Track>,
}

#[derive(Default, Debug, Clone, Deserialize)]
struct Track {
    pos: u16,
    duration_ms: i64,
    artist_name: String,
    artist_uri: String,
    track_uri: String,
    track_name: String,
    album_uri: String,
    album_name: String,
}

fn main() -> Result<()> {
    const USAGE: &'static str = "Call as json_to_arrow {zip-file} {out-path}";

    let zip_path = std::env::args_os().nth(1).expect(USAGE);
    let zip_path = PathBuf::from(zip_path);

    let out_path = std::env::args_os().nth(2).expect(USAGE);
    let out_path = PathBuf::from(out_path);

    println!("Convert data");
    let n = 1_000;
    let start = Instant::now();

    let file = File::open(zip_path)?;
    let file = BufReader::new(file);
    let mut zip = ZipArchive::new(file)?;

    let mut name = MutableDictionaryArray::<u32, MutableUtf8Array<i64>>::new();
    let mut collaborative = MutableDictionaryArray::<u32, MutableUtf8Array<i64>>::new();
    let mut pid = MutablePrimitiveArray::<i64>::new();
    let mut modified_at = MutablePrimitiveArray::<i64>::new();
    let mut num_tracks = MutablePrimitiveArray::<u16>::new();
    let mut num_albums = MutablePrimitiveArray::<u16>::new();
    let mut num_followers = MutablePrimitiveArray::<i64>::new();

    let mut track_offset: Vec<i64> = vec![0];
    let mut track_pos = MutablePrimitiveArray::<u16>::new();
    let mut track_duration_ms = MutablePrimitiveArray::<i64>::new();
    let mut track_artist_name = MutableDictionaryArray::<u32, MutableUtf8Array<i64>>::new();
    let mut track_artist_uri = MutableDictionaryArray::<u32, MutableUtf8Array<i64>>::new();
    let mut track_track_uri = MutableDictionaryArray::<u32, MutableUtf8Array<i64>>::new();
    let mut track_track_name = MutableDictionaryArray::<u32, MutableUtf8Array<i64>>::new();
    let mut track_album_uri = MutableDictionaryArray::<u32, MutableUtf8Array<i64>>::new();
    let mut track_album_name = MutableDictionaryArray::<u32, MutableUtf8Array<i64>>::new();

    for i in 0..n {
        if i % 20 == 0 {
            let took = (Instant::now() - start).as_secs_f32();
            println!("{i} / {n}: {took:.2}");
        }

        let mut content = Vec::new();
        zip.by_name(&format!(
            "data/mpd.slice.{}-{}.json",
            1000 * i,
            1000 * (i + 1) - 1
        ))?
        .read_to_end(&mut content)?;

        let mut data: Container = serde_json::from_slice(&content)?;
        for item in data.playlists.iter_mut() {
            item.modified_at = 1000 * item.modified_at;
        }

        for playlist in &data.playlists {
            name.try_push(Some(&playlist.name))?;
            collaborative.try_push(Some(&playlist.collaborative))?;
            pid.try_push(Some(playlist.pid))?;
            modified_at.try_push(Some(playlist.modified_at))?;
            num_tracks.try_push(Some(playlist.num_tracks))?;
            num_albums.try_push(Some(playlist.num_albums))?;
            num_followers.try_push(Some(playlist.num_followers))?;

            track_offset.push(track_offset.last().unwrap() + playlist.tracks.len() as i64);
            for track in &playlist.tracks {
                track_pos.try_push(Some(track.pos))?;
                track_duration_ms.try_push(Some(track.duration_ms))?;
                track_artist_name.try_push(Some(&track.artist_name))?;
                track_artist_uri.try_push(Some(&track.artist_uri))?;
                track_track_uri.try_push(Some(&track.track_uri))?;
                track_track_name.try_push(Some(&track.track_name))?;
                track_album_uri.try_push(Some(&track.album_uri))?;
                track_album_name.try_push(Some(&track.album_name))?;
            }
        }
    }

    let name = name.into_box();
    let collaborative = collaborative.into_box();
    let pid = pid.as_box();
    let modified_at = modified_at.as_box();
    let num_tracks = num_tracks.as_box();
    let num_albums = num_albums.as_box();
    let num_followers = num_followers.as_box();

    let track_pos = track_pos.as_box();
    let track_duration_ms = track_duration_ms.as_box();
    let track_artist_name = track_artist_name.into_box();
    let track_artist_uri = track_artist_uri.into_box();
    let track_track_uri = track_track_uri.into_box();
    let track_track_name = track_track_name.into_box();
    let track_album_uri = track_album_uri.into_box();
    let track_album_name = track_album_name.into_box();

    let tracks_data_type = DataType::Struct(vec![
        Field::new("pos", track_pos.data_type().clone(), false),
        Field::new("duration_ms", track_duration_ms.data_type().clone(), false),
        Field::new("artist_name", track_artist_name.data_type().clone(), false),
        Field::new("artist_uri", track_artist_uri.data_type().clone(), false),
        Field::new("track_uri", track_track_uri.data_type().clone(), false),
        Field::new("track_name", track_track_name.data_type().clone(), false),
        Field::new("album_uri", track_album_uri.data_type().clone(), false),
        Field::new("album_name", track_album_name.data_type().clone(), false),
    ]);
    let tracks = StructArray::try_new(
        tracks_data_type,
        vec![
            track_pos,
            track_duration_ms,
            track_artist_name,
            track_artist_uri,
            track_track_uri,
            track_track_name,
            track_album_uri,
            track_album_name,
        ],
        None,
    )?
    .boxed();

    let tracks_data_type = DataType::LargeList(Box::new(Field::new(
        "item",
        tracks.data_type().clone(),
        false,
    )));
    let tracks =
        ListArray::try_new(tracks_data_type, track_offset.try_into()?, tracks, None)?.boxed();

    let fields = vec![
        Field::new("name", name.data_type().clone(), false),
        Field::new("collaborative", collaborative.data_type().clone(), false),
        Field::new("pid", pid.data_type().clone(), false),
        Field::new("modified_at", modified_at.data_type().clone(), false),
        Field::new("num_tracks", num_tracks.data_type().clone(), false),
        Field::new("num_albums", num_albums.data_type().clone(), false),
        Field::new("num_followers", num_followers.data_type().clone(), false),
        Field::new("tracks", tracks.data_type().clone(), false),
    ];

    let arrays = vec![
        name,
        collaborative,
        pid,
        modified_at,
        num_tracks,
        num_albums,
        num_followers,
        tracks,
    ];

    let (mut writer, ipc_fields) = build_ipc_writer(out_path, &fields)?;
    writer.write(&Chunk::try_new(arrays)?, Some(&ipc_fields))?;
    writer.finish()?;

    Ok(())
}

fn build_ipc_writer<P: AsRef<Path>>(
    path: P,
    fields: &[Field],
) -> Result<(
    arrow2::io::ipc::write::FileWriter<BufWriter<File>>,
    Vec<arrow2::io::ipc::IpcField>,
)> {
    use arrow2::io::ipc::write::{self, FileWriter, WriteOptions};

    let file = File::create(path)?;
    let file = BufWriter::new(file);

    let schema = Schema::from(fields.to_vec());
    let ipc_fields = write::default_ipc_fields(&schema.fields);

    let options = WriteOptions { compression: None };
    let writer = FileWriter::try_new(file, schema, Some(ipc_fields.clone()), options)?;

    Ok((writer, ipc_fields))
}
