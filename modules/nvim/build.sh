#!/usr/bin/env bash
dir="$HOME/.config/nvim"
mkdir -p $dir

src="$(dirname $0)/init.vim"
dst="$dir/init.vim"
echo "->Configuring [newvim]..."
cp $src $dst
echo "<>Configured"
