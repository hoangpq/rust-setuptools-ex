#!/usr/bin/env bash
find `pwd` -name "lib.rs" | xargs rustfmt --force --write-mode overwrite
