use std::fs;
use std::io::{Read, Write};
use std::result::Result;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::Path;
use serde::{Serialize, Deserialize};
extern crate serde_json;

type EResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub file_path: String,
    pub title: String,
    pub language: String,
    pub contents: String,
}

impl Clone for Note {
    fn clone(&self) -> Self {
        Note{
            file_path: self.file_path.clone(),
            title: self.title.clone(),
            language: self.language.clone(),
            contents: self.contents.clone()
        }
    }
}

#[allow(dead_code)]
#[allow(unused_must_use)]
impl Note {
    const EXTENSION: &str = ".md";

    pub fn new(base_path: &str, language: &str, title: &str, contents: &str) -> Self {
        let new_path = Path::new(base_path).join(language).join([title, Note::EXTENSION].join(""));
        let final_path = match new_path.to_str() {
            Some(s) => s,
            None => panic!("New path is not a valid UTF-8 sequence")
        };
        Note {
            file_path: String::from(final_path),
            language: String::from(language),
            title: String::from(title),
            contents: String::from(contents)
        }
    }

    fn check_file_exist(&self) -> bool {
        Path::new(&self.file_path).exists()
    }

    pub fn delete(&self) -> EResult<()> {
        match self.check_file_exist() {
            true => {
                fs::remove_file(&self.file_path).expect("File delete failed");
                Ok(())
            },
            false => Err("File not exist".into())
        }
    }

    pub fn save(&self) -> EResult<String> {
        self.delete();
        let path = Path::new(&self.file_path);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        let mut f = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .open(&self.file_path)?;

        match f.write_all(serde_json::to_string(&self)?.as_bytes()) {
            Ok(_) => Ok(String::from(&self.file_path)),
            Err(_) => panic!("write error")
        }
    }
    
    pub fn load(path: &str) -> EResult<Self> {
        match Path::new(path).exists() {
            true  => {},
            false => panic!("File not existed, load failed!")
        }
        let mut file = File::open(path)?;
        let mut buf = vec![];
        match file.read_to_end(&mut buf).is_ok() {
            true => {
                let note = serde_json::from_slice::<Note>(&buf[..])?;
                Ok(Note{
                    file_path: note.file_path,
                    language: note.language,
                    title: note.title,
                    contents: note.contents
                })
            },
            false => Err("load error".into())
        }
    }
}

#[cfg(test)]
mod test{
    use super::Note;

    fn create_new_object(title: &str) -> Note {
        let file_path = "./target/temp";
        let language = "python";
        let title = title;
        let contents = "print('hello world')";
        Note::new(file_path, 
             language, 
             title, 
             contents
        )
    }

    fn save_a_object(note: &Note) -> &str {
        let rst: bool;
        match note.save() {
            Ok(_)  => &note.file_path,
            Err(_) => "",
        }
    }

    #[test]
    fn note_new_object() {
        let file_path = "./target/temp/python/hello.md";
        let language = "python";
        let title = "hello";
        let contents = "print('hello world')";
        let note = create_new_object(title);
        assert_eq!(note.file_path, file_path);
        assert_eq!(note.language, language);
        assert_eq!(note.title, title);
        assert_eq!(note.contents, contents);
    }

    #[test]
    fn note_should_be_created() {
        let note = create_new_object("python");
        let path = save_a_object(&note);
        assert!(path.len() > 0);
    }

    #[test]
    fn note_should_be_loaded() {
        let note = create_new_object("other");
        let loaded = match Note::load(save_a_object(&note)){
            Ok(nt)  => nt,
            Err(_) => panic!("load error")
        };
        assert_eq!(note.file_path, loaded.file_path);
    }

    #[test]
    fn note_should_be_deleted() {
        let note = create_new_object("anathor");
        assert_eq!(note.check_file_exist(), false);
        let path = save_a_object(&note);
        assert_eq!(note.check_file_exist(), true);
        let rst: bool;
        match note.delete() {
            Ok(_)  => rst = true,
            Err(_) => rst = false
        }
        assert_eq!(rst, true);
    }
}
