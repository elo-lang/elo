pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
}

impl std::fmt::Display for Binop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Binop::Add => write!(f, "+"),
            Binop::Sub => write!(f, "-"),
            Binop::Mul => write!(f, "*"),
            Binop::Div => write!(f, "/"),
        }
    }
}

pub enum Unop {
    Neg,
    BNot,
    Not,
    Addr,
}

impl std::fmt::Display for Unop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Unop::Neg => write!(f, "-"),
            Unop::BNot => write!(f, "~"),
            Unop::Not => write!(f, "!"),
            Unop::Addr => write!(f, "&"),
        }
    }
}

pub fn build_binop(lhs: String, rhs: String, op: Binop) -> String {
    return format!("({}) {} ({})", lhs, op, rhs);
}

pub fn build_unop(lhs: String, op: Unop) -> String {
    return format!("{}({})", op, lhs);
}

pub fn build_typed_field(r#type: String, name: String) -> String {
    return format!("{} {}", r#type, name);
}

pub fn build_comma_list(arguments: &[String]) -> String {
    let mut s = String::new();
    let l = arguments.len();
    for (i, x) in arguments.into_iter().enumerate() {
        s.push_str(x);
        if (i + 1 < l) {
            s.push(',');
        }
    }
    s
}

pub fn build_function_call(name: String, arguments: String) -> String {
    return format!("{name}({arguments})");
}

pub fn build_statement(core: String) -> String {
    return format!("{core};\n");
}

pub fn build_statement_list(statements: &[String]) -> String {
    let mut s = String::new();
    for x in statements {
        s.push_str(x);
        s.push_str(";\n");
    }
    s
}

pub fn build_function_declaration(
    r#return: String,
    name: String,
    arguments: String,
    varargs: bool,
) -> String {
    return format!(
        "{return} {name}({arguments}{})",
        if varargs { ",..." } else { "" }
    );
}

pub fn build_function_definition(
    r#return: String,
    name: String,
    arguments: String,
    varargs: bool,
    body: String,
) -> String {
    return format!(
        "{return} {name}({arguments}{}){{\n{body}}}",
        if varargs { ",..." } else { "" }
    );
}

pub fn build_if(condition: String, r#true: String, r#false: Option<String>) -> String {
    return format!(
        "if({condition})\n{{{true}}}{}",
        if let Some(r#false) = r#false {
            format!("else\n{{{false}}}")
        } else {
            String::new()
        }
    );
}

pub fn build_variable_definition(r#type: String, name: String, value: String) -> String {
    return format!("{}={value}", build_typed_field(r#type, name));
}

pub fn build_return(value: Option<String>) -> String {
    return format!(
        "return{}",
        if let Some(value) = value {
            format!(" {value}")
        } else {
            String::new()
        }
    );
}
