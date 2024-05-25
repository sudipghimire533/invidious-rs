use crate::endpoints;

pub fn test_type_str<Target: serde::de::DeserializeOwned>(
    file_name: &str,
) -> serde_json::Result<Target> {
    let file_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("tests")
        .join("responses-sample")
        .join(file_name);

    let file = std::fs::File::open(file_path.as_path())
        .expect(format!("Cannot open file: {}", file_path.to_string_lossy()).as_str());
    let reader = std::io::BufReader::new(file);

    serde_json::from_reader(reader)
}

macro_rules! assert_ok {
    ($statement: expr) => {
        match $statement {
            Ok(a) => a,
            Err(e) => panic!("Expected Ok. Got error: {e:?}"),
        }
    };
}

#[test]
fn stats() {
    assert_ok!(test_type_str::<endpoints::stats::OkCallbackResponse>(
        "instance-stats.json"
    ));
}

#[test]
fn video_info() {
    assert_ok!(test_type_str::<endpoints::videos::OkCallbackResponse>(
        "video-info.json"
    ));
}

#[test]
fn channel_info() {
    assert_ok!(test_type_str::<endpoints::channels::OkCallbackResponse>(
        "channel-info.json"
    ));
}

#[test]
fn playlist_info() {
    assert_ok!(test_type_str::<endpoints::playlists::OkCallbackResponse>(
        "playlist-info.json"
    ));
}

#[test]
fn search_all() {
    assert_ok!(test_type_str::<endpoints::search::OkCallbackResponse>(
        "search-all.json"
    ));
}
