PROJECT=$(notdir $(shell pwd))_devcontainer
FILE=./.devcontainer/docker-compose.yaml
SERVICE=
UP_FLAGS=

.PHONY: up
up:
	docker compose -f $(FILE) --project-name $(PROJECT) up $(SERVICE) --detach $(UP_FLAGS)

.PHONY: logs
logs:
	-docker compose -f $(FILE) --project-name $(PROJECT) logs $(SERVICE) --follow

.PHONY: down
down:
	docker compose -f $(FILE) --project-name $(PROJECT) down

.PHONY: stop
stop:
	docker compose -f $(FILE) --project-name $(PROJECT) stop $(SERVICE)
	docker compose -f $(FILE) --project-name $(PROJECT) rm $(SERVICE) --force

.PHONY: ps
ps:
	docker compose -f $(FILE) --project-name $(PROJECT) ps

.PHONY: clean
clean: down
	docker volume ls --format '{{- .Name -}}' | grep -e "^$(PROJECT).*" | xargs docker volume rm
	docker builder prune --filter type=exec.cachemount --force

.PHONY: code
code:
	code $$(pwd)
