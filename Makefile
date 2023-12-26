.PHONY: install generate-config

BIN_NAME = vaultkeeper
INSTALL_DIR = /usr/bin
CONFIG_DIR = /etc/vaultkeeper
SYSTEM := $(shell uname -s)
ARCH := $(shell uname -m)
RELEASE_FILE_NAME = $(BIN_NAME)-$(ARCH)-$(SYSTEM)

ifeq ($(SYSTEM),Linux)
    RELEASE_FILE_NAME := $(BIN_NAME)-$(ARCH)-unknown-$(SYSTEM)-gnu
endif

ifeq ($(SYSTEM),Darwin)
    RELEASE_FILE_NAME := $(BIN_NAME)-$(ARCH)-apple-$(SYSTEM)
endif

install:
	@wget -O $(RELEASE_FILE_NAME).tar.gz https://github.com/Fallenbagel/vaultkeeper/releases/latest/download/$(RELEASE_FILE_NAME).tar.gz
	@tar -zxvf $(RELEASE_FILE_NAME).tar.gz
	@chmod +x $(BIN_NAME)
	@sudo mv $(BIN_NAME) $(INSTALL_DIR)/
	@sudo mkdir -p $(CONFIG_DIR)
	@sudo $(BIN_NAME) generate-config --config $(CONFIG_DIR)
	@rm -f $(RELEASE_FILE_NAME).tar.gz
