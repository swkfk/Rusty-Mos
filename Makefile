ARCH                    ?= mipsel

ifeq ($(ARCH), mipsel)
	qemu_cpu            = -cpu 24Kc
	qemu_machine        = -M malta
else ifeq ($(ARCH), riscv32)
	triple              = riscv32imac-unknown-none-elf
else
	$(error Unknown Arch: $(ARCH))
endif

triple                  ?= $(ARCH)-unknown-none

export ARCH

target_path             = target/$(triple)

ifneq ($(MOS_RELEASE),)
	mos_elf             = $(target_path)/release/rusty_mos
else
	mos_elf             = $(target_path)/debug/rusty_mos
endif

disk_path               = target/user
user_disk               := $(disk_path)/fs.img
empty_disk              := $(disk_path)/empty.img

QEMU                    = qemu-system-$(ARCH)
QEMU_FLAGS              += $(qemu_cpu) -m 64 -nographic $(qemu_machine) \
						$(shell [ -f '$(user_disk)' ] && echo '-drive id=ide0,file=$(user_disk),if=ide,format=raw ')\
						$(shell [ -f '$(empty_disk)' ] && echo '-drive id=ide1,file=$(empty_disk),if=ide,format=raw ')\
						-no-reboot

CARGO                   = cargo
CARGO_TARGET            = --target $(triple)
CARGO_ZBUILD            = -Zbuild-std=core,alloc
CARGO_FEATURES          = --features $(ARCH),
CARGO_FLAG              = 

ifneq ($(MOS_RELEASE),)
	CARGO_FLAG += --release
endif

CARGO_BUILD = $(CARGO) build $(CARGO_TARGET) $(CARGO_FLAG) $(CARGO_FEATURES)

.all: build, check

.PHONY: build, clean, doc, test, check

test:
	MOS_TEST=1 $(CARGO_BUILD)
	$(QEMU) $(QEMU_FLAGS) -kernel $(mos_elf)

dbg_test:
	MOS_TEST=1 $(CARGO_BUILD)
	$(QEMU) $(QEMU_FLAGS) -kernel $(mos_elf) -s -S

build:
	MOS_USER=1 $(MAKE) --directory=mos_user
	MOS_BUILD=1 $(CARGO_BUILD)

check:
	$(CARGO) check $(CARGO_TARGET) $(CARGO_ZBUILD) $(CARGO_FEATURES)
	$(CARGO) clippy $(CARGO_TARGET) $(CARGO_ZBUILD) $(CARGO_FEATURES) -- -D warnings

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
	$(CARGO) doc $(CARGO_TARGET) $(CARGO_ZBUILD) $(CARGO_FEATURES) --document-private-items
	-rm -r ./doc/
	mv $(target_path)/doc/ .
