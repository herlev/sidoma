# sidoma

> **Si**mple **Do**tfile **Ma**nager

## What does this application do?
### Automatic linking
By default _sidoma_ will create symlinks from all directories in the dotfile directory to `$XDG_CONFIG_HOME`.

```
~/
└── dotfiles/
    ├── nvim/
    ├── bspwm/
    └── polybar/
```
After running `sidoma links create`, the following symlinks have been created in `.config`:
```
~/
└── .config/
    ├── nvim    ➔ ../dotfiles/nvim/
    ├── bspwm   ➔ ../dotfiles/bspwm/
    └── polybar ➔ ../dotfiles/polybar/
```

### What if there are some folders I don't want linked to `~/.config`?
To override the default behavior create a `.link` file inside any directory. An empty link file will result in the folder being ignored.
#### Manual linking
The `.link` file can be used to specify symlinks manually, overwriting the default behavior.
```
$ cat ~/dotfiles/bash/.link
bashrc ~/.bashrc
```
The following directory structure:
```
~/
└── dotfiles/
    └── bash/
        ├── bashrc
        └── .link
```
After running `sidoma link create`:
```
~/
└── .bashrc ➔ dotfiles/bash/bashrc
```

### Getting started

Tell _sidoma_ where your dotfiles are located:

`sidoma init <path to dotfile dir>`

## Features

- Automatic linking of configuration files

### TODO
- Installation of packages
- Built-in template engine based on [tera](https://github.com/Keats/tera)

