SITE := mfathirirhas.github.io/rust-note
BUILD_DIR := ./book
SITEMAP_DIR := $(BUILD_DIR)/sitemap.xml

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

build:
	mdbook build

cp_google_verif:
	cp -a ./.site/. $(BUILD_DIR)

sitemap:
	mdbook-sitemap-generator -d $(SITE) -o $(SITEMAP_DIR)

build_all: build sitemap cp_google_verif
