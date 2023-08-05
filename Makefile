SHELL         := /bin/bash
.DEFAULT_GOAL := help
APP_NAME      := rusty-server
ENV           := dev

run/watch: ## Runs the server in watch mode
	@echo "Start running the server in watch mode"
	cd server && cargo watch -x "run"
	
