#!/bin/bash

set -e

cd $(dirname $0)
cd src

rm verilog_parser.rs 2>/dev/null || true
rm verilog_parser.rs.gz 2>/dev/null || true

lalrpop verilog_parser.lalrpop
chmod 0644 verilog_parser.rs

gzip verilog_parser.rs
