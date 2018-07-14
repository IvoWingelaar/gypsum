use uni_app;

pub fn new(path: &str) -> uni_app::fs::File {
    let path = if cfg!(target_arch = "wasm32") {
        path.to_owned()
    } else {
        "static/".to_owned() + path
    };

    uni_app::fs::FileSystem::open(&path).unwrap()
}
