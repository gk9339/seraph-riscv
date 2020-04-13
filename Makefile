CC=riscv64-linux-gnu-g++
CFLAGS=-Wall -Wextra -pedantic -Wextra -O0 -g -std=c++17 \
-static -ffreestanding -nostdlib -fno-rtti -fno-exceptions \
-march=rv64gc -mabi=lp64
INCLUDES=
LFLAGS=-Tsrc/lds/virt.ld -Wl,--build-id=none
TYPE=debug
RUST_TARGET=./target/riscv64gc-unknown-none-elf/$(TYPE)
LIBS=-L$(RUST_TARGET)
SOURCE_ASM=$(wildcard src/asm/*.S)
LIB=-lseraph_riscv -lgcc
OUT=seraph.kernel

QEMU=qemu-system-riscv64
MACH=virt
CPU=rv64
CPUS=4
MEM=128M
DRIVE=hdd.dsk

all:
	cargo build
	$(CC) $(CFLAGS) $(LFLAGS) $(INCLUDES) -o $(OUT) $(SOURCE_ASM) $(LIBS) $(LIB)

run: all
	$(QEMU) -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM) -nographic -serial mon:stdio -bios none -kernel $(OUT) -drive if=none,format=raw,file=$(DRIVE),id=a -device virtio-blk-device,scsi=off,drive=a

.PHONY: clean
clean:
	cargo clean
	rm -f $(OUT)
