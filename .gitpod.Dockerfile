FROM gitpod/workspace-full:2022-05-08-14-31-53


RUN sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys CE7709D068DB5E88 && \
    sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb bionic stable" && \
    sudo apt update && \
    sudo apt-get install libindy -y && \
    export LIBINDY_DIR='/usr/lib/'