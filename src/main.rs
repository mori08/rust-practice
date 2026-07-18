use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let path = ".";
    println!("path : {}", path);

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        println!("-----");
        println!("{}", entry.path().display());

        if let Ok(metadata) = entry.metadata()
            && let Ok(modified) = metadata.modified()
        {
            println!("{:?}", modified);
        }
    }

    Ok(())
}
