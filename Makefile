kernel_source_files := $(shell find src/impl/kernel -name *.rs)
kernel_object_files := $(patsubst src/impl/kernel/%.rs, build/kernel/%.a, $(kernel_source_files))

x86_64_asm_source_files := $(shell find src/impl/x86_64 -name *.asm)
x86_64_asm_object_files := $(patsubst src/impl/x86_64/%.asm, build/x86_64/%.o, $(x86_64_asm_source_files))

$(kernel_object_files): build/kernel/%.o : src/impl/kernel/%.rs
	mkdir -p $(dir $@) && \
  cd src/impl/kernel && \
	cargo build -Z unstable-options --out-dir ../../../build/kernel 

$(x86_64_asm_object_files): build/x86_64/%.o : src/impl/x86_64/%.asm
	mkdir -p $(dir $@) && \
	nasm -f elf64 $(patsubst build/x86_64/%.o, src/impl/x86_64/%.asm, $@) -o $@ 

.PHONY: build-x86_64
build-x86_64: $(kernel_object_files) $(x86_64_asm_object_files)
	mkdir -p dist/x86_64 && \
	ld -n -o dist/x86_64/kernel.bin -T targets/x86_64/linker.ld $(x86_64_asm_object_files) build/kernel/libkernel.a && \
	cp dist/x86_64/kernel.bin targets/x86_64/iso/boot/kernel.bin && \
	grub-mkrescue /usr/lib/grub/i386-pc -o dist/x86_64/kernel.iso targets/x86_64/iso
