# rust-verilog

```
[dependencies]
verilog = "0.0.1"
```

Parses and generates Verilog code.

```rust
extern crate verilog;

let code: verilog::ast::Code = verilog::parse("module a(); endmodule");
```

## License

MIT or Apache-2.0, at your option.
