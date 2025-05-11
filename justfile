
test_inputs_dir := "test-inputs"
test_outputs_dir := "test-outputs"
test_inputs_tar := "test-inputs.tar.gz"
firesim_modules_dir := test_inputs_dir + "/" + "firesim"

[group: 'test']
uncompress:
  tar -xvzf {{test_inputs_tar}}
  python scripts/firrtl-module-splitter.py {{test_inputs_dir}}/FireSim.fir  {{firesim_modules_dir}}

[group: 'test']
make_output_dir:
  mkdir -p {{test_outputs_dir}}

[group: 'test']
test: uncompress make_output_dir
  cargo nextest run --release

[group: 'test']
test_debug: uncompress make_output_dir
  cargo nextest run

[group: 'test']
test_only name:
  cargo nextest run --release {{name}} --nocapture

[group: 'test']
list:
  cargo nextest list

[group: 'test']
repackage_test_inputs:
  rm -rf {{firesim_modules_dir}}
  rm {{test_inputs_tar}}
  tar -cvzf {{test_inputs_tar}} {{test_inputs_dir}}

[group: 'clean']
clean:
  rm -rf {{test_inputs_dir}} {{test_outputs_dir}}

[group: 'clean']
clean_build:
  cargo clean

[group: 'clean']
clean_all: clean clean_build

[group: 'publish']
publish:
  cargo release patch
  cargo package
  cargo publish
