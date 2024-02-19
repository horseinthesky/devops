CR := cr.yandex/crp2kiiol9t6tbj1tiog

APPS := \
	python-app \
	go-app \
	rust-app \
	client \

VERSION := latest

build:
	for app in $(APPS); do \
		cd $$app ; \
		docker build -t $(CR)/$$app:$(VERSION) . ; \
		cd .. ; \
	done

push:
	for app in $(APPS); do \
		docker push $(CR)/$$app:$(VERSION) ; \
	done

all: build push

.PHONY: build push all
