set dotenv-load := false

help:
    just --list --unsorted

build_osa target src:
    checkexec {{target}} {{src}} -- osacompile -o {{target}} {{src}}

copy_if_updated target src:
    checkexec {{target}} {{src}} -- cp {{src}} {{target}}

build_dir:
    mkdir -p build/

build_cpp: build_dir
    checkexec build/__get_window_names --infer -- clang++ -framework carbon -framework foundation src/get_window_names.cpp -o build/__get_window_names

build_scripts: build_dir
    @just build_osa build/get_iterm_tabs.scpt src/applescript/get_iterm_tabs.applescript
    @just build_osa build/activate_iterm_tab.scpt src/applescript/activate_iterm_tab.applescript

    @just build_osa build/get_chrome_tabs.scpt src/applescript/get_chrome_tabs.applescript
    @just build_osa build/activate_chrome_tab.scpt src/applescript/activate_chrome_tab.applescript

    @just build_osa build/activate_application_window.scpt src/applescript/activate_application_window.applescript

build: build_cpp build_scripts
    SUBCOMMAND_DIR=$(pwd)/build SCRIPT_DIR=$(pwd)/build checkexec target/debug/alfwin src/main.rs -- cargo build

release: build_cpp build_scripts
    checkexec /usr/local/bin/__get_window_names build/__get_window_names -- sudo cp build/__get_window_names /usr/local/bin/__get_window_names

    sudo mkdir -p /usr/local/opt/alfwin/
    @sudo just copy_if_updated /usr/local/opt/alfwin/get_iterm_tabs.scpt build/get_iterm_tabs.scpt
    @sudo just copy_if_updated /usr/local/opt/alfwin/activate_iterm_tab.scpt build/activate_iterm_tab.scpt

    @sudo just copy_if_updated /usr/local/opt/alfwin/get_chrome_tabs.scpt build/get_chrome_tabs.scpt
    @sudo just copy_if_updated /usr/local/opt/alfwin/activate_chrome_tab.scpt build/activate_chrome_tab.scpt

    @sudo just copy_if_updated /usr/local/opt/alfwin/activate_application_window.scpt build/activate_application_window.scpt

    SUBCOMMAND_DIR=/usr/local/bin/ SCRIPT_DIR=/usr/local/opt/alfwin/ cargo build --release

install: release
    @sudo just copy_if_updated /usr/local/bin/alfwin ${CARGO_TARGET_DIR:-target}/release/alfwin

run *args: build
    ${CARGO_TARGET_DIR:-target}/debug/alfwin {{args}}