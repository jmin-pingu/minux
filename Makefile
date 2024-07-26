QEMU=qemu-system-riscv64
MACH=virt
CPU=rv64
CPUS=1
MEM=128M
DISK=hdd.dsk
LINKER=src/virt.lds
TARGET=riscv64gc-unknown-none-elf
CONFIG=.cargo/config.toml

QEMUOPTS = -machine $(MACH) -cpu $(CPU) -smp $(CPUS) -m $(MEM) -drive if=none,format=raw,file=$(DISK),id=foo
QEMUOPTS +=-nographic -serial mon:stdio -bios none $(DEVICES) -kernel
DEVICES =-device virtio-rng-device -device virtio-gpu-device -device virtio-net-device -device virtio-tablet-device -device virtio-keyboard-device
GDB = -S -s

all:
	./make_hdd.sh
	@echo "[build]" > $(CONFIG)
	@echo "target=\"$(TARGET)\"" >> $(CONFIG)
	@echo "rustflags=['-Clink-arg=-T$(LINKER)']" >> $(CONFIG)
	@echo "[target.$(TARGET)]" >> $(CONFIG)
	@echo "runner =\"$(QEMU) $(QEMUOPTS)\"" >> $(CONFIG)
	cargo build

all-gdb:
	./make_hdd.sh
	@echo "[build]" > $(CONFIG)
	@echo "target=\"$(TARGET)\"" >> $(CONFIG)
	@echo "rustflags=['-Clink-arg=-T$(LINKER)']" >> $(CONFIG)
	@echo "[target.$(TARGET)]" >> $(CONFIG)
	@echo "runner =\"$(QEMU) $(GDB) $(QEMUOPTS)\"" >> $(CONFIG)
	cargo build
	
qemu: all
	cargo run

qemu-gdb: all-gdb
	@echo "*** Now run 'gdb' in another window." 
	cargo run


.PHONY: clean
clean:
	cargo clean
	rm -f $(OUT)




