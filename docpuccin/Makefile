SHELL:=/usr/bin/env bash

LOG_INFO=$(shell date +"%H:%M:%S") \e[0;34mINFO\e[0m
LOG_ERROR=$(shell date +"%H:%M:%S") \e[1;31mERROR\e[0m
LOG_WARNING=$(shell date +"%H:%M:%S") \e[0;33mWARNING\e[0m
LOG_SUCCESS=$(shell date +"%H:%M:%S") \e[0;32mSUCCESS\e[0m

.DEFAULT_GOAL := link

link:
	@echo -e "$(LOG_INFO) Linking..."
	@npm link

unlink:
	@echo -e "$(LOG_INFO) Unlinking..."
	@npm rm --global docpuccin

publish:
	@echo -e "$(LOG_INFO) Publishing to NPM..."
	@npm publish --access public
