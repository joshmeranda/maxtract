use std::{
    env,
    string::String
};

/// Get the file url for the test pages found in ./tests/pages.
pub fn get_uri() -> String {
    format!("file://{}/tests/pages/index.html", env::current_dir().unwrap().to_str().unwrap())
}
