.PHONY: default test vendor build all clippy

CARGO      := MAKEFLAGS= env CARGO_BUILD_JOBS=12 NUM_JOBS=12 cargo
CARGO_TEST := $(CARGO) test

DEFAULT         := test
DEFAULT         := test_file
#DEFAULT         := test_all
#DEFAULT         := build
#DEFAULT         := test_file_one
DEFAULT         := all

RUST_LOG       := info
TEST_FILE       := surge-math/tests/convert.rs

#INDIVIDUAL_TEST := test_get_absmax_2
INDIVIDUAL_TEST := xxx

default: $(DEFAULT)

#--release
#--color always 
TAIL_FLAGS := 

#RUSTFLAGS := "-Awarnings"
RUSTFLAGS := ""

#ACTIVE_PACKAGE := surge-adsr
#ACTIVE_PACKAGE := surge-biquad
#ACTIVE_PACKAGE := surge-blitter
#ACTIVE_PACKAGE := surge-coeffmaker
#ACTIVE_PACKAGE := surge-constants
#ACTIVE_PACKAGE := surge-derive
#ACTIVE_PACKAGE := surge-filter
#ACTIVE_PACKAGE := surge-halfrate
#ACTIVE_PACKAGE := surge-hound
#ACTIVE_PACKAGE := surge-imports
#ACTIVE_PACKAGE := surge-input
#ACTIVE_PACKAGE := surge-lag
#ACTIVE_PACKAGE := surge-lfo
#ACTIVE_PACKAGE := surge-lipol
#ACTIVE_PACKAGE := surge-macros
ACTIVE_PACKAGE := surge-math
#ACTIVE_PACKAGE := surge-midi
#ACTIVE_PACKAGE := surge-modulation
#ACTIVE_PACKAGE := surge-mpe
#ACTIVE_PACKAGE := surge-output
#ACTIVE_PACKAGE := surge-param
#ACTIVE_PACKAGE := surge-qfunit
#ACTIVE_PACKAGE := surge-quadrosc
#ACTIVE_PACKAGE := surge-samplerate
#ACTIVE_PACKAGE := surge-scene
#ACTIVE_PACKAGE := surge-stepseq
#ACTIVE_PACKAGE := surge-svf
#ACTIVE_PACKAGE := surge-synthesizer
#ACTIVE_PACKAGE := surge-tables
#ACTIVE_PACKAGE := surge-timeunit
#ACTIVE_PACKAGE := surge-traits
#ACTIVE_PACKAGE := surge-tuning
#ACTIVE_PACKAGE := surge-types
#ACTIVE_PACKAGE := surge-voice
#ACTIVE_PACKAGE := surge-wavetable
#ACTIVE_PACKAGE := surgefilter-comb
#ACTIVE_PACKAGE := surgefilter-diode
#ACTIVE_PACKAGE := surgefilter-huovilainen
#ACTIVE_PACKAGE := surgefilter-iir
#ACTIVE_PACKAGE := surgefilter-k35
#ACTIVE_PACKAGE := surgefilter-moog
#ACTIVE_PACKAGE := surgefilter-nlfeedback
#ACTIVE_PACKAGE := surgefilter-nlstates
#ACTIVE_PACKAGE := surgefilter-obxd
#ACTIVE_PACKAGE := surgefilter-rungekutta
#ACTIVE_PACKAGE := surgefilter-snh
#ACTIVE_PACKAGE := surgefilter-svf
#ACTIVE_PACKAGE := surgefx
#ACTIVE_PACKAGE := surgefx-allpass
#ACTIVE_PACKAGE := surgefx-chorus
#ACTIVE_PACKAGE := surgefx-conditioner
#ACTIVE_PACKAGE := surgefx-distortion
#ACTIVE_PACKAGE := surgefx-dualdelay
#ACTIVE_PACKAGE := surgefx-emphasize
#ACTIVE_PACKAGE := surgefx-eq3band
#ACTIVE_PACKAGE := surgefx-flanger
#ACTIVE_PACKAGE := surgefx-freqshift
#ACTIVE_PACKAGE := surgefx-phaser
#ACTIVE_PACKAGE := surgefx-reverb
#ACTIVE_PACKAGE := surgefx-ringmod
#ACTIVE_PACKAGE := surgefx-rotary
#ACTIVE_PACKAGE := surgefx-vocoder
#ACTIVE_PACKAGE := surgeosc-audioin
#ACTIVE_PACKAGE := surgeosc-fm
#ACTIVE_PACKAGE := surgeosc-fm2
#ACTIVE_PACKAGE := surgeosc-sine
#ACTIVE_PACKAGE := surgeosc-snh
#ACTIVE_PACKAGE := surgeosc-super
#ACTIVE_PACKAGE := surgeosc-wavetable
#ACTIVE_PACKAGE := surgeosc-window
#ACTIVE_PACKAGE := surgeshaper-asym
#ACTIVE_PACKAGE := surgeshaper-clip
#ACTIVE_PACKAGE := surgeshaper-digi
#ACTIVE_PACKAGE := surgeshaper-sine
#ACTIVE_PACKAGE := surgeshaper-tanh

all:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) build $(TAIL_FLAGS)

build:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) build -p $(ACTIVE_PACKAGE) $(TAIL_FLAGS)

tracemacro:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS="-Z macro-backtrace -Awarnings" $(CARGO) build -p $(ACTIVE_PACKAGE) $(TAIL_FLAGS)

test:
	RUST_LOG=$(RUST_LOG) RUST_BACKTRACE=1 RUSTFLAGS=$(RUSTFLAGS) $(CARGO_TEST) -p $(ACTIVE_PACKAGE) $(TAIL_FLAGS) -- --nocapture

bench:
	RUST_LOG=$(RUST_LOG) RUST_BACKTRACE=1 RUSTFLAGS=$(RUSTFLAGS) $(CARGO) bench -p $(ACTIVE_PACKAGE) $(TAIL_FLAGS) -- --nocapture

test_all:
	RUST_LOG=$(RUST_LOG) RUST_BACKTRACE=full RUSTFLAGS=$(RUSTFLAGS) $(CARGO_TEST)


TEST_FILE_TARGET := $(basename $(notdir $(TEST_FILE)))

test_file:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO_TEST) -p $(ACTIVE_PACKAGE) --test $(TEST_FILE_TARGET) -- --nocapture

test_file_one:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO_TEST) -p $(ACTIVE_PACKAGE) --test $(TEST_FILE_TARGET) $(INDIVIDUAL_TEST) -- --nocapture

vendor:
	RUST_LOG=$(RUST_LOG) $(CARGO) vendor

fixhelp:
	RUST_LOG=$(RUST_LOG) $(CARGO) help fix

protogen: 
	protoc \
	    --rust_out=surge-protos/src \
	    --experimental_allow_proto3_optional \
	    --grpc_out=surge-protos/src \
	    --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
	    --proto_path \
	    ./surge-protos/protos ./surge-protos/protos/surge.proto
clippy:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) clippy -p $(ACTIVE_PACKAGE)  -- \
		  -A clippy::redundant_field_names \
		  -W clippy::all 

clippy_all:
	RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) clippy  -- \
		  -A clippy::redundant_field_names \
		  -W clippy::all 
