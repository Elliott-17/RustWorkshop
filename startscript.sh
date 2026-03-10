#!/usr/bin/env bash

cd ~/Desktop


set -e

performinstall=true
performclone=true

### Check if rustlings installed
if command -v rustlings >/dev/null 2>&1; then
    echo "Rustlings already installed."
    performinstall=false
fi

### Install Rustlings
if [ "$performinstall" = true ]; then
    echo "Installing Rustlings..."
    cargo install rustlings >/dev/null 2>&1
    user=$(whoami)
    export PATH="/home/$user/.cargo/bin:$PATH"
    echo "Installed"
fi


### Check if in git tree
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    echo "Already inside a git repository."
    performclone=false
else
    echo "Not inside a git repository."
fi

### Check if already cloned
if [ -d "RustWorkshop" ]; then
    echo "RustWorkshop directory already exists."
    performclone=false
fi

### If not cloned and we arent in a git tree
### Then Clone it
if [ "$performclone" = true ]; then
    echo "Cloning Rustlings workshop..."
    git clone https://github.com/Elliott-17/RustWorkshop RustWorkshop >/dev/null 2>&1
fi

### Open VS Code in the ./RustWorkshop/exercises folder
if [ -d "RustWorkshop/exercises" ]; then
    echo "Opening VS Code..."
    code RustWorkshop/exercises
fi

### Clear this command prompt and run rustlings here

printf '\e[8;50;185t'
cd RustWorkshop
clear
rustlings