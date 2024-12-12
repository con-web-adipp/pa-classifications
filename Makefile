# Makefile

TARGET = x86_64-pc-windows-gnu
BIN_NAME = pa-classifications
OUT_DIR = tmp
APPKG_NAME = $(BIN_NAME).appkg


.PHONY: all clean

release: build-release package


build-release:
	cargo build --bin $(BIN_NAME) --release --target=$(TARGET)
	mkdir -p $(OUT_DIR)/$(BIN_NAME)
	cp ./config/settings.json $(OUT_DIR)/settings.json
	cp ./target/$(TARGET)/release/$(BIN_NAME).exe $(OUT_DIR)/$(BIN_NAME)/$(BIN_NAME).exe

package: build-release
	cd $(OUT_DIR) && zip $(APPKG_NAME) $(BIN_NAME)/$(BIN_NAME).exe settings.json
	mv $(OUT_DIR)/$(APPKG_NAME) ./$(APPKG_NAME)
	rm -rf $(OUT_DIR)


clean:
	rm -rf $(OUT_DIR)
	rm -f $(APPKG_NAME)
	cargo clean
