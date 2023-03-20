FROM purtontech/rust-on-nails-devcontainer:1.1.3 AS development

COPY .bash_aliases /home/vscode/.bash_aliases

COPY ps1.bash .
RUN cat ps1.bash >> ~/.bashrc && sudo rm ps1.bash

# Enable our git hooks.
RUN echo 'git config core.hooksPath /workspace/.devcontainer/.githooks' >> ~/.bashrc

# Zola for static content generation
ARG ZOLA_VERSION=0.15.3
RUN sudo curl -OL https://github.com/getzola/zola/releases/download/v$ZOLA_VERSION/zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    && sudo tar -xvf zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    && sudo mv zola /usr/bin/zola \
    && sudo chmod +x /usr/bin/zola

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

ARG USERNAME=vscode

# Install az
RUN sudo apt-get update --allow-releaseinfo-change && curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash

# Digital Ocean
RUN cd ~ \
    && curl -OL https://github.com/digitalocean/doctl/releases/download/v1.37.0/doctl-1.37.0-linux-amd64.tar.gz \
    && tar xvf doctl-1.37.0-linux-amd64.tar.gz \
    && sudo mv doctl /usr/bin \
    && rm doctl-1.37.0-linux-amd64.tar.gz

# download latest replibyte archive for Linux
RUN curl -L -s https://github.com/Qovery/Replibyte/releases/download/v0.10.0/replibyte_v0.10.0_x86_64-unknown-linux-musl.tar.gz | tar xvz -C /tmp && sudo mv /tmp/replibyte /usr/bin 
    
# all the volumes configured in the docker-compose.yml
RUN sudo mkdir -p /workspace/target && sudo chown $USERNAME:$USERNAME /workspace/target
RUN sudo mkdir -p /workspace/crates/asset-pipeline/node_modules && sudo chown $USERNAME:$USERNAME /workspace/crates/asset-pipeline/node_modules
