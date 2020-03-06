
use std::collections::HashMap;

use serde_json::Value;

use isahc::prelude::*;
use isahc::ResponseExt;

const MC_VERSION_MANIFEST: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

fn main() {

    let versions = request_versions();
    dbg!(versions);

}

fn request_versions() -> Option<HashMap<String, String>> {

    let manifest: Value = isahc::get(MC_VERSION_MANIFEST)
        .unwrap()
        .json()
        .unwrap();

    let versions = &manifest["versions"];
    let mut ret: HashMap<String, String> = HashMap::new();

    if let Value::Array(versions) = versions {
        for version in versions {

            if let Value::Object(version) = version {

                if let Some(Value::String(id)) = version.get("id") {
                    if let Some(Value::String(url)) = version.get("url") {
                        ret.insert(String::clone(id), String::clone(url));
                    }
                }

            }

        }
    }

    Some(ret)

}