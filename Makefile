CAPNP_SRC_DIR=src
OUT_SYMLINK=target/out


CAPNP_SCHEMAS=$(wildcard $(CAPNP_SRC_DIR)/*.capnp)
CAPNP_SCHEMAS_DST=$(addprefix $(OUT_SYMLINK)/, $(CAPNP_SCHEMAS:.capnp=_capnp.rs))

#
# Make to Cargo wrappers
#
# So you can issue `make` and project will compile, or `make test` and will compile and issue
#
.PHONY: run test build doc
run test build doc:
	echo $(CAPNP_SCHEMAS_DST)
	RUST_LOG="osmium=debug" cargo $@

#
# Cargo issued commands
#
# So cargo can outsource some of it's work
#

# Build schemas (target called by cargo)

.PHONY: capnpc
capnpc: $(CAPNP_SCHEMAS_DST)

$(OUT_SYMLINK):
	ln -sf $(OUT_DIR) $(OUT_SYMLINK)

$(CAPNP_SCHEMAS_DST): | $(OUT_SYMLINK)
	capnpc --src-prefix=$(CAPNP_SRC_DIR) -o rust:$(OUT_DIR) `echo "$@" | sed -e 's!$(OUT_SYMLINK)/\(.*\)_capnp.rs!\1.capnp!g'`
