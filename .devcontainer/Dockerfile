FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN apt-get update && \
	apt-get install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev -y --no-install-recommends && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt
