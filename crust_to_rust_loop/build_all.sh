#!/bin/bash

./shell_build.sh

./testfixture_build.sh

cd /sqlite
./rustfixture test/testrunner.tcl