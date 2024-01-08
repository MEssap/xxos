K = target/riscv64gc-unknown-none-elf/debug
OBJDUMP = rust-objdump
OBJCOPY = rust-objcopy
QEMU = qemu-system-riscv64

QFLAGS = -machine virt 
QFLAGS += -nographic 
QFLAGS += -bios default
#QFLAGS += -bios opensbi-1.3.1-rv-bin/share/opensbi/lp64/generic/firmware/fw_dynamic.bin
QFLAGS += -m 128M 
QFLAGS += -smp 3
QFLAGS += -kernel $K/xxos.bin

CFLAGS = --release

DBGFLAGS = -s -S

all:
	@cargo build
	@echo 'build done.'

clean:
	@cargo clean
	@echo 'clean done.'

qemu: all
	$(OBJCOPY) --strip-all $K/xxos -O binary $K/xxos.bin
	$(QEMU) $(QFLAGS)

qemu-gdb: all
	$(OBJCOPY) --strip-all $K/xxos -O binary $K/xxos.bin
	$(QEMU) $(QFLAGS) $(DBGFLAGS)
