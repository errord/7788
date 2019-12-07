DEFAULT_BUILD_OPT := "dbg"
VERBOSE := "--verbose_failures"

uninstall_py_tensorflow:
	pip uninstall tensorflow

build_and_pip build_opt="opt":
	bazel build -c {{build_opt}} //tensorflow/tools/pip_package:build_pip_package
	bazel-bin/tensorflow/tools/pip_package/build_pip_package /tmp/tensorflow_pkg
	pip install /tmp/tensorflow_pkg/`ls /tmp/tensorflow_pkg`

re_build_and_pip build_opt="opt": uninstall_py_tensorflow
	bazel build -c {{build_opt}} //tensorflow/tools/pip_package:build_pip_package
	bazel-bin/tensorflow/tools/pip_package/build_pip_package /tmp/tensorflow_pkg
	pip install /tmp/tensorflow_pkg/`ls /tmp/tensorflow_pkg`

build_example build_opt=DEFAULT_BUILD_OPT:
	bazel build -c {{build_opt}} //tensorflow/cc:tutorials_example_trainer
	@echo "./bazel-bin/tensorflow/cc/tutorials_example_trainer"

build_labX build_opt=DEFAULT_BUILD_OPT:
	bazel build -c dbg //tensorflow/cc:labX_test
	@echo "./bazel-bin/tensorflow/cc/labX_test"
	@echo "export TF_CPP_MIN_VLOG_LEVEL=5; ./bazel-bin/tensorflow/cc/labX_test --num_concurrent_sessions=1 --num_concurrent_steps=1 --num_iterations=1"

build_chezscheme build_opt=DEFAULT_BUILD_OPT:
	bazel build -c {{build_opt}} {{VERBOSE}} //tensorflow/chezscheme:gen_chezscheme_source
	bazel build -c {{build_opt}} {{VERBOSE}} //tensorflow/chezscheme:tf_chezscheme
	@echo "export TF_CPP_MIN_VLOG_LEVEL=5; ./bazel-bin/tensorflow/chezscheme/tf_chezscheme -b ./bazel-bin/tensorflow/chezscheme/boot/petite.boot -b ./bazel-bin/tensorflow/chezscheme/boot/scheme.boot  --num_concurrent_sessions=1 --num_concurrent_steps=1 --num_iterations=1"

build_benchmark_model build_opt=DEFAULT_BUILD_OPT:
	bazel build -c opt tensorflow/tools/benchmark:benchmark_model
	@echo "./bazel-bin/tensorflow/tools/benchmark/benchmark_model" \
		" --graph=./test_data/tensorflow_inception_graph.pb" \
		" --input_layer=\"input:0\"" \
		" --input_layer_shape=\"1,224,224,3\"" \
		" --input_layer_type=\"float\"" \
		" --output_layer=\"output:0\"" \
		" --show_run_order=false" \
		" --show_time=false --show_memory=false --show_summary=true"

help:
	@echo "build_opt: opt dbg default: {{DEFAULT_BUILD_OPT}}"
