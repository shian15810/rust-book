# https://github.com/devcontainers/images/pull/960
FROM mcr.microsoft.com/devcontainers/base:bookworm

# https://github.com/dependabot-fixtures/docker-with-source/blob/main/Dockerfile
LABEL org.opencontainers.image.source="https://github.com/shian15810/rust-book"
LABEL org.opencontainers.image.description="Rust Book"

# https://github.com/microsoft/vscode-remote-release/issues/7690
RUN mkdir -p /home/vscode/.vscode-server/extensions \
    && chown -R vscode:vscode /home/vscode/.vscode-server

# https://github.com/devcontainers/images/blob/main/src/rust/.devcontainer/Dockerfile
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    # Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
    && apt-get purge -y imagemagick imagemagick-6-common \
    && apt-get autoremove -y && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# https://github.com/devcontainers/templates/blob/main/src/rust-postgres/.devcontainer/Dockerfile
# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends clang lld \
    && apt-get autoremove -y && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# [Optional] Uncomment this section to install additional OS packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here> \
#     && apt-get autoremove -y && apt-get clean -y \
#     && rm -rf /var/lib/apt/lists/*

# [Optional] Uncomment this line to install global node packages.
# RUN su vscode -c "source /usr/local/share/nvm/nvm.sh && npm install -g <your-package-here>" 2>&1
