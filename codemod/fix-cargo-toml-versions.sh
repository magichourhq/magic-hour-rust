#!/bin/bash

sed -i 's/openssl = { version = "\*"/openssl = { version = "0.10.46"/g' Cargo.toml
