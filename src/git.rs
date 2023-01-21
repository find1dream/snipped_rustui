use std::process::Command;

pub fn git_add_all(path: &str) {
    Command::new("git")      
            .arg("-C")
            .arg(path)
            .arg("add")
            .arg(".")
            .output()
            .expect("git add . failed");
}

pub fn git_commit(path: &str, message: &str) {
    Command::new("git")      
            .arg("-C")
            .arg(path)
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .expect("git commit failed");
}

pub fn git_push(path: &str) {
    Command::new("git")      
            .arg("-C")
            .arg(path)
            .arg("push")
            .arg("origin")
            .arg("main")
            .output()
            .expect("git commit failed");
}

pub fn git_pull(path: &str) {
    Command::new("git")      
            .arg("-C")
            .arg(path)
            .arg("pull")
            .arg("origin")
            .arg("main")
            .output()
            .expect("git commit failed");
}
