//! Self-contained HTML reports for ownership analysis results.

use std::{fmt::Write, path::Path};

use crate::Diagnostic;

pub struct HtmlFile<'a> {
    pub path: &'a Path,
    pub source: &'a str,
    pub diagnostics: &'a [Diagnostic],
}

pub struct HtmlReport<'a> {
    pub files: &'a [HtmlFile<'a>],
    pub tracked_owners: usize,
    pub tracked_borrows: usize,
}

pub fn render(report: &HtmlReport<'_>) -> String {
    let diagnostic_count = report
        .files
        .iter()
        .map(|file| file.diagnostics.len())
        .sum::<usize>();
    let status = if diagnostic_count == 0 {
        "Passed"
    } else {
        "Violations found"
    };
    let status_class = if diagnostic_count == 0 {
        "passed"
    } else {
        "failed"
    };
    let mut html = String::with_capacity(16_384);
    html.push_str(HEADER);
    let _ = writeln!(
        html,
        "<header class=\"topbar\"><div><p class=\"eyebrow\">tsborrow analysis</p><h1>{status}</h1></div><span class=\"status {status_class}\">{status}</span></header>"
    );
    let _ = writeln!(
        html,
        "<section class=\"metrics\" aria-label=\"Analysis summary\"><div><strong>{}</strong><span>files</span></div><div><strong>{diagnostic_count}</strong><span>diagnostics</span></div><div><strong>{}</strong><span>ownership contracts</span></div><div><strong>{}</strong><span>borrows</span></div></section>",
        report.files.len(),
        report.tracked_owners,
        report.tracked_borrows
    );
    html.push_str(
        "<div class=\"layout\"><aside><label for=\"file-filter\">Filter files</label><input id=\"file-filter\" type=\"search\" placeholder=\"Type a path or diagnostic\"><nav aria-label=\"Analyzed files\"><ul>\n",
    );
    for (index, file) in report.files.iter().enumerate() {
        let path = escape_html(&file.path.display().to_string());
        let _ = writeln!(
            html,
            "<li data-nav=\"{index}\"><a href=\"#file-{index}\"><span>{path}</span><b>{}</b></a></li>",
            file.diagnostics.len()
        );
    }
    html.push_str("</ul></nav></aside><main>\n");
    for (file_index, file) in report.files.iter().enumerate() {
        render_file(&mut html, file_index, file);
    }
    html.push_str("</main></div>");
    html.push_str(FOOTER);
    html
}

fn render_file(html: &mut String, file_index: usize, file: &HtmlFile<'_>) {
    let path = file.path.display().to_string();
    let searchable = format!(
        "{} {}",
        path,
        file.diagnostics
            .iter()
            .map(|diagnostic| format!("{} {}", diagnostic.code, diagnostic.message))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let _ = writeln!(
        html,
        "<section class=\"file\" id=\"file-{file_index}\" data-file=\"{file_index}\" data-search=\"{}\"><header><div><p class=\"eyebrow\">File</p><h2>{}</h2></div><span>{} diagnostic(s)</span></header>",
        escape_html(&searchable.to_lowercase()),
        escape_html(&path),
        file.diagnostics.len()
    );
    if file.diagnostics.is_empty() {
        html.push_str("<p class=\"clean\">No ownership violations found in this file.</p>");
    } else {
        for (diagnostic_index, diagnostic) in file.diagnostics.iter().enumerate() {
            render_diagnostic(html, file_index, diagnostic_index, file.source, diagnostic);
        }
    }
    html.push_str("</section>\n");
}

fn render_diagnostic(
    html: &mut String,
    file_index: usize,
    diagnostic_index: usize,
    source: &str,
    diagnostic: &Diagnostic,
) {
    let _ = write!(
        html,
        "<article class=\"diagnostic\" id=\"file-{file_index}-diagnostic-{diagnostic_index}\"><div class=\"diagnostic-heading\"><code>{}</code><span>line {}, column {}</span></div><h3>{}</h3>",
        escape_html(diagnostic.code),
        diagnostic.line,
        diagnostic.column,
        escape_html(&diagnostic.message)
    );
    render_snippet(html, source, diagnostic.line);
    html.push_str("</article>\n");
}

fn render_snippet(html: &mut String, source: &str, target_line: usize) {
    let lines = source.lines().collect::<Vec<_>>();
    if lines.is_empty() || target_line == 0 {
        return;
    }
    let start = target_line.saturating_sub(2).max(1);
    let end = (target_line + 1).min(lines.len());
    html.push_str("<pre aria-label=\"Source context\"><code>");
    for line_number in start..=end {
        let class = if line_number == target_line {
            "line target"
        } else {
            "line"
        };
        let _ = writeln!(
            html,
            "<span class=\"{class}\"><span class=\"line-number\">{line_number}</span><span>{}</span></span>",
            escape_html(lines[line_number - 1])
        );
    }
    html.push_str("</code></pre>");
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            character => escaped.push(character),
        }
    }
    escaped
}

const HEADER: &str = r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline'">
<title>tsborrow analysis report</title>
<style>
:root { color-scheme: dark; --bg:#0b1020; --panel:#121a2d; --line:#28334d; --text:#eef2ff; --muted:#9ba8c7; --accent:#7dd3fc; --danger:#fb7185; --success:#4ade80; }
* { box-sizing:border-box; }
html { scroll-behavior:smooth; }
body { margin:0; background:var(--bg); color:var(--text); font:15px/1.55 ui-sans-serif,system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif; }
.topbar { display:flex; justify-content:space-between; align-items:center; gap:24px; padding:28px clamp(20px,4vw,56px); border-bottom:1px solid var(--line); background:linear-gradient(135deg,#111a32,#0b1020); }
h1,h2,h3,p { margin-top:0; }
h1 { margin-bottom:0; font-size:clamp(28px,4vw,44px); }
.eyebrow { margin-bottom:4px; color:var(--accent); font-size:12px; font-weight:800; letter-spacing:.13em; text-transform:uppercase; }
.status { padding:8px 14px; border:1px solid; border-radius:999px; font-weight:800; }
.status.passed { color:var(--success); } .status.failed { color:var(--danger); }
.metrics { display:grid; grid-template-columns:repeat(4,minmax(0,1fr)); gap:1px; background:var(--line); border-bottom:1px solid var(--line); }
.metrics div { display:flex; flex-direction:column; padding:18px clamp(16px,3vw,40px); background:var(--panel); }
.metrics strong { font-size:24px; } .metrics span { color:var(--muted); }
.layout { display:grid; grid-template-columns:minmax(240px,320px) minmax(0,1fr); min-height:calc(100vh - 190px); }
aside { position:sticky; top:0; align-self:start; max-height:100vh; overflow:auto; padding:24px; border-right:1px solid var(--line); }
aside label { display:block; margin-bottom:8px; font-weight:700; }
input { width:100%; margin-bottom:18px; padding:10px 12px; border:1px solid var(--line); border-radius:8px; background:#080d19; color:var(--text); }
nav ul { margin:0; padding:0; list-style:none; }
nav li { margin-bottom:4px; }
nav a { display:flex; justify-content:space-between; gap:10px; padding:9px 10px; border-radius:7px; color:var(--muted); text-decoration:none; overflow-wrap:anywhere; }
nav a:hover,nav a:focus { background:var(--panel); color:var(--text); }
nav b { min-width:24px; color:var(--accent); text-align:right; }
main { min-width:0; padding:28px clamp(20px,4vw,56px) 80px; }
.file { margin-bottom:28px; scroll-margin-top:18px; }
.file>header { display:flex; justify-content:space-between; gap:20px; align-items:end; padding-bottom:14px; border-bottom:1px solid var(--line); }
.file h2 { margin-bottom:0; font:700 20px/1.4 ui-monospace,SFMono-Regular,Consolas,monospace; overflow-wrap:anywhere; }
.file>header>span,.diagnostic-heading span { color:var(--muted); white-space:nowrap; }
.clean { margin-top:14px; padding:16px; border-left:3px solid var(--success); background:var(--panel); color:var(--muted); }
.diagnostic { margin-top:16px; padding:18px; border:1px solid var(--line); border-left:3px solid var(--danger); border-radius:8px; background:var(--panel); }
.diagnostic-heading { display:flex; justify-content:space-between; gap:12px; }
.diagnostic-heading code { color:var(--danger); font-weight:800; }
.diagnostic h3 { margin:10px 0 14px; font-size:16px; }
pre { margin:0; padding:12px 0; overflow:auto; border:1px solid var(--line); border-radius:6px; background:#070b14; }
.line { display:grid; grid-template-columns:52px minmax(max-content,1fr); min-height:24px; padding:0 14px 0 0; }
.line.target { background:#3b1722; color:#fff; }
.line-number { padding-right:14px; color:#64748b; text-align:right; user-select:none; }
[hidden] { display:none !important; }
@media (max-width:760px) { .metrics { grid-template-columns:repeat(2,1fr); } .layout { display:block; } aside { position:static; max-height:none; border-right:0; border-bottom:1px solid var(--line); } }
</style>
</head>
<body>
"#;

const FOOTER: &str = r#"
<script>
const filter=document.querySelector('#file-filter');
filter.addEventListener('input',()=>{
  const query=filter.value.trim().toLowerCase();
  document.querySelectorAll('[data-file]').forEach(section=>{
    const visible=!query||section.dataset.search.includes(query);
    section.hidden=!visible;
    const nav=document.querySelector(`[data-nav="${section.dataset.file}"]`);
    if(nav) nav.hidden=!visible;
  });
});
</script>
</body>
</html>
"#;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{HtmlFile, HtmlReport, render};
    use crate::Diagnostic;

    #[test]
    fn report_navigates_files_and_escapes_untrusted_content() {
        let diagnostics = vec![Diagnostic {
            code: "TSB002",
            line: 1,
            column: 1,
            message: "bad & disposed <value>".to_owned(),
        }];
        let files = vec![HtmlFile {
            path: Path::new("src/<unsafe>.ts"),
            source: "<script>alert('x')</script>",
            diagnostics: &diagnostics,
        }];
        let rendered = render(&HtmlReport {
            files: &files,
            tracked_owners: 1,
            tracked_borrows: 0,
        });

        assert!(rendered.contains("href=\"#file-0\""));
        assert!(rendered.contains("src/&lt;unsafe&gt;.ts"));
        assert!(rendered.contains("bad &amp; disposed &lt;value&gt;"));
        assert!(rendered.contains("&lt;script&gt;alert(&#39;x&#39;)&lt;/script&gt;"));
        assert!(!rendered.contains("<script>alert('x')</script>"));
    }
}
