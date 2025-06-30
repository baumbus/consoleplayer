list:
    just --list

doc:
    cargo doc --document-private-items --open --workspace

doc-build:
    cargo doc --document-private-items

build:
    cargo build

release:
    cargo build --release

dev: build doc-build

run-daemon:
    cargo run --bin console-player-daemon

commit:
    git add .
    git commit
    git push

act:
    sudo systemctl start docker
    act