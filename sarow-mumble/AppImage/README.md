#Building AppImage in docker
In this directory:

`docker run --rm --network=host -v $PWD:/sarow-mumble -e BUILD_AS=$USER --privileged python:3.9 /sarow-mumble/appimage/build.sh`

AppImage will be located in /target/sarow-mumble.bin
