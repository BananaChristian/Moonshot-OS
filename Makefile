CC = x86_64-elf-gcc
LD = x86_64-elf-ld
ASM = nasm

CFLAGS = -ffreestanding -mno-red-zone -mno-mmx -mno-sse -mno-sse2 -nostdlib -O2 -Isrc
LDFLAGS = -T kaguya.ld -nostdlib
ASMFLAGS = -f elf64

C_SOURCES = $(shell find src -name "*.c")
ASM_SOURCES= $(shell find src -name "*.asm")

C_OBJECTS = $(patsubst src/%.c, build/%.o, $(C_SOURCES))
ASM_OBJECTS = $(patsubst src/%.asm, build/%-asm.o, $(ASM_SOURCES))

OBJECTS = build/boot.o $(ASM_OBJECTS) $(C_OBJECTS)

all: kaguya.iso

build/boot.o: boot/boot.asm
	mkdir -p build
	$(ASM) $(ASMFLAGS) $< -o $@

build/%-asm.o: src/%.asm
	@mkdir -p $(dir $@)
	$(ASM) $(ASMFLAGS) $< -o $@

build/%.o: src/%.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

kaguya.bin: $(OBJECTS)
	$(LD) $(LDFLAGS) $^ -o $@

kaguya.iso: kaguya.bin
	mkdir -p iso/boot/grub
	cp kaguya.bin iso/boot/
	cp grub.cfg iso/boot/grub/
	grub-mkrescue -o kaguya.iso iso -- -volid MOONSHOT

run: kaguya.iso
	qemu-system-x86_64 -cdrom kaguya.iso -m 256M

clean:
	rm -rf build iso kaguya.bin kaguya.iso

.PHONY: all run clean
