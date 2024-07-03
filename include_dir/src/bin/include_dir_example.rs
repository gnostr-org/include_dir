use include_dir::{include_dir, Dir};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR");

fn main() {
    let lib_rs = PROJECT_DIR.get_file("src/lib.rs").unwrap();
    let body = lib_rs.contents_utf8().unwrap();
    assert!(body.contains("globs"));
    //#[cfg(feature = "metadata")]
    #[allow(unused_variables)]
    //let glob: &str = "**/*.rs";
    let glob: &str = "**";
    #[cfg(feature = "glob")]
    for entry in PROJECT_DIR.find(&glob).unwrap() {
        println!("Found {}", entry.path().display());
        print!(
            "16:{:?}\n",
            PROJECT_DIR.get_file(format!("{}", entry.path().display()))
        );
        let file = PROJECT_DIR.get_file(format!("{}", entry.path().display()));
        print!("{:?}\n", Some(file).unwrap());
        //let body = Some(lib_rs.expect("REASON").contents_utf8());

        if file.is_none() {
            print!("file.is_none()={}\n", file.is_none());
        } else {
            //let file_contents = Some(file.expect("REASON").contents_utf8().unwrap());
            let mut file_contents = Some(file.expect("REASON").contents_utf8());
            if file_contents.is_none() {
                print!("file_contents.is_none()={}\n", file_contents.is_none());
            } else {
                print!("file_contents.is_none()={}\n", file_contents.is_none());
		file_contents = Some(Some(lib_rs.contents_utf8().unwrap()));
            }
        }
        //let body = lib_rs.expect("REASON").contents_utf8().unwrap();
        //print!("{:?}",Some(body).unwrap());
        //let entry_body: &str = PROJECT_DIR.get_file(entry.path().display().unwrap());
        //let body = entry.contents_utf8().unwrap();
    }
}
