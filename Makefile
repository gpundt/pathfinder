SHELL := /bin/bash
CURRENT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
BUILD_OUTPUT_FILE := $(CURRENT_DIR)target/debug/pathfinder
BUILD_DST_DIR := $(CURRENT_DIR)build/
BUILD_DST_FILE ?= $(BUILD_DST_DIR)pathfinder

## Colors ##
RED     := \033[0;31m
GREEN   := \033[0;32m
YELLOW  := \033[0;33m
BLUE    := \033[0;34m
CYAN	:= \033[0;36m
RESET   := \033[0m
define start_step_message
	@echo -e "\n$(CYAN)[*] $(1) [*]$(RESET)"
endef
define error_message
	@echo "$(RED)ERROR$(RESET): $(1)"
endef
define successful
	@echo -e "\t - $(GREEN)*Successful*$(RESET)\n"
endef

all: prep_dirs build_pathfinder

prep_dirs:
	@mkdir -p $(BUILD_DST_DIR)
	@rm -rf $(BUILD_DST_FILE)

build_pathfinder:				## Builds the Pathfinder binary
	$(call start_step_message,"Building '$(BUILD_DST_FILE)'")
	@cargo build
	@mv -f $(BUILD_OUTPUT_FILE) $(BUILD_DST_FILE)
	$(call successful)

help:							## Displays available make targets
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "$(BLUE)  %-30s$(RESET) %s\n", $$1, $$2}'