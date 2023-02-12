FROM ubuntu:22.04

USER root

# default settings
RUN apt-get update -y \
    # to prevent `debconf: (No usable dialog-like program is installed` prompt
    && apt-get install whiptail -y

RUN apt-get upgrade -y
RUN apt-get install -y \
    curl \
    wget \
    zsh \
    fzf

# Add user and group
ENV USER=ostep
ENV GROUP=ostep
ENV HOME_DIR=/home/$USER

RUN groupadd $GROUP
RUN useradd $USER \
    # create home
    -m \
    # specify directory
    -d /home/$USER \
    -p 12345 \
    -g $GROUP

# If want to set the user as sudoer
# RUN apt-get install sudo -y
# RUN usermod \
#     # append
#     -a \
#     # append the user to the supplemental GROUPS
#     # without removing the user from other groups
#     -G sudo $USER

# switch to the USER
USER $USER
# go to workdir
WORKDIR $HOME_DIR

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh \
    && chmod +x ./rustup-init.sh
RUN ./rustup-init.sh -y && rm -f ./rustup-init.sh
