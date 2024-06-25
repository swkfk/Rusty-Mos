target_path             = target/mipsel-unknown-none

ifneq ($(MOS_RELEASE),)
	mos_elf             = $(target_path)/release/rusty_mos
else
	mos_elf             = $(target_path)/debug/rusty_mos
endif

disk_path               = target/user
user_disk               := $(disk_path)/fs.img
empty_disk              := $(disk_path)/empty.img

QEMU                    = qemu-system-mipsel
QEMU_FLAGS              += -cpu 24Kc -m 64 -nographic -M malta \
						$(shell [ -f '$(user_disk)' ] && echo '-drive id=ide0,file=$(user_disk),if=ide,format=raw ')\
						$(shell [ -f '$(empty_disk)' ] && echo '-drive id=ide1,file=$(empty_disk),if=ide,format=raw ')\
						-no-reboot

CARGO                   = cargo
CARGO_TARGET            += --target mipsel-unknown-none
CARGO_ZBUILD            += -Zbuild-std=core,alloc
CARGO_FLAG              = 

ifneq ($(MOS_RELEASE),)
	CARGO_FLAG += --release
endif

CARGO_BUILD = $(CARGO) build $(CARGO_TARGET) $(CARGO_FLAG)

.all: build

.PHONY: build, clean, doc, test

test:
	MOS_TEST=1 $(CARGO_BUILD)
	$(QEMU) $(QEMU_FLAGS) -kernel $(mos_elf)

dbg_test:
	MOS_TEST=1 $(CARGO_BUILD)
	$(QEMU) $(QEMU_FLAGS) -kernel $(mos_elf) -s -S

build:
	MOS_USER=1 $(MAKE) --directory=mos_user
	MOS_BUILD=1 $(CARGO_BUILD)

run: build
	$(QEMU) $(QEMU_FLAGS) -kernel $(mos_elf)

dbg_run: build
	$(QEMU) $(QEMU_FLAGS) -kernel $(mos_elf) -s -S

pts:
	gdb-multiarch -q $(mos_elf) -ex "target remote localhost:1234"

clean:
	$(CARGO) clean
	MOS_USER=1 $(MAKE) --directory=mos_user clean

doc:
	$(CARGO) doc $(CARGO_TARGET) $(CARGO_ZBUILD) --document-private-items
	-rm -r ./doc/
	mv $(target_path)/doc/ .
