list:
    just --list

doc:
    cargo doc --document-private-items --open

doc-build:
    cargo doc --document-private-items

build:
    cargo build

release:
    cargo build --release

dev: build doc-build

commit:
    git add .
    git commit
    git push