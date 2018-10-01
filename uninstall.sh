#!/bin/bash
echo 'Starting to remove bender-job'
echo

Do you want to remove file at /usr/local/lib/optimize_blend.py ? [Y/n] " YN

    [[ $YN == "y" || $YN == "Y" || $YN == "" ]] && sudo rm -f /usr/local/lib/optimize_blend.py;
fi