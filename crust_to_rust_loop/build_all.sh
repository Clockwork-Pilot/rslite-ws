#!/bin/bash

set -euo pipefail

./scripts/shell_build.sh

./scripts/testfixture_build.sh

cd /sqlite
./rustfixture test/testrunner.tcl 2>&1 | tee /tmp/test_output.log

if grep -q "0 errors out of" /tmp/test_output.log; then
    echo "SUCCESS: All tests passed."
    exit 0
else
    echo "FAILURE: Tests failed or regex '0 errors out of' not found."
    exit 1
fi