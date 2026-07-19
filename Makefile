# Directories
APOLLO_DIR = src/apollo
KAGUYA_DIR = src/kaguya

# Output Binaries
APOLLO_BIN = $(APOLLO_DIR)/bin/apollo.bin
KAGUYA_ELF = $(KAGUYA_DIR)/target/x86_64-unknown-none/debug/kaguya
KAGUYA_BIN = $(KAGUYA_DIR)/bin/kaguya.bin
OS_IMAGE   = os_image.bin

.PHONY: all build_apollo build_kaguya_head build_rust cargo_build objcopy_kaguya image run clean

all: image

build_apollo:
	nasm -f bin $(APOLLO_DIR)/src/apollo.asm -o $(APOLLO_BIN)

build_kaguya_head:
	nasm -f elf64 $(KAGUYA_DIR)/src/entry.asm -o $(KAGUYA_DIR)/bin/entry.o

cargo_build: build_kaguya_head
	cd $(KAGUYA_DIR) && cargo build

objcopy_kaguya: cargo_build
	objcopy -O binary $(KAGUYA_ELF) $(KAGUYA_BIN)

image: build_apollo objcopy_kaguya
	# 1. Create a blank, zero-filled file exactly 32 sectors large (32 * 512 = 16384 bytes)
	dd if=/dev/zero of=$(OS_IMAGE) bs=512 count=32
	# 2. Burn Apollo onto the absolute beginning (Sector 1), without truncating the file
	dd if=$(APOLLO_BIN) of=$(OS_IMAGE) bs=512 conv=notrunc
	# 3. Burn Kaguya directly into Sector 2 (skipping past Apollo's 512 bytes)
	dd if=$(KAGUYA_BIN) of=$(OS_IMAGE) bs=512 seek=1 conv=notrunc

run: image
	qemu-system-x86_64 -drive format=raw,file=$(OS_IMAGE)

clean:
	rm -f $(APOLLO_BIN) $(KAGUYA_DIR)/bin/entry.o $(KAGUYA_BIN) $(OS_IMAGE)
	cd $(KAGUYA_DIR) && cargo clean
