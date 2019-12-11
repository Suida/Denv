# Denv -- A Customized Development Environment Builder
-------

## What this builder will install for you

### Shell

As the most popular choice, *zsh* + *Oh-My-Zsh* combination will be performed.

#### Zsh

**Zsh** will be installed through default package manager.

#### Proxy

If `--require-proxy` is specified or the configuration file is set as:
```yaml
proxy:
  required: true
```
a proxy will be installed and set globally, thus all the network operations
below will be through it.

By default, *clash* is chosen as the client, which requires user to prepare a
*clash.yaml* under directory *~/.config/clash*.

#### Ohmyzsh

*Oh-My-Zsh* will be installed to set zsh. Extensions to add can be specified in
*config.yml*, like:
```yaml
zsh:
  extensions:
    - git
    - vscode
    ...
```
Also, a *~/bin* folder will be created and added to *$PATH* to hold user binaries.

### Python

`pyenv` and `pyenv-virtualenv` are to be installed for python environment.

### Go

The **Golang** installer follows the [installation guidline](https://golang.org/doc/install), but instead of directory */usr/local*, it's under *~/.go*.

### Rust

As indicated by [official recommandation](https://www.rust-lang.org/tools/install), this installer will use rustup as the installer as well, and the version to install is choosen by user interactively.

# But

All in one, there's even not any one function that has been implemented at all.
All the description is just a road map of this project. Don't be sad, I will
complete them.

***Really.***
