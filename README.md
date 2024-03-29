**G**it br**A**nch **IN**form**ER**

![TEST](https://github.com/gwirn/gainer/actions/workflows/build_test.yml/badge.svg)
[![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](/LICENSE.md)

Shows you on which git branch you are currently on

to replace 
```
parse_git_branch() {
    git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/ (\1)/'
}
```
in .bashrc or .zshrc in the PS1 

## (colored) zsh example
`PS1='%~'$'%B%F{166}$(/PATH/TO/GAINER)%f%b %B%F{3} %f%b '`

## (colored) bash example
`PS1=$'\w \[\033[1;31m\]$(/PATH/TO/GAINER) \[\e[0m\] '`

## sample output
`~/Code/rust_projects/gainer🌿 master:`
### detached HEAD
`~/Code/rust_projects/gainer🌿 SHA-1:`

 
## Description
It searches for a .git/HEAD file to obtain the current branch information.
It intentionally omits any error messages if a required component is not found, so the function's error does not need to be redirected using '2> /dev/null', and instead simply returns. 

## Alternative use
Can also be used like any other program in the directory of interest like `./PATH/TO/GAINER`
