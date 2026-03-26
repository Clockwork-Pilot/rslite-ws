#!/bin/bash

./scripts/shell_build.sh

./scripts/testfixture_build.sh

cd /sqlite
./rustfixture test/testrunner.tcl