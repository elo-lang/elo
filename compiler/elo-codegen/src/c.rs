pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Gt,
    Le,
    Ge,
    Ne,
    Eq,
    Assign,
}

impl std::fmt::Display for Binop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Binop::Add => write!(f, "+"),
            Binop::Sub => write!(f, "-"),
            Binop::Mul => write!(f, "*"),
            Binop::Div => write!(f, "/"),
            Binop::Lt => write!(f, "<"),
            Binop::Gt => write!(f, ">"),
            Binop::Le => write!(f, "<="),
            Binop::Ge => write!(f, ">="),
            Binop::Ne => write!(f, "!="),
            Binop::Eq => write!(f, "=="),
            Binop::Assign => write!(f, "="),
        }
    }
}

pub enum Unop {
    Neg,
    BNot,
    Not,
    Addr,
    Deref,
}

impl std::fmt::Display for Unop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Unop::Neg => write!(f, "-"),
            Unop::BNot => write!(f, "~"),
            Unop::Not => write!(f, "!"),
            Unop::Addr => write!(f, "&"),
            Unop::Deref => write!(f, "*"),
        }
    }
}

pub fn string_expr(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for &b in text.as_bytes() {
        match b {
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            b'\0' => out.push_str("\\0"),
            b'\x07' => out.push_str("\\a"),
            b'\x08' => out.push_str("\\b"),
            b'\x0c' => out.push_str("\\f"),
            b'\x0b' => out.push_str("\\v"),
            b'\\' => out.push_str("\\\\"),
            b'\'' => out.push_str("\\'"),
            b'"'  => out.push_str("\\\""),
            b'?'  => out.push_str("\\?"),
            0x20..=0x7e => out.push(b as char),
            _ => {
                out.push_str(&format!("\\x{:02X}", b));
            }
        }
    }

    return format!("\"{out}\"");
}

pub fn field(r#type: &str, name: &str) -> String {
    return format!("{} {}", r#type, name);
}

pub fn list(arguments: &[String]) -> String {
    let mut s = String::new();
    let l = arguments.len();
    for (i, x) in arguments.into_iter().enumerate() {
        s.push_str(x);
        if i + 1 < l {
            s.push(',');
        }
    }
    s
}

pub fn binop_expr(lhs: &str, rhs: &str, op: Binop) -> String {
    return format!("({}) {} ({})", lhs, op, rhs);
}

pub fn unop_expr(lhs: &str, op: Unop) -> String {
    return format!("{}({})", op, lhs);
}

pub fn member_expr(origin: &str, member: &str) -> String {
    return format!("({origin}).{member}");
}

pub fn subscript_expr(origin: &str, index: &str) -> String {
    return format!("({origin})[{index}]")
}

pub fn array_expr(typ: &str, items: &str) -> String {
    return format!("({typ}[]){{{items}}}");
}

pub fn struct_expr(name: &str, fields: &[(String, String)]) -> String {
    let mut xs = format!("(struct {name}){{");
    for (field, value) in fields {
        xs.push_str(&format!(".{field} = {value},"));
    }
    xs.push('}');
    xs
}

pub fn struct_expr_ordered(name: &str, fields: &[String]) -> String {
    let mut xs = format!("(struct {name}){{");
    for value in fields {
        xs.push_str(&format!("{value},"));
    }
    xs.push('}');
    xs
}

pub fn function_call_expr(name: &str, arguments: &str) -> String {
    return format!("{name}({arguments})");
}

pub fn statement_list(statements: &[String]) -> String {
    let mut s = String::new();
    for x in statements {
        s.push_str(x);
    }
    s
}

pub fn function_stmt(
    r#return: &str,
    name: &str,
    arguments: &str,
    varargs: bool,
    body: &str,
) -> String {
    return format!(
        "{return} {name}({arguments}{}){{\n{body}}};\n",
        if varargs { ",..." } else { "" }
    );
}

pub fn function_decl_stmt(
    r#return: &str,
    name: &str,
    arguments: &str,
    varargs: bool,
) -> String {
    return format!(
        "{return} {name}({arguments}{});\n",
        if varargs { ",..." } else { "" }
    );
}

pub fn if_stmt(condition: &str, r#true: &str, r#false: Option<String>) -> String {
    return format!(
        "if({condition})\n{{{true}}}{};\n",
        if let Some(r#false) = r#false {
            format!("else\n{{{false}}}")
        } else {
            String::new()
        }
    );
}

pub fn while_stmt(condition: &str, block: &str) -> String {
    return format!("while({condition})\n{{{block}}};\n");
}

pub fn variable_stmt(r#type: &str, name: &str, value: &str) -> String {
    return format!("{type} {name} = {value};\n");
}

pub fn return_stmt(value: Option<String>) -> String {
    return format!(
        "return{};\n",
        if let Some(value) = value {
            format!(" {value}")
        } else {
            String::new()
        }
    );
}

pub fn enum_stmt(name: &str, body: &str) -> String {
    return format!("enum {name} {{ {body} }};\n");
}

pub fn struct_stmt(name: &str, body: &str) -> String {
    return format!("struct {name} {{ {body} }};\n");
}

pub fn expr_stmt(expr: &str) -> String {
    return format!("{expr};\n")
}
