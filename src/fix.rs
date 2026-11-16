//! `--lint --fix`: apply the edits of the violations that have one (`Violation::fix`):
//! delete what UnusedImport, DuplicateImport, ImportFromSamePackage and
//! UnnecessarySemicolon flag, and rewrite what the fixable Gradle rules flag. Then prove the
//! result safe, as `format` does its output. The fixed text must parse cleanly; its tokens
//! outside the edited ranges must be the input's outside them; its parse tree outside them
//! must have the input's shape (so a removed `;` can't join two statements, or make a
//! statement another's body, and a rewritten statement can't take in its neighbor); and
//! linting it must find the input's other violations and nothing new. Otherwise the file is
//! left as it was. One fix can expose another (`a();;`), and of two overlapping edits only
//! the first is made in a round, so this runs a few rounds.

use crate::lint::{Edit, Violation, lint_file};
use std::ops::Range;
use std::path::Path;
use tree_sitter::{Node, Tree};

#[derive(Debug, PartialEq)]
pub enum Fix {
    /// Nothing to fix.
    Nothing,
    /// The fixed text, and the violations fixed.
    Fixed(String, Vec<Violation>),
    /// The file doesn't parse, or a fix failed a check; nothing changes.
    Refused(&'static str),
}

const MAX_ROUNDS: usize = 6;

/// Fix the violations of `src`, the text of `path` (which decides the Jenkins, Gradle and
/// Spock rules), that have a fix and that `wanted` (a rule enabled in the config, and not
/// in the baseline) accepts; it is also given the text the violation is in. `compile` is
/// the config's `gradle_compile_replacement`, which makes `compile` dependencies fixable.
pub fn fix(
    path: &Path,
    src: &str,
    compile: Option<&str>,
    wanted: impl Fn(&Violation, &str) -> bool,
) -> Fix {
    let mut text = src.to_string();
    let mut fixed = Vec::new();
    for _ in 0..MAX_ROUNDS {
        let Ok(mut vs) = lint_file(path, &text) else {
            return Fix::Refused("parse error");
        };
        crate::lint::opt_in_fixes(&mut vs, &text, compile);
        let (todo, keep): (Vec<_>, Vec<_>) = vs
            .into_iter()
            .partition(|v| v.fix.is_some() && wanted(v, &text));
        if todo.is_empty() {
            break;
        }
        let (edits, done) = edits(&todo);
        let (out, inserted) = apply(&text, &edits);
        let auto = |v: &Violation| v.fix.is_some() && wanted(v, &out);
        if let Err(why) = check(path, &text, &out, &edits, &inserted, keep, compile, &auto) {
            return Fix::Refused(why);
        }
        fixed.extend(
            todo.into_iter()
                .zip(done)
                .filter(|(_, d)| *d)
                .map(|(v, _)| v),
        );
        text = out;
    }
    if fixed.is_empty() {
        Fix::Nothing
    } else {
        Fix::Fixed(text, fixed)
    }
}

/// The edits to make this round, sorted and none overlapping another, and for each
/// violation whether this round fixes it: its edit is made, or lies in a range one made
/// deletes (the `;` of a removed import line goes with the line). An edit overlapping one
/// made otherwise waits for the next round, which lints the text again.
fn edits(todo: &[Violation]) -> (Vec<Edit>, Vec<bool>) {
    let mut order: Vec<usize> = (0..todo.len()).collect();
    let range = |i: usize| todo[i].fix.as_ref().map_or(0..0, |e| e.range.clone());
    order.sort_by_key(|&i| (range(i).start, std::cmp::Reverse(range(i).end)));
    let mut out: Vec<Edit> = Vec::new();
    let mut done = vec![false; todo.len()];
    for i in order {
        let Some(e) = todo[i].fix.clone() else {
            continue;
        };
        match out.last() {
            Some(last) if e.range.start < last.range.end => {
                done[i] = last.with.is_empty() && e.range.end <= last.range.end;
            }
            _ => {
                out.push(e);
                done[i] = true;
            }
        }
    }
    (out, done)
}

/// The text with `edits` made, and where each edit's new text is in it.
fn apply(src: &str, edits: &[Edit]) -> (String, Vec<Range<usize>>) {
    let mut out = String::with_capacity(src.len());
    let mut inserted = Vec::new();
    let mut at = 0;
    for e in edits {
        out.push_str(&src[at..e.range.start]);
        inserted.push(out.len()..out.len() + e.with.len());
        out.push_str(&e.with);
        at = e.range.end;
    }
    out.push_str(&src[at..]);
    (out, inserted)
}

fn inside(n: Node, removed: &[Range<usize>]) -> bool {
    removed
        .iter()
        .any(|r| r.start <= n.start_byte() && n.end_byte() <= r.end)
}

#[allow(clippy::too_many_arguments)] // the two texts and the edits between them
fn check(
    path: &Path,
    src: &str,
    out: &str,
    edits: &[Edit],
    inserted: &[Range<usize>],
    keep: Vec<Violation>,
    compile: Option<&str>,
    auto: &impl Fn(&Violation) -> bool,
) -> Result<(), &'static str> {
    let removed: Vec<Range<usize>> = edits.iter().map(|e| e.range.clone()).collect();
    let (before, after) = (crate::parse(src), crate::parse(out));
    if after.root_node().has_error() {
        return Err("the fixed file would not parse");
    }
    if leaves(src, &before, &removed) != leaves(out, &after, inserted) {
        return Err("tokens other than the edited ones would change");
    }
    if shape(&before, &removed) != shape(&after, inserted) {
        return Err("the parse tree would change shape");
    }
    // the other violations stay, except one inside an edited range (a `;` whose rule is off,
    // an unused import that is also unnecessary, a dynamic version in a rewritten
    // dependency), whose counterpart in the new text isn't compared either; new fixable ones
    // are left for the next round
    let key = |v: &Violation| (v.rule, v.message.clone());
    let within = |rs: &[Range<usize>], x: &Range<usize>| {
        rs.iter().any(|r| r.start <= x.start && x.end <= r.end)
    };
    let mut want: Vec<_> = keep
        .iter()
        .filter(|v| {
            !within(&removed, &v.span)
                && !v.fix.as_ref().is_some_and(|e| within(&removed, &e.range))
        })
        .map(key)
        .collect();
    let new: Vec<Range<usize>> = inserted.iter().filter(|r| !r.is_empty()).cloned().collect();
    let mut vs = lint_file(path, out).map_err(|_| "the fixed file would not parse")?;
    crate::lint::opt_in_fixes(&mut vs, out, compile);
    let mut got: Vec<_> = vs
        .iter()
        .filter(|v| !auto(v) && !within(&new, &v.span))
        .map(key)
        .collect();
    want.sort();
    got.sort();
    if want != got {
        return Err("other lint violations would change");
    }
    Ok(())
}

/// Leaf tokens outside `removed`, as (kind, text).
fn leaves<'a>(src: &'a str, tree: &Tree, removed: &[Range<usize>]) -> Vec<(u16, &'a str)> {
    let mut v = Vec::new();
    crate::preorder(tree.root_node(), |n| {
        if inside(n, removed) {
            return false;
        }
        if n.child_count() == 0 {
            v.push((n.kind_id(), crate::text(n, src)));
        }
        true
    });
    v
}

/// Named nodes outside `removed`, as (kind, field, depth) in document order.
fn shape(tree: &Tree, removed: &[Range<usize>]) -> Vec<(u16, Option<u16>, usize)> {
    let mut v = Vec::new();
    let mut c = tree.walk();
    let mut depth = 0;
    loop {
        let n = c.node();
        let skip = inside(n, removed);
        if !skip && n.is_named() {
            v.push((n.kind_id(), c.field_id().map(|f| f.get()), depth));
        }
        if !skip && c.goto_first_child() {
            depth += 1;
            continue;
        }
        while !c.goto_next_sibling() {
            if !c.goto_parent() {
                return v;
            }
            depth -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lint::lint;

    fn fixed(src: &str) -> Fix {
        fix(Path::new("A.groovy"), src, None, |_, _| true)
    }

    fn text(src: &str) -> String {
        match fixed(src) {
            Fix::Fixed(t, _) => t,
            other => panic!("{src:?}: {other:?}"),
        }
    }

    #[test]
    fn removes_unused_and_duplicate_import_lines_and_semicolons() {
        let src = "import a.Used\nimport a.Unused\nimport a.Used\nimport b.X;\n\nUsed u = new Used();\nprintln u;\n";
        assert_eq!(
            text(src),
            "import a.Used\n\nUsed u = new Used()\nprintln u\n"
        );
        assert_eq!(fixed(&text(src)), Fix::Nothing, "idempotent");
    }

    #[test]
    fn imports_used_anywhere_lint_counts_a_use_stay() {
        for (import, usage) in [
            ("import a.Foo", "println \"$Foo\""),
            ("import a.Foo", "println \"${Foo.X}\""),
            ("import a.Foo", "/** see {@link Foo} */\nclass X {}"),
            ("import a.Foo", "@Foo\nclass X {}"),
            ("import static a.B.go", "go()"),
            ("import a.B as C", "C.go()"),
            ("import a.*", "x()"),
            ("import static a.B.*", "x()"),
        ] {
            let src = format!("{import}\n{usage}\n");
            assert_eq!(fixed(&src), Fix::Nothing, "{src:?}");
        }
        // unused static, aliased and duplicate star imports go
        assert_eq!(text("import static a.B.go\nx()\n"), "x()\n");
        assert_eq!(text("import a.B as C\nB.go()\n"), "B.go()\n");
        assert_eq!(text("import a.*\nimport a.*\nx()\n"), "import a.*\nx()\n");
        // a mention in a string is not a use
        assert_eq!(text("import a.Foo\nprintln 'a Foo'\n"), "println 'a Foo'\n");
        assert_eq!(text("import a.Foo\nprintln 'Foo'\n"), "println 'Foo'\n");
    }

    #[test]
    fn removes_same_package_imports_but_not_default_ones() {
        // `import a.C` stays: the file declares a C of its own; `java.util.Date` stays: it
        // may be beating an `a.Date` that this file can't show
        let src = "package a\nimport a.B\nimport a.*\nimport a.C\nimport java.util.Date\nclass C { B b; Date d }\n";
        assert_eq!(
            text(src),
            "package a\nimport a.C\nimport java.util.Date\nclass C { B b; Date d }\n"
        );
        assert_eq!(fixed(&text(src)), Fix::Nothing, "idempotent");
    }

    #[test]
    fn suppressed_violations_are_not_fixed() {
        let src = "// groovyfmt-ignore UnusedImport\nimport a.B\nx = 1; // groovyfmt-ignore\n@SuppressWarnings('UnnecessarySemicolon')\ndef f() { g(); \n}\nimport c.D\n";
        assert_eq!(
            text(src),
            "// groovyfmt-ignore UnusedImport\nimport a.B\nx = 1; // groovyfmt-ignore\n@SuppressWarnings('UnnecessarySemicolon')\ndef f() { g(); \n}\n"
        );
    }

    #[test]
    fn a_line_removed_takes_its_other_reports_along() {
        // `java.util.List` is unused (fixable) and unnecessary (not fixable): removing the
        // line for the first removes the second too, which is not a change to refuse
        assert_eq!(
            text("import java.util.List\nimport a.B\nB.go();\n"),
            "import a.B\nB.go()\n"
        );
    }

    #[test]
    fn leaves_lines_holding_anything_else() {
        // two imports on a line, a comment after one: reported but not fixable
        let src = "import a.B; import c.D\nimport e.F // why\nx = 1\n";
        let vs = lint(src).unwrap();
        assert!(
            vs.iter()
                .all(|v| v.rule != "UnusedImport" || v.fix.is_none()),
            "{vs:?}"
        );
        // the `;` between the two imports is not at a line end
        assert_eq!(fixed(src), Fix::Nothing);
    }

    #[test]
    fn line_endings_and_a_missing_final_newline() {
        assert_eq!(text("import a.B\r\nx = 1;\r\n"), "x = 1\r\n");
        assert_eq!(text("x = 1\nimport a.B"), "x = 1\n");
        assert_eq!(text("x = 1;"), "x = 1");
    }

    #[test]
    fn semicolons_that_are_needed_stay() {
        for src in [
            "a(); b()\n",
            "for (int i = 0; i < 3; i++) { f(i) }\n",
            "for (;;) { f() }\n",
            "while (it.next());\nf()\n",
            "f();\n{ -> g() }.call()\n",
        ] {
            assert_eq!(fixed(src), Fix::Nothing, "{src:?}");
        }
        // a `(` or `[` line after it starts a statement of its own
        assert_eq!(
            text("b();\n(c).d()\ne();\n[1].f()\n"),
            "b()\n(c).d()\ne()\n[1].f()\n"
        );
        // removing one exposes the next
        assert_eq!(text("a();;\n"), "a()\n");
    }

    #[test]
    fn unparseable_files_are_never_fixed() {
        assert_eq!(
            fixed("import a.B;\ndef x = {\n"),
            Fix::Refused("parse error")
        );
        // a classic `for`'s empty `;` body parses and is the body, so it stays
        assert_eq!(fixed("for (;;);\nf()\n"), Fix::Nothing);
        // the grammar doesn't parse an empty for-in or `if` body, so those aren't either
        for src in ["if (c);\nf()\n", "for (x in xs);\nf()\n"] {
            assert_eq!(fixed(src), Fix::Refused("parse error"), "{src:?}");
        }
    }

    #[test]
    fn only_wanted_rules_are_fixed() {
        let src = "import a.B;\nx = 1;\n";
        let semis = |v: &Violation, _: &str| v.rule == "UnnecessarySemicolon";
        let Fix::Fixed(t, f) = fix(Path::new("A.groovy"), src, None, semis) else {
            panic!()
        };
        assert_eq!((t.as_str(), f.len()), ("import a.B\nx = 1\n", 2));
        // the import line goes with its `;` even when UnnecessarySemicolon is off
        let imports = |v: &Violation, _: &str| v.rule == "UnusedImport";
        let Fix::Fixed(t, f) = fix(Path::new("A.groovy"), src, None, imports) else {
            panic!()
        };
        assert_eq!(t, "x = 1;\n");
        assert_eq!(f.len(), 1);
    }

    #[test]
    #[allow(clippy::single_range_in_vec_init)] // one range deleted is the common case
    fn the_checks_reject_a_change_of_shape() {
        let del = |range| Edit {
            range,
            with: String::new(),
        };
        // without the empty-body `;` (which lint never flags), `a()` would become the body:
        // same tokens, different tree
        assert_eq!(
            check(
                Path::new("A.groovy"),
                "while (x);\na()\n",
                "while (x)\na()\n",
                &[del(9..10)],
                &[9..9],
                Vec::new(),
                None,
                &|_| true
            ),
            Err("the parse tree would change shape")
        );
        // a change of tokens
        assert!(
            check(
                Path::new("A.groovy"),
                "x = 1;\n",
                "x = 2\n",
                &[del(5..6)],
                &[5..5],
                Vec::new(),
                None,
                &|_| true
            )
            .is_err()
        );
        assert!(
            check(
                Path::new("A.groovy"),
                "x = 1;\n",
                "x = 1\n",
                &[del(5..6)],
                &[5..5],
                Vec::new(),
                None,
                &|_| true
            )
            .is_ok()
        );
        assert_eq!(
            check(
                Path::new("A.groovy"),
                "x = 1;\ny = 2\n",
                "x = 1\ny = 2 +\n",
                &[del(5..6)],
                &[5..5],
                Vec::new(),
                None,
                &|_| true
            ),
            Err("the fixed file would not parse")
        );
    }
}
