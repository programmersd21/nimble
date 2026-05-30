use crate::ast::*;

pub struct DocGenerator {
    modules: Vec<DocModule>,
}

pub struct DocModule {
    pub name: String,
    pub description: String,
    pub functions: Vec<DocFunction>,
    pub structs: Vec<DocStruct>,
    pub interfaces: Vec<DocInterface>,
}

pub struct DocFunction {
    pub name: String,
    pub signature: String,
    pub description: String,
    pub params: Vec<(String, String)>,
    pub return_type: String,
    pub is_pub: bool,
}

pub struct DocStruct {
    pub name: String,
    pub description: String,
    pub fields: Vec<(String, String)>,
}

pub struct DocInterface {
    pub name: String,
    pub description: String,
    pub methods: Vec<String>,
}

impl Default for DocGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl DocGenerator {
    pub fn new() -> Self {
        DocGenerator {
            modules: Vec::new(),
        }
    }

    /// Extract documentation from a parsed program
    pub fn extract_from_program(&mut self, program: &Program, module_name: &str) {
        let mut module = DocModule {
            name: module_name.to_string(),
            description: String::new(),
            functions: Vec::new(),
            structs: Vec::new(),
            interfaces: Vec::new(),
        };

        for stmt in &program.statements {
            self.extract_doc(stmt, &mut module);
        }

        self.modules.push(module);
    }

    fn extract_doc(&self, stmt: &Stmt, module: &mut DocModule) {
        match stmt {
            Stmt::FunctionDef {
                name,
                params,
                return_type,
                body,
                ..
            } => {
                let description = self.extract_doc_comment(body);
                let sig = self.format_signature(name, params, return_type);
                module.functions.push(DocFunction {
                    name: name.clone(),
                    signature: sig,
                    description,
                    params: params
                        .iter()
                        .map(|p| (p.name.clone(), p.type_annot.name.clone()))
                        .collect(),
                    return_type: return_type
                        .as_ref()
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| "Void".to_string()),
                    is_pub: false,
                });
            }
            Stmt::StructDef { name, fields, .. } => {
                module.structs.push(DocStruct {
                    name: name.clone(),
                    description: String::new(),
                    fields: fields
                        .iter()
                        .map(|f| (f.name.clone(), f.type_annot.name.clone()))
                        .collect(),
                });
            }
            Stmt::InterfaceDef { name, methods, .. } => {
                module.interfaces.push(DocInterface {
                    name: name.clone(),
                    description: String::new(),
                    methods: methods
                        .iter()
                        .map(|m| format!("{}: {}", m.name, m.type_annot.name))
                        .collect(),
                });
            }
            _ => {}
        }
    }

    fn extract_doc_comment(&self, _body: &[Stmt]) -> String {
        String::new()
    }

    fn format_signature(&self, name: &str, params: &[Param], return_type: &Option<Type>) -> String {
        let params_str: Vec<String> = params
            .iter()
            .map(|p| format!("{}: {}", p.name, p.type_annot.name))
            .collect();
        let ret = return_type
            .as_ref()
            .map(|t| t.name.clone())
            .unwrap_or_else(|| "Void".to_string());
        format!("fn {}({}) -> {}", name, params_str.join(", "), ret)
    }

    /// Generate HTML documentation
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<title>Nimble Documentation</title>\n");
        html.push_str("<style>");
        html.push_str("body { font-family: -apple-system, sans-serif; max-width: 900px; margin: 0 auto; padding: 20px; }");
        html.push_str("h1 { color: #333; border-bottom: 2px solid #4a90d9; }");
        html.push_str("h2 { color: #4a90d9; }");
        html.push_str("h3 { color: #666; }");
        html.push_str(
            ".fn { background: #f5f5f5; padding: 10px; border-radius: 5px; margin: 10px 0; }",
        );
        html.push_str(".fn-name { color: #4a90d9; font-weight: bold; }");
        html.push_str(".params { color: #888; }");
        html.push_str(
            ".struct { background: #f0f8ff; padding: 10px; border-radius: 5px; margin: 10px 0; }",
        );
        html.push_str(".interface { background: #f0fff0; padding: 10px; border-radius: 5px; margin: 10px 0; }");
        html.push_str("</style>\n</head>\n<body>\n");
        html.push_str("<h1>Nimble Standard Library</h1>\n");

        // Navigation
        html.push_str("<h2>Modules</h2><ul>\n");
        for module in &self.modules {
            html.push_str(&format!(
                "<li><a href=\"#{}\">{}</a></li>\n",
                module.name, module.name
            ));
        }
        html.push_str("</ul>\n");

        // Module details
        for module in &self.modules {
            html.push_str(&format!(
                "<h2 id=\"{}\">{}</h2>\n",
                module.name, module.name
            ));
            if !module.description.is_empty() {
                html.push_str(&format!("<p>{}</p>\n", module.description));
            }

            // Functions
            if !module.functions.is_empty() {
                html.push_str("<h3>Functions</h3>\n");
                for f in &module.functions {
                    html.push_str(&format!(
                        "<div class=\"fn\">\
                         <span class=\"fn-name\">{}</span> \
                         <span class=\"params\">{}</span> -> {} \
                         <p>{}</p>\
                         </div>\n",
                        f.name,
                        f.params
                            .iter()
                            .map(|(n, t)| format!("{}: {}", n, t))
                            .collect::<Vec<_>>()
                            .join(", "),
                        f.return_type,
                        f.description
                    ));
                }
            }

            // Structs
            if !module.structs.is_empty() {
                html.push_str("<h3>Structs</h3>\n");
                for s in &module.structs {
                    html.push_str(&format!(
                        "<div class=\"struct\"><strong>{}</strong><br>\n",
                        s.name
                    ));
                    for (name, ty) in &s.fields {
                        html.push_str(&format!("&nbsp;&nbsp;{}: {}<br>\n", name, ty));
                    }
                    html.push_str("</div>\n");
                }
            }

            // Interfaces
            if !module.interfaces.is_empty() {
                html.push_str("<h3>Interfaces</h3>\n");
                for i in &module.interfaces {
                    html.push_str(&format!(
                        "<div class=\"interface\"><strong>{}</strong><br>\n",
                        i.name
                    ));
                    for m in &i.methods {
                        html.push_str(&format!("&nbsp;&nbsp;{}<br>\n", m));
                    }
                    html.push_str("</div>\n");
                }
            }
        }

        html.push_str("</body>\n</html>");
        html
    }
}

/// Generate doc comment with special marker
pub fn is_doc_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("##") || trimmed.starts_with("///")
}

pub fn extract_doc_text(line: &str) -> String {
    let trimmed = line.trim();
    if let Some(stripped) = trimmed.strip_prefix("## ") {
        stripped.to_string()
    } else if let Some(stripped) = trimmed.strip_prefix("/// ") {
        stripped.to_string()
    } else if let Some(stripped) = trimmed.strip_prefix("##") {
        stripped.to_string()
    } else if let Some(stripped) = trimmed.strip_prefix("///") {
        stripped.to_string()
    } else {
        String::new()
    }
}
