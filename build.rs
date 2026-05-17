use std::{fs, path::Path};

fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    let bin_dir = Path::new(&out).ancestors().nth(3).unwrap().to_path_buf();

    let src = Path::new("res");
    let dst = bin_dir.join("res");

    copy_dir(src, &dst);

    println!("cargo:rerun-if-changed=res");
}

fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let dst_path = dst.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &dst_path);
        } else {
            fs::copy(entry.path(), dst_path).unwrap();
        }
    }
}
