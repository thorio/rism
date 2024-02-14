FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN apt-get update && \
	apt-get install libappindicator3-dev libgtk-3-dev librsvg2-dev libssl-dev libjavascriptcoregtk-4.0-dev libsoup2.4-dev libwebkit2gtk-4.0-dev -y --no-install-recommends && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt
