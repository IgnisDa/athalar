ARG VARIANT=18
FROM mcr.microsoft.com/vscode/devcontainers/javascript-node:0-${VARIANT}

RUN su node -c "npm install --global pnpm && npm config set store-dir ~/.store" ;\
    # global node tools and set the store directory
    curl https://raw.githubusercontent.com/IgnisDa/developrs/main/apps/esteem/install.sh | sh -s -- --yes ;\
    # to make it easier to manage nx monorepos
    wget -O tokei.tar.gz https://github.com/XAMPPRocky/tokei/releases/latest/download/tokei-x86_64-unknown-linux-gnu.tar.gz ;\
    tar xf tokei.tar.gz -C /usr/local/bin ;\
    tokei --version ;\
    # to count code lines count
    cargo install cargo-binstall
    # to install cargo binaries
    cargo binstall cargo-watch cargo-nextest --secure --no-confirm
