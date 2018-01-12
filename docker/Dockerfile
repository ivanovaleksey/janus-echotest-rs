FROM ubuntu:16.04

## -----------------------------------------------------------------------------
## Installing dependencies
## -----------------------------------------------------------------------------
ENV DEBIAN_FRONTEND noninteractive
RUN set -xe \
    && apt-get update \
    && apt-get -y --no-install-recommends install \
        software-properties-common \
        apt-transport-https \
        ca-certificates \
        lsb-release \
        nginx \
        telnet \
        curl \
        git \
        vim-nox

## -----------------------------------------------------------------------------
## Installing Janus
## -----------------------------------------------------------------------------
RUN set -xe \
    && apt-get update && apt-get -y --no-install-recommends install \
        automake \
        gdb \
        gengetopt \
        libmicrohttpd-dev \
        libnice-dev \
        libssl-dev \
        libsofia-sip-ua-dev \
        libglib2.0-dev \
        libopus-dev \
        libogg-dev \
        libtool \
    && JANSSON_BUILD_DIR='/tmp/libjansson' \
    && mkdir "${JANSSON_BUILD_DIR}" \
    && cd "${JANSSON_BUILD_DIR}" \
        && git clone https://github.com/akheron/jansson.git . \
        && autoreconf -i \
        && ./configure --prefix=/usr CFLAGS='-g -O0' \
        && make \
        && make install \
    && SRTP_BUILD_DIR='/tmp/srtp' \
    && mkdir "${SRTP_BUILD_DIR}" \
    && cd "${SRTP_BUILD_DIR}" \
        && curl -fsSL -o libsrtp.tar.gz "https://github.com/cisco/libsrtp/archive/v2.1.0.tar.gz" \
        && tar xf libsrtp.tar.gz \
        && cd libsrtp* \
        && ./configure --prefix=/usr --enable-openssl \
        && make shared_library && make install \
        && rm -fr "${SRTP_BUILD_DIR}" \
    && USRSCTP_BUILD_DIR='/tmp/usrsctp' \
    && mkdir "${USRSCTP_BUILD_DIR}" \
    && cd "${USRSCTP_BUILD_DIR}" \
        && git clone https://github.com/sctplab/usrsctp . \
        && ./bootstrap \
        && ./configure --prefix=/usr \
        && make -j $(nproc) \
        && make install \
        && rm -fr "${USRSCTP_BUILD_DIR}" \
    && cd /tmp \
        && curl -fsSL -o cmake.tar.gz  "https://cmake.org/files/v3.9/cmake-3.9.6-Linux-x86_64.tar.gz" \
        && mkdir cmake-3.9.6 \
        && tar xf cmake.tar.gz -C cmake-3.9.6 --strip-components=1 \
        && mv cmake-3.9.6 /opt \
    && WEBSOCKETS_BUILD_DIR='/tmp/libwebsockets' \
    && mkdir "${WEBSOCKETS_BUILD_DIR}" \
    && cd "${WEBSOCKETS_BUILD_DIR}" \
        && git clone https://github.com/warmcat/libwebsockets.git . \
        && mkdir build \
        && cd build \
        && /opt/cmake-3.9.6/bin/cmake -DLWS_MAX_SMP=1 -DCMAKE_INSTALL_PREFIX:PATH=/usr -DCMAKE_C_FLAGS="-fpic" .. \
        && make \
        && make install \
    && JANUS_BUILD_DIR='/tmp/janus' \
    && mkdir "${JANUS_BUILD_DIR}" \
    && cd "${JANUS_BUILD_DIR}" \
        && git clone https://github.com/meetecho/janus-gateway.git . \
        && git checkout -b refcount origin/refcount \
        && ./autogen.sh \
        && ./configure --prefix=/opt/janus --enable-websockets --disable-rabbitmq --disable-mqtt --disable-all-plugins enable_plugin_echotest=yes\
        && make || true \
        && make -j $(nproc) \
        && make install \
        && make configs

## -----------------------------------------------------------------------------
## Installing Gstreamer
## -----------------------------------------------------------------------------
# RUN set -xe \
#     && add-apt-repository -y "deb https://archive.ubuntu.com/ubuntu/ $(lsb_release -sc) multiverse" \
#     && add-apt-repository -y "deb https://archive.ubuntu.com/ubuntu/ $(lsb_release -sc)-updates multiverse" \
#     && add-apt-repository -y "deb https://archive.ubuntu.com/ubuntu/ $(lsb_release -sc)-security multiverse" \
#     && apt-get update && apt-get -y --no-install-recommends install \
#         gstreamer1.0-tools \
#         gstreamer1.0-libav \
#         gstreamer1.0-plugins-good \
#         gstreamer1.0-plugins-bad

## -----------------------------------------------------------------------------
## Configuring Janus
## -----------------------------------------------------------------------------
RUN set -xe \
    && perl -pi -e 's/(http = ).*/${1}no/' /opt/janus/etc/janus/janus.transport.http.cfg \
    && perl -pi -e 's/(https = ).*/${1}yes/' /opt/janus/etc/janus/janus.transport.http.cfg \
    && perl -pi -e 's/(wss = ).*/${1}yes/' /opt/janus/etc/janus/janus.transport.websockets.cfg

## -----------------------------------------------------------------------------
## Configuring Janus Examples
## -----------------------------------------------------------------------------
RUN set -xe \
    && echo "\
        server {\n\
            listen 8443 ssl;\n\
            ssl_certificate /opt/janus/share/janus/certs/mycert.pem;\n\
            ssl_certificate_key /opt/janus/share/janus/certs/mycert.key;\n\
            location / {\n\
                root /opt/janus/share/janus/demos;\n\
            }\n\
        }\n\
    " > /etc/nginx/sites-enabled/janus

## -----------------------------------------------------------------------------
## Installing Rust
## -----------------------------------------------------------------------------
RUN set -xe \
    && cd /tmp \
    && curl https://sh.rustup.rs -o rustup.sh \
    && chmod +x rustup.sh \
    && ./rustup.sh -y

COPY bash_aliases /root/.bash_aliases
