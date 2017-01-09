extern crate lalrpop_util;
extern crate regex;

pub mod ast;
pub mod verilog_parser;

pub use lalrpop_util::ParseError as ParseError;
use regex::Regex;
use std::fmt::Debug;


pub fn codelist(code: &str) {
    for (i, line) in code.lines().enumerate() {
        println!("{:>3} | {}", i+1, line);
    }
}

pub fn code_error(code: &str, tok_pos: usize) {
    let code = format!("\n\n{}", code);
    let code = code.lines().collect::<Vec<_>>();
    let mut pos: isize = 0;
    for (i, lines) in (&code[..]).windows(3).enumerate() {
        if pos + lines[2].len() as isize >= tok_pos as isize {
            if i > 1 {
                println!("{:>3} | {}", i - 1, lines[0]);
            }
            if i > 0 {
                println!("{:>3} | {}", i, lines[1]);
            }
            println!("{:>3} | {}", i + 1, lines[2]);

            println!("{}^", (0..(tok_pos as isize) - (pos - 6)).map(|_| "~").collect::<String>());
            return;
        }
        pos += (lines[2].len() as isize) + 1;
    }
}

pub fn parse_results<C,T,E>(code: &str, res: Result<C, ParseError<usize,T,E>>) -> C
where C: Debug, T: Debug, E: Debug {
    match res {
        Ok(value) => {
            return value;
        }
        Err(ParseError::InvalidToken {
            location: loc
        }) => {
            println!("Error: Invalid token:");
            code_error(code, loc);
            panic!("{:?}", res);
        }
        Err(ParseError::UnrecognizedToken {
            token: Some((loc, _, _)),
            ..
        }) => {
            println!("Error: Unrecognized token:");
            code_error(code, loc);
            panic!("{:?}", res);
        }
        err => {
            panic!("{:?}", err);
        }
    }
}

// Easy invocation of Verilog parsing.
pub fn parse(code: &str) -> ast::Code {
    // Removes comments.
    let re = Regex::new(r"(?m)//.*").unwrap();
    let code = re.replace_all(&code, "");

    parse_results(&code, verilog_parser::parse_Code(&code))
}

trait ToVerilog {
    fn to_verilog(&self) -> String;
}

impl ToVerilog for ast::Code {
    fn to_verilog(&self) -> String {
        self.0.iter()
            .map(|x| x.to_verilog())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl ToVerilog for ast::Toplevel {
    fn to_verilog(&self) -> String {
        match self {
            &ast::Toplevel::Module(ast::Ident(ref name), ref args, ref decls) => {
                format!("module {name} ({args}\n);{decls}\nendmodule\n",
                    name=name,
                    args=args.iter()
                        .map(|a| format!("\n{}", (a.0).0))
                        .collect::<Vec<_>>()
                        .join(","),
                    decls=decls.iter()
                        .map(|a| a.to_verilog())
                        .collect::<Vec<_>>()
                        .join(""))
            }
        }
    }
}

impl ToVerilog for ast::Decl {
    fn to_verilog(&self) -> String {
        match self {
            &ast::Decl::InnerArg(ref args) => {
                format!("\n{args};",
                    args=args.iter()
                        .map(|a| format!("{}", (a.0).0))
                        .collect::<Vec<_>>()
                        .join(", "))
            }
            &ast::Decl::Reg(ast::Ident(ref name), ref dims) => {
                format!("\nreg [{dims}] {name};",
                    name=name,
                    dims=dims.iter()
                        .map(|a| format!("{}", a.to_verilog()))
                        .collect::<Vec<_>>()
                        .join(":"))
            }
            &ast::Decl::Always(ref edge, ref body) => {
                format!("\nalways @({edge}) {body}",
                    edge=edge.to_verilog(),
                    body=body.to_verilog())
            }
            _ => { "\nTODO;".to_string() },
        }
    }
}

impl ToVerilog for ast::Seq {
    fn to_verilog(&self) -> String {
        match self {
            &ast::Seq::If(ref cond, ref then, ref alt) => {
                format!("\nif ({cond}) {then}{alt}",
                    cond=cond.to_verilog(),
                    then=then.to_verilog(),
                    alt=if let &Some(ref alt) = alt {
                        format!("\nelse {then}",
                            then=alt.to_verilog())
                    } else {
                        "".to_string()
                    })
            }
            &ast::Seq::Set(ref kind, ast::Ident(ref name), ref value) => {
                format!("\n{name} {kind} {value};",
                    name=name,
                    kind="<=",
                    value=value.to_verilog())
            }
            _ => { "\nTODO;".to_string() },
        }
    }
}

impl ToVerilog for ast::SeqBlock {
    fn to_verilog(&self) -> String {
        if (self.0).len() == 0 {
            panic!("should be > 0")
        } else if (self.0).len() == 1 {
            (self.0)[0].to_verilog()
        } else {
            format!("\nbegin{}\nend", self.0.iter().map(|x| x.to_verilog()).collect::<Vec<_>>().join(""))
        }
    }
}

impl ToVerilog for ast::Expr {
    fn to_verilog(&self) -> String {
        match self {
            &ast::Expr::Ref(ast::Ident(ref name)) => {
                name.to_string()
            }
            &ast::Expr::Num(value) => {
                format!("{}", value)
            }
            &ast::Expr::Arith(ref op, ref left, ref right) => {
                format!("{left} {op} {right}",
                    left=left.to_verilog(),
                    op=match *op {
                        ast::Op::Add => "+",
                        _ => "??",
                    },
                    right=right.to_verilog())
            }
            _ => { "TODO EXPR".to_string() },
        }
    }
}

impl ToVerilog for ast::EdgeRef {
    fn to_verilog(&self) -> String {
        format!("{} {}", self.1.to_verilog(), (self.0).0)
    }
}

impl ToVerilog for ast::Edge {
    fn to_verilog(&self) -> String {
        match self {
            &ast::Edge::Pos => "posedge".to_string(),
            &ast::Edge::Neg => "negedge".to_string(),
        }
    }
}

#[test]
fn test_parser() {
    let input = r#"
//-----------------------------------------------------
// Design Name : up_counter
// File Name   : up_counter.v
// Function    : Up counter
// Coder       : Deepak
//-----------------------------------------------------
module up_counter    (
out     ,  // Output of the counter
enable  ,  // enable for counter
clk     ,  // clock Input
reset      // reset Input
);
//----------Output Ports--------------
     output [7:0] out;
//------------Input Ports--------------
  input enable, clk, reset;
//------------Internal Variables--------
 reg [7:0] out;
//-------------Code Starts Here-------
 always @(posedge clk)
 if (reset) begin
   out <= 8'b0 ;
 end else if (enable) begin
   out <= out + 1;
 end
endmodule
"#;

    let res = parse(input);
    println!("Out: {:?}", res);
    println!("Out: \n{}\n", res.to_verilog());
}
