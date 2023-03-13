BUILD_DIR := ./book

default: install

all: hooks install build


h help:
	@grep '^[a-z]' Makefile


.PHONY: hooks
hooks:
	cd .git/hooks && ln -s -f ../../hooks/pre-push pre-push

install:
	cargo install mdbook


s serve:
	mdbook serve

cp_google_verif:
	cp ./google571d168c4df584a0.html ./book/

build:
	mdbook build

build_with_google_verif: build cp_google_verif
