#
# Please do not override something here.
# You must create an env.mk to overload the variable under this.
#
DOCKER_COMPOSE_BIN=docker compose
DOCKER_COMPOSE_DAEMON=-d

ifneq ($(wildcard .env),)
include .env
endif

ifneq ($(wildcard env.mk),)
include env.mk
endif

up:
	$(DOCKER_COMPOSE_BIN) up $(DOCKER_COMPOSE_DAEMON)

down:
	$(DOCKER_COMPOSE_BIN) down

run:
	go run main.go server