FROM catthehacker/ubuntu:act-latest

ENV ACTIONS_RUNNER_FORCE_ACTIONS_NODE_VERSION=node20

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y