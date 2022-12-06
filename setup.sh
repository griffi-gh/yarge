#!/bin/bash
#to be used by the CI scripts only to setup the environment!
echo '[WARN] to be used by the CI scripts only to setup the environment!'
sudo apt update
sudo apt install libsdl2-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev libatk1.0-dev gir1.2-gtk-3.0 libasound2-dev
