#!/bin/bash

set -e

cd $(dirname $0)
cd src

lalrpop verilog_parser.lalrpop
chmod 0644 verilog_parser.rs
