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

gzip: sources
	@echo "[....] Installing gzip..."
	apt install gzip
	@echo [ OK ] Gzip installed

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
	mkdir -p ~/bin
	echo 'export PATH=$$HOME/bin:$$PATH' >> ~/.zshrc
	@echo [ OK ] oh-my-zsh installed

proxy: oh-my-zsh gzip
	@echo "[....] Installing proxy"
	curl -fsSLo clash.gz https://github.com/Dreamacro/clash/releases/download/v0.16.0/clash-linux-amd64-v0.16.0.gz
	gzip -d clash.gz
	chmod 755 clash
	mv clash ~/bin/clash
	mkdir -p ~/.config/clash
	cp ./modules/proxy/clash/* ~/.config/clash
	export http_proxy=http://127.0.0.1:7890
	export https_proxy=http://127.0.0.1:7890
	echo "export http_proxy=http://127.0.0.1:7890" >> ~/.zshrc
	echo "export https_proxy=http://127.0.0.1:7890" >> ~/.zshrc
	@echo "[....] Proxy installed..."

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
