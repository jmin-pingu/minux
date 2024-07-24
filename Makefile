QEMU=qemu-system-riscv64
MACH=virt
CPU=rv64
CPUS=4
MEM=128M
DRIVE=hdd.dsk


all:
	./make_hdd.sh
	cargo build
	
qemu: all
	cargo run

run: all
	cargo run


.PHONY: clean
clean:
	cargo clean
	rm -f $(OUT)


