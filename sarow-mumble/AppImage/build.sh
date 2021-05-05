#!/bin/sh

# Install dependencies
apt-get update -y
apt-get upgrade -y
apt-get install -y libfuse2
pip install Nuitka
pip install -r /sarow-mumble/requirements.txt

# Create AppImage
mkdir /sarow-mumble/target
cd /sarow-mumble/target
nuitka3 --python-flag=no_site \
        --python-flag=no_warnings \
        --onefile \
        --follow-imports \
        --assume-yes-for-downloads \
        "/sarow-mumble/src/sarow-mumble.py"

# Change to owner to current user
useradd "$BUILD_AS" --user-group
chown -R "$BUILD_AS":"$BUILD_AS" /sarow-mumble/target
