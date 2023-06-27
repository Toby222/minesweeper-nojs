#!/usr/bin/just --justfile

# Lists all recipes
list:
	{{ just_executable() }} --list

# Run a dev server anad recompile on file change
dev:
	bun --port=55782 --watch ./server.ts

# Run a production server
run:
	NODE_ENV="production" bun run --port=55782 ./server.ts

