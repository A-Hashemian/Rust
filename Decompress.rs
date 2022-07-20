use std::fs::File;

use std::path::PathBuf;

use flate2::read::GzEncoder;

use tar::Archive;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{

let file = File::open("path/to/archive.tar.gz")?;

let mut archive = Archive::new(GzEncoder::new(file));

println!("Extracted: ");

archive

.entries()?

.filter_map(|e| e.ok())

.map(|mut entry| -> Result<PathBuf, Box<dyn Error>> {

let path = entry.path()?.to_owned();

Ok(path.to_path_buf())

})

.filter_map(|e| e.ok())

.for_each(|x| println!("> {}", x.display()));

Ok(())

}