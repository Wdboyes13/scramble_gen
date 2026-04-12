#!/bin/bash
set -euo pipefail

BUILD_CORES=$(($(nproc) / 2))
BUILD_PROFILE="release"

CYAN="\033[36m"
GREEN="\033[32m"
RED="\033[31m"
RESET="\033[00m"

run_bench() {
    printf "${GREEN}Running benchmark for ${CYAN}${1}${GREEN} generated scrambles${RESET}\n"
    hyperfine -Nw10 "./target/release/scramble -n${1}" 2>/dev/null
}

printf "${GREEN}Building binary${RESET}:\n"
printf "    Profile: ${CYAN}${BUILD_PROFILE}${RESET}\n"
printf "    Using up to ${CYAN}${BUILD_CORES}${RESET} cores\n\n"
cargo build --jobs ${BUILD_CORES} --profile ${BUILD_PROFILE} >/dev/null 2>&1

run_bench 1
run_bench 100
run_bench 1000
