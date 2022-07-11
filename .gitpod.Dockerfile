FROM gitpod/workspace-full


RUN sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys CE7709D068DB5E88 && \
    sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb bionic stable" && \
    sudo apt update && \
    sudo apt install libindy && \
    export LIBINDY_DIR='/usr/lib/'