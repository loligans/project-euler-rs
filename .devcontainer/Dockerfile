ARG RUST_VERSION="latest"
FROM mcr.microsoft.com/vscode/devcontainers/rust:${RUST_VERSION}

RUN cat /etc/passwd
ARG USERNAME="coder"
RUN usermod -l ${USERNAME} vscode
RUN groupmod -n ${USERNAME} vscode
RUN usermod -d /home/${USERNAME} -m ${USERNAME}

ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN wget -O- https://aka.ms/install-vscode-server/setup.sh | sh

# Add sudo support
RUN apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# Add ZSH and Vim
RUN apt-get update \
    && apt-get install -y zsh vim \
    && chsh -s $(which zsh) \
    && cp ~/.oh-my-zsh/templates/zshrc.zsh-template ~/.zshrc \
    && chsh -s $(which zsh) $USERNAME

# Add required dependencies
RUN apt-get update \
    && apt-get install -y clang zlib1g-dev

# Set the default user
USER $USERNAME

# Use ZSH
RUN cp ~/.oh-my-zsh/templates/zshrc.zsh-template ~/.zshrc \
    && sudo chsh -s $(which zsh) $USERNAME

COPY postCreate.sh /usr/local/share/
RUN sudo chmod +x /usr/local/share/postCreate.sh
