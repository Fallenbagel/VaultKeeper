.PHONY: install generate-config

BIN_NAME = vaultkeeper
INSTALL_DIR = /usr/bin
CONFIG_DIR = /etc/vaultkeeper

install:
    @wget -O $(BIN_NAME) https://github.com/Fallenbagel/vaultkeeper/releases/latest/download/$(BIN_NAME)
    @chmod +x $(BIN_NAME)
    @sudo mv $(BIN_NAME) $(INSTALL_DIR)/
    @sudo mkdir -p $(CONFIG_DIR)
    @$(BIN_NAME) generate-config --config $(CONFIG_DIR)