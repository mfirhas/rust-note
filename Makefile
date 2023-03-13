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
	cp -a ./.site/. ./book/

build:
	mdbook build

build_all: build cp_google_verif
