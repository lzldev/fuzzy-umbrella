use std::io::Write;

#[test]
fn generate_index_binding() {
    if !std::path::Path::new("./bindings").exists() {
        panic!("Bindings directory not found");
    }

    let mut index = std::fs::File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./bindings/index.ts")
        .expect("To open index.ts file");

    let dir = std::fs::read_dir("./bindings").expect("To Read dir");

    dir.map(|d| d.expect("To unwrap DirEntry"))
        .filter(|file| !file.file_name().eq("index.ts".into()))
        .for_each(|file| {
            let full_filename = file
                .file_name()
                .to_str()
                .expect("To unwrap filename.")
                .to_owned();

            let (filename, end) = full_filename.rsplit_once('.').unwrap();

            eprintln!("fs : {},{}", filename, end);

            let _ = &index
                .write(format!("export * from './{}';\n", filename).as_bytes())
                .expect("To write buffer into file.");
        });
}
