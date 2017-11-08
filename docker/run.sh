#!/bin/bash

PROJECT='janus-echotest-rs'
PROJECT_DIR="/opt/sandbox/${PROJECT}"
DOCKER_CONTAINER_NAME="sandbox/${PROJECT}"
DOCKER_CONTAINER_COMMAND=${DOCKER_CONTAINER_COMMAND:-'/bin/bash'}
DOCKER_RUN_OPTIONS=${DOCKER_RUN_OPTIONS:-'-ti --rm'}

read -r DOCKER_RUN_COMMAND <<-EOF
    source ~/.profile \
    && ln -s /opt/janus/bin/janus /usr/local/bin/janus \
    && service nginx start \
    && janus
EOF

docker volume create janus-echotest-rs-cargo
docker build -t ${DOCKER_CONTAINER_NAME} docker/
docker run ${DOCKER_RUN_OPTIONS} \
    -v $(pwd):${PROJECT_DIR} \
    -v janus-echotest-rs-cargo:/root/.cargo \
    -p 8443:8443 \
    -p 8089:8089 \
    -p 5002:5002/udp \
    -p 5004:5004/udp \
    ${DOCKER_CONTAINER_NAME} \
    /bin/bash -c "set -x && cd ${PROJECT_DIR} && ${DOCKER_RUN_COMMAND} && set +x && ${DOCKER_CONTAINER_COMMAND}"
