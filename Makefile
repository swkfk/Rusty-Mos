mos_elf                 = target/mipsel-unknown-none/debug/rusty_mos
QEMU                    = qemu-system-mipsel
QEMU_FLAGS              += -cpu 4Kc -m 64 -nographic -M malta \
						$(shell [ -f '$(user_disk)' ] && echo '-drive id=ide0,file=$(user_disk),if=ide,format=raw ')\
						$(shell [ -f '$(empty_disk)' ] && echo '-drive id=ide1,file=$(empty_disk),if=ide,format=raw ')\
						-no-reboot

CARGO                   = cargo
CARGO_TARGET            += --target mipsel-unknown-none
CARGO_ZBUILD            += -Zbuild-std=core,alloc

CARGO_BUILD = $(CARGO) build $(CARGO_TARGET) $(CARGO_ZBUILD)

.all: build

.PHONY: clean

build: clean
ifneq ($(test),)
	CARGO_BUILD_RUSTFLAGS='--cfg ktest_item="$(test)" --cfg ktest' $(CARGO_BUILD)
else
	$(CARGO_BUILD)
endif

run:
	$(QEMU) $(QEMU_FLAGS) -kernel $(mos_elf)

dbg_run:
	$(QEMU) $(QEMU_FLAGS) -kernel $(mos_elf) -s -S

clean:
	$(CARGO) clean
