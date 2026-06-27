use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::source_map::SourceMap;

pub struct DiagnosticRenderer<'a> {
    source_map: &'a SourceMap,
    json_mode: bool,
}

impl<'a> DiagnosticRenderer<'a> {
    pub fn new(source_map: &'a SourceMap, json_mode: bool) -> Self {
        DiagnosticRenderer {
            source_map,
            json_mode,
        }
    }

    pub fn render(&self, diagnostic: &Diagnostic) -> String {
        if self.json_mode {
            crate::diagnostics::json::to_json_string(diagnostic, self.source_map)
        } else {
            crate::diagnostics::pretty::render_diagnostic(diagnostic, self.source_map)
        }
    }

    pub fn emit(&self, diagnostic: &Diagnostic) {
        let text = self.render(diagnostic);
        eprint!("{}", text);
    }
}
