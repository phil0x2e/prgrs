#!/bin/bash

if [[ $(cargo fmt --all -- --check) -eq 0 ]]; then
    exit 0
fi
echo ""
echo "Your code has formatting issues, you can see them above. Please format your code with 'cargo fmt --all'"
exit 1
