use serde::Deserialize;
use std::{fs::File, path::Path};

// A data structure that can be deserialised from any data format supported by serde.
#[derive(Debug, Deserialize)]
struct Feature {
    location: Location,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: i32,
    longitude: i32,
}

#[allow(dead_code)]
pub fn load() -> Vec<crate::routeguide::Feature> {
    // cargo run --bin routeguide-server from the workspace root. We need to
    // make sure the path starts with routeguide-server to conform with the Bazel
    // runfiles output.
    let file = File::open(Path::new("routeguide-server/data/route_guide_db.json"))
        .expect("failed to open data file");

    let decoded: Vec<Feature> =
        serde_json::from_reader(&file).expect("failed to deserialize features");

    decoded
        .into_iter()
        .map(|feature| crate::routeguide::Feature {
            name: feature.name,
            location: Some(crate::routeguide::Point {
                longitude: feature.location.longitude,
                latitude: feature.location.latitude,
            }),
        })
        .collect()
}
