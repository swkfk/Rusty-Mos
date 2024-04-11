mos_elf                 = target/mipsel-unknown-none/debug/rusty_mos
QEMU                    = qemu-system-mipsel
QEMU_FLAGS              += -cpu 4Kc -m 64 -nographic -M malta \
						$(shell [ -f '$(user_disk)' ] && echo '-drive id=ide0,file=$(user_disk),if=ide,format=raw ')\
						$(shell [ -f '$(empty_disk)' ] && echo '-drive id=ide1,file=$(empty_disk),if=ide,format=raw ')\
						-no-reboot

.all: build

.PHONY: clean

run:
	$(QEMU) $(QEMU_FLAGS) -kernel $(mos_elf)

dbg_run:
	$(QEMU) $(QEMU_FLAGS) -kernel $(mos_elf) -s -S

clean:
	cargo clean

build: clean
	cargo build --target mipsel-unknown-none -Zbuild-std=core,alloc
