sources:
	@echo "[....] Changing source file to tsinghua the mirror one..."
	mv /etc/apt/sources.list /etc/apt/sources.list.bkp
	cp modules/sources/tsinghua.apt.sources.list /etc/apt/sources.list
	apt update
	@echo [ OK ] Source file changed

git: sources
	@echo "[....] Installing git..."
	apt install git
	@echo [ OK ] Git installed

curl: sources
	@echo "[....] Installing curl..."
	apt install curl
	@echo [ OK ] Curl installed

# Replace the old shell with zsh
shell: zsh oh-my-zsh proxy
	@echo [ OK ] Zsh shell configured

zsh: git
	@echo "[....] Installing Zsh..."
	apt install zsh
	@echo [ OK ] Zsh installed

oh-my-zsh: zsh curl
	@echo "[....] Installing oh-my-zsh..."
	sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
	@echo [ OK ] oh-my-zsh installed

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
