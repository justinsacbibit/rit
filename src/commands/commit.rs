fn packed_hash(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.input(&input);
    let mut vec: Vec<u8> = Vec::with_capacity(20);
    for _ in 0..20 {
        vec.push(0);
    }
    hasher.result(&mut vec);
    vec
}

fn blob(blob_path: &path::PathBuf, metadata: &fs::Metadata) -> Vec<u8> {
    let mut file = File::open(blob_path).unwrap();
    let mut data: Vec<u8> = Vec::new();
    let mut header: Vec<u8> = format!("blob {}\0", metadata.len()).bytes().collect();
    data.append(&mut header);
    file.read_to_end(&mut data).ok();
    let packed = packed_hash(&data);
    packed
}

fn tree(p: &path::PathBuf) -> Vec<u8> {
    let mut s: Vec<u8> = Vec::new();
    for entry in fs::read_dir(p).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        // TODO: Use ignore file
        if entry_path.ends_with(".redox") || entry_path.ends_with(".git") || entry_path.ends_with("target") {
            continue;
        }

        let metadata = fs::metadata(&entry_path).unwrap();
        let f_str = entry.file_name();
        let name = f_str.to_str().unwrap();
        let permissions = metadata.permissions();
        if metadata.is_dir() {
            let mut tree_hash = tree(&entry_path);
            let mut t: Vec<u8> = format!("40000 {0}\0", name).bytes().collect();
            t.append(&mut tree_hash);
            s.append(&mut t);
        } else if metadata.is_file() {
            let mode = permissions.mode();
            let mut blob_hash = blob(&entry_path, &metadata);
            let mode_str = match mode & 0o111 {
                0 => "100644",
                _ => "100755",
            };
            let mut f: Vec<u8> = format!("{} {}\0", mode_str, name).bytes().collect();
            f.append(&mut blob_hash);
            s.append(&mut f);
        }
    }
    let mut header: Vec<u8> = format!("tree {}\0", s.len()).bytes().collect();
    let mut data: Vec<u8> = Vec::new();
    data.append(&mut header);
    data.append(&mut s);
    let packed = packed_hash(&data);

    let tree_hash = hash_to_string(&packed);
    let folder = &tree_hash[0..2];
    let folder_path = format!(".redox/objects/{}", folder);
    let folder_meta = fs::metadata(&folder_path);
    match folder_meta {
        Ok(m) => {
            if m.is_dir() {
                create_file(&tree_hash, &data);
            } else {
                exit!("{} exists but it's not a directory", folder_path);
            }
        }
        Err(e) => {
            match e.raw_os_error() {
                Some(2) => {
                    match fs::create_dir(&folder_path) {
                        Ok(_) => {
                            info!("<commit>: Created {} directory", folder_path);

                            create_file(&tree_hash, &data);
                        },
                        Err(e) => {
                            exit!("An error occurred when creating a {} directory: {}", folder_path, e);
                        }
                    }
                }
                Some(_) | None => {
                    exit!("An error occurred checking for a {} directory: {}", folder_path, e);
                }
            }
        }
    }

    packed
}

fn create_file(hash: &str, raw_content: &Vec<u8>) {
    let folder = &hash[0..2];
    let file = &hash[2..];
    let path = format!(".redox/objects/{}/{}", folder, file);
    let meta = fs::metadata(&path);
    match meta {
        Ok(m) => {
            if m.is_file() {
                // do nothing
            } else {
                exit!("{} exists but it's not a file", path);
            }
        }
        Err(e) => {
            match e.raw_os_error() {
                Some(2) => {
                    let mut e = ZlibEncoder::new(Vec::new(), Compression::Best);
                    e.write(&raw_content).ok();
                    let compressed = e.finish();
                    let mut f = File::create(path).unwrap();
                    f.write_all(&compressed.unwrap()).ok();
                }
                Some(_) | None => {
                    exit!("An error occurred checking for a {} file: {}", path, e);
                }
            }
        }
    }
}

fn hash_to_string(input: &Vec<u8>) -> String {
    let slice = &input;
    slice.to_hex()
}

fn pretty_tree(p: &path::PathBuf) -> String {
    let mut s = String::new();
    for entry in fs::read_dir(p).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();

        // TODO: Use ignore file
        if entry_path.ends_with(".redox") || entry_path.ends_with(".git") || entry_path.ends_with("target") {
            continue;
        }

        let metadata = fs::metadata(&entry_path).unwrap();
        let f_str = entry.file_name();
        let name = f_str.to_str().unwrap();
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        if metadata.is_dir() {
            let raw = tree(&entry_path);
            let tree_hash = hash_to_string(&raw);
            let t = format!("040000 {} {}\n", &tree_hash, name);
            s.push_str(&t);
        } else if metadata.is_file() {
            let raw = blob(&entry_path, &metadata);
            let file_hash = hash_to_string(&raw);
            let mode_str = match mode & 0o111 {
                0 => "100644",
                _ => "100755",
            };
            let f = format!("{} {} {}\n", mode_str, &file_hash, name);
            s.push_str(&f);
        }
    }
    s
}

struct CommitCommand;

impl CommitCommand {
    fn execute(self) {
        let current_dir = env::current_dir().unwrap();
        let tr = pretty_tree(&current_dir);
        println!("{}", tr);
    }
}
