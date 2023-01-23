## Introduction
This is a code snippet save&&sync cli-tool based on rust. This tool can search/create/edit/delete/sync your snippets and should run on Linux/Windows/MacOS.
![rust snippet tool](https://github.com/find1dream/snipped_rustui/image/rust_snippet_tool.png)

## How to initialize the tool
1. create a database folder and init it with `touch .gitignore && git init && git add . && git commit -m "first commit" && git branch -M main`
2. create a remote repository from `github` or `gitlab`, then generate a token, say `https://github.com/your_git_name/snippet.git` with access token `hsbuoea4y.ub76mu`
3. add that remote repository with `git remote add origin https://your_name:hsbuoea4y.ub76mu@github.com/your_git_name/snippet.git`
4. install `rust`, clone this repository and run `cargo run`
5. input your database folder path, say `/mnt/c/Users/a1234567/snippet`, then the program will generate a `.env` file which saves it. You need to delete that file to regenerate it

## How to use
- `tab`: switch between `normal mode`, `search bar`, `snippet list`, `title bar`, `language bar`, `content area`
- `esc`: return to `normal mode` 

### `normal mode`
- create snippet with `ctrl-n`, and `enter` to edit `title` (then `tab` to edit `language` ...)
- delete snippet with `ctrl-d`
- save && sync all with `ctrl-s`
- `up` or `down` to navigate the list

### `search bar`
- input your text, and `enter`, then `esc` with `up` or `down` to navigate the filtered list
- delete the text with `backspace` or `ctrl-u`

### `snippet list`
- navigate snippets with `up` or `down`
- create, delete or sync because it's in `normal mode`

### `title bar`
- input your snippet title

### `language bar`
- input your snippet language

### `content area`
- edit the contents
- copy snippet with `ctrl-c`
- paste content with `ctrl-v` 
