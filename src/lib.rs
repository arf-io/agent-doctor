//! Agent Doctor extraction scaffold.
//!
//! This crate is the future standalone diagnostic tool for Engrave-compatible
//! AI agent environments.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolScaffold {
    pub name: &'static str,
    pub summary: &'static str,
    pub source_modules: &'static [&'static str],
    pub release_steps: &'static [&'static str],
}

pub fn scaffold() -> ToolScaffold {
    ToolScaffold {
        name: "agent-doctor",
        summary: "Check AI CLI installs, config, database readiness, policy files, and local environment health.",
        source_modules: &[
            "crates/engrave-cli/src/commands/doctor.rs",
            "crates/engrave-cli/src/commands/readiness.rs",
            "crates/engrave-cli/src/commands/health.rs",
            "crates/runner-registry/src/probe.rs",
        ],
        release_steps: &[
            "extract readiness checks into reusable check traits",
            "add machine-readable diagnostic output",
            "separate local-only checks from Engrave service checks",
            "document common repair commands for Claude, Codex, Gemini, and runner shims",
        ],
    }
}

pub fn render_human() -> String {
    let tool = scaffold();
    let mut out = format!("{}: {}\n\nSources:\n", tool.name, tool.summary);
    for source in tool.source_modules {
        out.push_str("- ");
        out.push_str(source);
        out.push('\n');
    }
    out.push_str("\nRelease steps:\n");
    for step in tool.release_steps {
        out.push_str("- ");
        out.push_str(step);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_includes_readiness_checks() {
        let tool = scaffold();
        assert_eq!(tool.name, "agent-doctor");
        assert!(tool
            .source_modules
            .iter()
            .any(|path| path.contains("readiness")));
        assert!(tool
            .release_steps
            .iter()
            .any(|step| step.contains("machine-readable")));
    }
}
