use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, PartialEq)]
enum Notice {
    Add(PathBuf),
    Remove(PathBuf),
    Update(PathBuf),
}

fn scan(path: &str) -> Result<HashMap<PathBuf, SystemTime>, io::Error> {
    let mut map: HashMap<PathBuf, SystemTime> = HashMap::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        if let Ok(metadata) = entry.metadata()
            && let Ok(modified) = metadata.modified()
        {
            map.insert(entry.path(), modified);
        }
    }

    Ok(map)
}

fn diff(a: &HashMap<PathBuf, SystemTime>, b: &HashMap<PathBuf, SystemTime>) -> Vec<Notice> {
    let mut rtn: Vec<Notice> = Vec::new();

    for (path, time) in b {
        match a.get(path) {
            Some(modified) if modified != time => rtn.push(Notice::Update(path.clone())),
            None => rtn.push(Notice::Add(path.clone())),
            _ => {}
        }
    }

    for path in a.keys() {
        if !b.contains_key(path) {
            rtn.push(Notice::Remove(path.clone()));
        }
    }

    rtn
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn empty() {
        let a: HashMap<PathBuf, SystemTime> = HashMap::new();
        let b: HashMap<PathBuf, SystemTime> = HashMap::new();

        let r = diff(&a, &b);
        assert_eq!(r, Vec::new());
    }

    #[test]
    fn add_file() {
        let a: HashMap<PathBuf, SystemTime> = HashMap::new();
        let mut b: HashMap<PathBuf, SystemTime> = HashMap::new();

        b.insert(
            PathBuf::from("hoge.txt"),
            SystemTime::UNIX_EPOCH + Duration::from_secs(100),
        );

        let r = diff(&a, &b);
        assert_eq!(r, vec![Notice::Add("hoge.txt".into())]);
    }

    #[test]
    fn update_file() {
        let mut a: HashMap<PathBuf, SystemTime> = HashMap::new();
        let mut b: HashMap<PathBuf, SystemTime> = HashMap::new();

        a.insert(
            PathBuf::from("hoge.txt"),
            SystemTime::UNIX_EPOCH + Duration::from_secs(100),
        );
        b.insert(
            PathBuf::from("hoge.txt"),
            SystemTime::UNIX_EPOCH + Duration::from_secs(200),
        );

        let r = diff(&a, &b);
        assert_eq!(r, vec![Notice::Update("hoge.txt".into())]);
    }

    #[test]
    fn remove_file() {
        let mut a: HashMap<PathBuf, SystemTime> = HashMap::new();
        let b: HashMap<PathBuf, SystemTime> = HashMap::new();

        a.insert(
            PathBuf::from("hoge.txt"),
            SystemTime::UNIX_EPOCH + Duration::from_secs(100),
        );

        let r = diff(&a, &b);
        assert_eq!(r, vec![Notice::Remove("hoge.txt".into())]);
    }
}

fn main() -> Result<(), io::Error> {
    let path = std::env::args().nth(1).unwrap_or(".".to_string());
    println!("path : {}", path);

    let a = scan(&path)?;
    let b = scan(&path)?;
    let r = diff(&a, &b);

    println!("{:#?}", r);

    Ok(())
}
