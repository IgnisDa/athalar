ARG VARIANT=18
FROM mcr.microsoft.com/vscode/devcontainers/javascript-node:0-${VARIANT}

# global node tools and set the store directory
RUN su node -c "npm install --global pnpm && npm config set store-dir ~/.store" ;\
    # to make it easier to manage nx monorepos
    curl https://raw.githubusercontent.com/IgnisDa/developrs/main/apps/esteem/install.sh | sh -s -- --yes ;\
    # to count code lines count
    wget -O tokei.tar.gz https://github.com/XAMPPRocky/tokei/releases/latest/download/tokei-x86_64-unknown-linux-gnu.tar.gz ;\
    tar xf tokei.tar.gz -C /usr/local/bin ;\
    tokei --version ;
