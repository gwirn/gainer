use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// A script to replace
/// parse_git_branch() {
///     git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/ (\1)/'
/// }
/// in .bashrc or .zshrc in the PS1
///
/// *zsh example*
/// PS1='%~'$'%B%F{166}$(/PATH/TO/GAINER)%f%b %B%F{3} %f%b '
/// *bash example*
/// PS1=$'\w \033[1;31m\]$(/PATH/TO/GAINER) \033[1;34m\] \e[0m '
///  
/// The function searches for a .git/HEAD file to obtain the current branch information.
/// It intentionally omits any error messages if a required component is not found, so
/// the function's error does not need to be redirected using '2> /dev/null', and instead simply returns.
///
fn main() {
    // get path of the current directory
    let mut cur_loc = match env::current_dir() {
        Ok(cloc) => cloc,
        Err(..) => return,
    };
    let cur_loc_inmut = cur_loc.clone();
    //  iteratively remove one component of the path and check if .git/HEAD is present
    for _c in cur_loc_inmut.components() {
        let file_path: PathBuf = [cur_loc.clone(), ".git".into(), "HEAD".into()]
            .iter()
            .collect();
        if file_path.exists() {
            // read symbolic reference to branch currently on
            let contents = match fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(..) => return,
            };
            let content_path: &Vec<&str> = &contents.split(' ').collect();
            // get filename and remove newline so it can be printed without disturbing PS1
            let branch_name = match Path::new(&content_path[content_path.len() - 1]).file_name() {
                Some(name) => match name.to_str() {
                    Some(namestring) => match namestring.strip_suffix('\n') {
                        Some(final_name) => final_name,
                        None => return,
                    },
                    None => return,
                },
                None => return,
            };
            // print the branch name with indicating branch emoji
            print!("\u{1F33F} {branch_name}");
            break;
        }
        cur_loc.pop();
    }
}
