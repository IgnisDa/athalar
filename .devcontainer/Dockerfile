
FROM archlinux

ENV UUID=1000
ENV GUID=${UUID}
ENV USERNAME=vscode

RUN pacman -Syyu --noconfirm reflector ;\
    reflector --latest 20 --protocol https --sort rate --save /etc/pacman.d/mirrorlist ;\
    pacman -S --noconfirm --needed base-devel git sudo

RUN groupadd -g ${GUID} ${USERNAME} ;\
    useradd -ms /bin/bash ${USERNAME} -u ${UUID} -g ${USERNAME} -g wheel ;\
    echo "ALL ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${USERNAME} ;\
    chmod 0440 /etc/sudoers.d/${USERNAME}

RUN pacman -S --noconfirm rustup nodejs npm python3 fish curl wget python-poetry

RUN printf '#!/bin/sh\nexport PATH=$PATH:$HOME/.cargo/bin\n' >> /etc/profile.d/cargo.sh ;\
    chmod +x /etc/profile.d/cargo.sh

RUN set -eu ;\
    cd /tmp ;\
    npm install -g pnpm ;\
    wget "https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz" -O "cargo-binstall.tgz" ;\
    tar zxvf "cargo-binstall.tgz" ;\
    mv "cargo-binstall" "/usr/bin/cargo-binstall" ;\
    wget "https://github.com/Morganamilo/paru/releases/download/v1.11.2/paru-v1.11.2-x86_64.tar.zst" ;\
    tar -xvf "paru-v1.11.2-x86_64.tar.zst" ;\
    mv "paru" "/usr/bin/paru"

USER $USERNAME

RUN set -eu ;\
    cd /tmp ;\
    npm config set store-dir "$HOME/.store" ;\
    rustup default stable ;\
    cargo binstall cargo-nextest cargo-watch cargo-expand --secure --no-confirm
