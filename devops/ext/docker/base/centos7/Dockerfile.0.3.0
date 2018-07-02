FROM centos:7
# TODO LABEL maintainer="Name <email-address>"

# generally useful packages
# TODO not sure that we need yum update
RUN yum install -y \
        curl \
        wget \
        vim \
        git \
        procps \
        autoconf \
        automake \
        g++ \
        gcc \
        make \
        pkgconfig \
        zip \
        unzip \
    && yum clean all

# install ruby
ENV RUBY_VERSION=2.3.0
RUN yum install -y \
            patch \
            bison \
            bzip2 \
            gcc-c++ \
            libffi-devel \
            libtool \
            patch \
            readline-devel \
            sqlite-devel \
            zlib-devel \
            libyaml-devel \
            openssl-devel \
    && gpg --keyserver hkp://keys.gnupg.net --recv-keys 409B6B1796C275462A1703113804BB82D39DC0E3 7D2BAF1CF37B13E2069D6956105BD0E739499BDB \
    && curl -sSL https://get.rvm.io | bash -s stable --ruby=$RUBY_VERSION \
    && yum clean all

# install fpm
ENV FPM_VERSION=1.9.3
RUN yum install -y \
        rpm-build \
    && /bin/bash -c -l "gem install --no-ri --no-rdoc fpm -v $FPM_VERSION" \
    && yum clean all

# TODO CMD ENTRYPOINT ...

ENV BASE_ENV_VERSION=0.3.0
