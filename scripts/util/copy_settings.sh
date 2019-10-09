#!/bin/bash

cd $ARC_DIR;
rm ./res/vscode/*;
cp ~/Library/Application\ Support/Code/User/settings.json ./res/vscode
cp ~/Library/Application\ Support/Code/User/keybindings.json ./res/vscode
code --list-extensions | xargs -L 1 echo code --install-extension > ./res/vscode/extensions.txt
cd -;
