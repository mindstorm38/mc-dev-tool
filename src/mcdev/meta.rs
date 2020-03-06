use isahc::prelude::*;
use serde_json::Value;
use crate::version::{MinecraftVersion, MinecraftVersionType};

pub fn request_version_meta(version: &MinecraftVersion) -> Option<MinecraftMeta> {

    let meta: Value = isahc::get(&version.url)
        .unwrap()
        .json()
        .unwrap();

    let root = meta.as_object().unwrap();

    let assets = root["assets"].as_str().unwrap();
    let main_class = root["mainClass"].as_str().unwrap();
    let downloads = root["downloads"].as_object().unwrap();

    let client_downloads = MinecraftSideDownloads {
        jar: MinecraftDownload::parse_json(&downloads["client"]).unwrap(),
        mappings: MinecraftDownload::parse_json(downloads.get("client_mappings").unwrap_or(&Value::Null))
    };

    let server_downloads: Option<MinecraftSideDownloads> =
        if let Some(dl) = MinecraftDownload::parse_json(downloads.get("server").unwrap_or(&Value::Null)) {
            Some(MinecraftSideDownloads {
                jar: dl,
                mappings: MinecraftDownload::parse_json(downloads.get("server_mappings").unwrap_or(&Value::Null))
            })
        } else {
            None
        };

    let mut libraries: Vec<MinecraftLibrary> = Vec::new();

    for lib_raw in root["libraries"].as_array().unwrap() {

        let lib = lib_raw.as_object().unwrap();

        let name = lib["name"].as_str().unwrap();
        let url = lib["downloads"]["artifact"]["url"].as_str();

        libraries.push(MinecraftLibrary {
            name: name.into(),
            url: url.map_or(None, |r| Some(String::from(r)))
        });

    }

    Some(MinecraftMeta {
        id: version.id.clone(),
        typ: version.typ,
        assets: assets.into(),
        libraries,
        main_class: main_class.into(),
        client_downloads,
        server_downloads
    })

}

#[derive(Debug)]
pub struct MinecraftLibrary {
    pub name: String,
    pub url: Option<String>
}

#[derive(Debug)]
pub struct MinecraftDownload {
    pub size: usize,
    pub url: String
}

#[derive(Debug)]
pub struct MinecraftSideDownloads {
    pub jar: MinecraftDownload,
    pub mappings: Option<MinecraftDownload>
}

#[derive(Debug)]
pub struct MinecraftMeta {
    pub id: String,
    pub typ: MinecraftVersionType,
    pub assets: String,
    pub libraries: Vec<MinecraftLibrary>,
    pub main_class: String,
    pub client_downloads: MinecraftSideDownloads,
    pub server_downloads: Option<MinecraftSideDownloads>
}

impl MinecraftDownload {

    fn parse_json(raw: &Value) -> Option<Self> {

        if raw.is_object() {

            let root = raw.as_object().unwrap();

            Some(MinecraftDownload {
                size: root["size"].as_u64().unwrap() as usize,
                url: root["url"].as_str().unwrap().into()
            })

        } else {
            None
        }

    }

}