ROOT_DIR := $(CURDIR)

GENERATE_DIR ?= $(ROOT_DIR)/generate

PYTHON2 ?= python2
PYTHON3 ?= python3
DIFF ?= diff
RM ?= rm
GIT ?= git

# TODO: Validate signature of source code.
LINUX_GIT ?= git://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git
LINUX_TAG ?= v4.16
MINGW_GIT ?= https://git.code.sf.net/p/mingw/mingw-org-wsl
MINGW_TAG ?= wsl-5.1-release

LINUX_ARCHITECTURES ?= \
	alpha \
	arc \
	arm \
	arm64 \
	c6x \
	h8300 \
	hexagon \
	ia64 \
	m68k \
	microblaze \
	mips \
	nds32 \
	nios2 \
	openrisc \
	parisc \
	powerpc \
	riscv \
	s390 \
	sh \
	sparc \
	um \
	unicore32 \
	x86 \
	xtensa

LINUX_RESULTS = $(patsubst %,$(ROOT_DIR)/errno-codes/src/unix/linux/%.*,$(LINUX_ARCHITECTURES))

.PHONY: help
help:
	@echo "Usage:"
	@echo "    make generate"
	@echo "    make clean"
	@echo "    make test"

.PHONY: generate
generate: $(GENERATE_DIR)/.rustfmt-done

.PHONY: clean
clean:
	rm -fr "$(GENERATE_DIR)"
	rm -f $(LINUX_RESULTS)

.PHONY: test
test:
	@set -eu; \
	for test_dir in generate-tests/fail/*; do \
		echo "Test (py2): $${test_dir} (should fail)"; \
		$(PYTHON2) generate.py test none "$${test_dir}/main.h" "$${test_dir}" >"./.test-out" && exit 1 || echo "SUCCESS"; \
		$(DIFF) -us "$${test_dir}/expected.stdout" "./.test-out"; \
		\
		echo "Test (py3): $${test_dir} (should fail)"; \
		$(PYTHON3) generate.py test none "$${test_dir}/main.h" "$${test_dir}" >"./.test-out" && exit 1 || echo "SUCCESS"; \
		$(DIFF) -us "$${test_dir}/expected.stdout" "./.test-out"; \
	done

	@set -eu; \
	for test_dir in generate-tests/ok/*; do \
		echo "Test (py2): $${test_dir}"; \
		$(PYTHON2) generate.py test none "$${test_dir}/main.h" "$${test_dir}" >"./.test-out" && echo "SUCCESS" || exit 1; \
		$(DIFF) -us "$${test_dir}/expected.stdout" "./.test-out"; \
		\
		echo "Test (py3): $${test_dir}"; \
		$(PYTHON3) generate.py test none "$${test_dir}/main.h" "$${test_dir}" >"./.test-out" && echo "SUCCESS" || exit 1; \
		$(DIFF) -us "$${test_dir}/expected.stdout" "./.test-out"; \
	done

	$(RM) -fv "./.test-out"

$(GENERATE_DIR)/.mkdir-done:
	[ -e "$(GENERATE_DIR)" ] && \
		$(RM) -fr "$(GENERATE_DIR)" || \
		true
	mkdir "$(GENERATE_DIR)"
	touch "$(@)"

$(GENERATE_DIR)/linux/.clone-done: GNUmakefile $(GENERATE_DIR)/.mkdir-done
	[ -e "$(GENERATE_DIR)/linux" ] && \
		$(RM) -fr "$(GENERATE_DIR)/linux" || \
		true

	$(GIT) clone --depth 1 --branch "$(LINUX_TAG)" "$(LINUX_GIT)" "$(GENERATE_DIR)/linux"
	touch "$(@)"

$(GENERATE_DIR)/linux-generic-arch/.done: $(GENERATE_DIR)/.mkdir-done
	[ -e "$(GENERATE_DIR)/linux-generic-arch" ] && \
		$(RM) -fr "$(GENERATE_DIR)/linux-generic-arch" || \
		true

	mkdir "$(GENERATE_DIR)/linux-generic-arch"
	mkdir "$(GENERATE_DIR)/linux-generic-arch/asm"
	echo "#include <asm-generic/errno.h>" >"$(GENERATE_DIR)/linux-generic-arch/asm/errno.h"

	touch "$(@)"

$(GENERATE_DIR)/.generate-linux-done: GNUmakefile \
		generate.py \
		$(GENERATE_DIR)/linux/.clone-done \
		$(GENERATE_DIR)/linux-generic-arch/.done
	set -eu; \
	for arch in $(LINUX_ARCHITECTURES); do \
		$(PYTHON2) generate.py \
			generate \
			linux \
			"$(GENERATE_DIR)/linux/include/uapi/linux/errno.h" \
			"$(ROOT_DIR)/errno-codes/src/unix/linux/$${arch}" \
			"$(GENERATE_DIR)/linux/arch/$${arch}/include/uapi" \
			"$(GENERATE_DIR)/linux/include/uapi" \
			"$(GENERATE_DIR)/linux-generic-arch"; \
	done

	touch "$(@)"

$(GENERATE_DIR)/mingw-org-wsl/.clone-done: GNUmakefile $(GENERATE_DIR)/.mkdir-done
	[ -e "$(GENERATE_DIR)/mingw-org-wsl" ] && \
		$(RM) -fr "$(GENERATE_DIR)/mingw-org-wsl" || \
		true

	$(GIT) clone --depth 1 --branch "$(MINGW_TAG)" "$(MINGW_GIT)" "$(GENERATE_DIR)/mingw-org-wsl"
	touch "$(@)"

$(GENERATE_DIR)/.generate-windows-done: GNUmakefile \
		generate.py \
		$(GENERATE_DIR)/mingw-org-wsl/.clone-done
	$(PYTHON2) generate.py \
		generate \
		windows \
		"$(GENERATE_DIR)/mingw-org-wsl/mingwrt/include/errno.h" \
		"$(ROOT_DIR)/errno-codes/src/windows"
	touch "$(@)"

$(GENERATE_DIR)/.rustfmt-done: \
		$(GENERATE_DIR)/.generate-linux-done \
		$(GENERATE_DIR)/.generate-windows-done
	./rustfmt
	touch "$(@)"
