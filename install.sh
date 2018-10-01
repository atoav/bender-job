#!/bin/bash
echo 'Starting to install bender-job'
echo

read -e -p "
Copy optimize_blend.py to /usr/local/lib/optimize_blend.py? [Y/n] " YN

[[ $YN == "y" || $YN == "Y" || $YN == "" ]] && sudo cp ./src/optimize_blend.py /usr/local/lib/optimize_blend.py
