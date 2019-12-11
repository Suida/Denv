sources:
	# TODO: sources

git: sources
	# TODO: git

# Replace the old shell with zsh
shell: zsh oh-my-zsh proxy
	# TODO: shell

zsh: git
	# TODO: Zsh

oh-my-zsh: zsh
	# TODO: oh-my-zsh

proxy: oh-my-zsh
	# TODO: proxy

# Build python environment
python: pyenv pyenv-virtualenv
	# TODO: python-env

pyenv: shell
	# TODO: pyenv

pyenv-virtualenv: pyenv
	# TODO: pyenv-virtualenv

# Golang
golang: shell
	# TODO: golang

# Rust-lang
rust: shell
	# TODO: rust-lang

# Docker
docker: shell
	# TODO: docker

docker-mysql: docker
	# TODO: mysql running in docker

docker-mongo: docker
	# TODO: mongodb running in docker

docker-postgres: docker
	# TODO: postgresql running in docker
