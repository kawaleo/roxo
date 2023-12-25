# Roxo

[roxo](https://www.github.com/kawaleo/roxo) is designed to replace commands such as _ls_ as _dir_

## Installation

roxo works on all operating systems. In order to get started, clone the repo, and make sure you have `rust` installed.

If you are unsure if `rust` is installed on your system, run

```shell
cargo
```

You should see a list of commands, and if not, `rust` can be found [here](https://www.rust-lang.org)

To continue with installation, go into the cloned repo and run

```shell
cargo build
```

Put the `.exe` or `.pdb` file (depending on operating system) in your PATH Environment Variables and now you should be able to run roxo anywhere!

## Options

roxo gives a variety of options, including ways to sort how files are listed

### Filtering options

- **-a**, **--all**: show hidden files

### Sorting Options

- **-s**, **--size**: sort by size
- **-t**, **--time**: sort by time of last modification

## Todo

- [x] Compatability with all operating systems
- [ ] config.toml file
- [ ] Add support for really long file/folder names

## More info here eventually.
