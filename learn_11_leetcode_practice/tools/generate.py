#!/usr/bin/env python3
"""Generate Markdown exercises, practice starter code and the README table from solution files.

For every level, `src/solutions/pNNNN_name.rs` is the SINGLE SOURCE OF TRUTH. This tool generates:

    exercises/pNNNN_name.md     the exercise: statement, examples, hints, starter code, tests, answer (collapsed)
    src/practice/pNNNN_name.rs  starter code where the learner writes a solution and runs the tests
    README.md                   the problem table (between the two PROBLEM_TABLE markers)

Solution code is marked with a pair of comments:

    // @solution-start              -> in the starter code the whole region becomes `todo!("...")`
    // @solution-start remove       -> the whole region is removed
    // @solution-start replace: X   -> the whole region becomes the line `X`
    ...
    // @solution-end

Usage (run from the learn_11_leetcode_practice directory):

    python3 -I tools/generate.py level_01_basics            # refresh exercises/, src/practice/ and the README table
    python3 -I tools/generate.py level_01_basics --force    # same (kept for compatibility)
    python3 -I tools/generate.py level_01_basics --reset-practice   # ALSO discard your own code in src/practice/

Your work is safe: src/practice/.generated records a hash of every generated starter file. A starter file is
only rewritten while it is still exactly what the tool generated; files you edited are kept (unless you pass
--reset-practice).
"""

from __future__ import annotations

import hashlib
import re
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path

START = re.compile(r"^(?P<indent>[ \t]*)// @solution-start(?:[ \t]+(?P<mode>remove|replace:)(?P<text>.*?))?[ \t]*$")
END = re.compile(r"^[ \t]*// @solution-end[ \t]*$")
MOD_LINE = re.compile(r"^pub mod (p\d{4}_[a-z0-9_]+);\s*$")
TODO_TEXT = 'todo!("Write your solution here")'

TITLE = re.compile(r"^//! # Problem (\d{4}): (.+?)\s*$", re.M)
DIFFICULTY = re.compile(r"^//! - \*\*Difficulty:\*\* (.+?)\s*$", re.M)
TOPICS = re.compile(r"^//! - \*\*Topics:\*\* (.+?)\s*$", re.M)
SOURCE = re.compile(r"^//! - \*\*Source:\*\* (.+?)\s*$", re.M)
LEETCODE_REF = re.compile(
    r"(?P<adapted>Adapted from )?\[LeetCode (?P<num>\d+)\. (?P<title>[^\]]+)\]\(https://leetcode\.com/problems/(?P<slug>[a-z0-9-]+)/\)"
)

TABLE_START = "<!-- PROBLEM_TABLE:START -->"
TABLE_END = "<!-- PROBLEM_TABLE:END -->"
PRACTICE_HEADER_LINES = 6  # number of comment lines this tool puts at the top of each practice file
MANIFEST = ".generated"  # in src/practice: one '<sha256> <file name>' line per generated file


class FormatError(Exception):
    """A formatting problem in a solution file."""


@dataclass
class ProblemInfo:
    number: int
    title: str
    difficulty: str
    topics: str
    source: str
    module: str


# ---------------------------------------------------------------- practice starter code (src/practice)


def to_practice(content: str, file_name: str, module: str) -> str:
    """Strip the marked solution regions and return the starter code (before rustfmt)."""
    out: list[str] = []
    inside = False
    start_line = 0
    for line_no, line in enumerate(content.splitlines(), start=1):
        m = START.match(line)
        if m:
            if inside:
                raise FormatError(f"{file_name}:{line_no}: @solution-start nested inside the region opened at line {start_line}")
            inside, start_line = True, line_no
            mode, indent = m.group("mode"), m.group("indent")
            if mode is None:
                out.append(f"{indent}{TODO_TEXT}")
            elif mode == "replace:":
                text = (m.group("text") or "").strip()
                if not text:
                    raise FormatError(f"{file_name}:{line_no}: 'replace:' needs replacement code")
                out.append(f"{indent}{text}")
            continue
        if END.match(line):
            if not inside:
                raise FormatError(f"{file_name}:{line_no}: @solution-end without a matching @solution-start")
            inside = False
            continue
        if "@solution" in line:
            raise FormatError(f"{file_name}:{line_no}: malformed marker line: {line.strip()!r}")
        if not inside:
            out.append(line)
    if inside:
        raise FormatError(f"{file_name}:{start_line}: @solution-start is never closed with @solution-end")

    number = module[1:5]
    header = [
        "// ==================================================================",
        f"// PRACTICE {number}: replace every `todo!(...)` with your code; keep the tests unchanged.",
        f"// Full problem statement: exercises/{module}.md",
        f"// Run the tests:          cargo test practice::p{number}",
        "// ==================================================================",
    ]
    assert len(header) + 1 == PRACTICE_HEADER_LINES
    return "\n".join(header + [""] + out) + "\n"


# ---------------------------------------------------------------- Markdown exercise (exercises/*.md)


def _trim(lines: list[str]) -> list[str]:
    while lines and not lines[0].strip():
        lines = lines[1:]
    while lines and not lines[-1].strip():
        lines = lines[:-1]
    return lines


def _split_doc_and_code(content: str) -> tuple[list[str], list[str], list[str]]:
    """Return (markdown doc lines, code lines before #[cfg(test)], test code lines). Marker lines are dropped."""
    doc, code = [], []
    for line in content.splitlines():
        s = line.lstrip()
        if s.startswith("//!"):
            rest = s[3:]
            doc.append(rest[1:] if rest.startswith(" ") else rest)
        elif START.match(line) or END.match(line):
            continue
        else:
            code.append(line)
    test_at = next((i for i, d in enumerate(code) if d.strip() == "#[cfg(test)]"), len(code))
    return doc, _trim(code[:test_at]), _trim(code[test_at:])


def to_markdown(solution: str, practice: str, module: str) -> str:
    doc, solution_code, test_code = _split_doc_and_code(solution)
    if "## Solution" in doc:
        i = doc.index("## Solution")
        statement, explanation = _trim(doc[:i]), _trim(doc[i + 1 :])
    else:
        statement, explanation = _trim(doc), []
    practice_lines = practice.splitlines()[PRACTICE_HEADER_LINES:]
    _, starter_code, _ = _split_doc_and_code("\n".join(practice_lines))
    number = module[1:5]

    md: list[str] = []
    md += statement
    md += [
        "",
        "## Starter code",
        "",
        f"Write your code in [`src/practice/{module}.rs`](../src/practice/{module}.rs) "
        "(replace every `todo!(...)`, keep the tests unchanged), then run:",
        "",
        "```bash",
        f"cargo test practice::p{number}",
        "```",
        "",
        "```rust",
        *starter_code,
        "```",
        "",
        "## Tests",
        "",
        "<details>",
        "<summary>Show the tests used to check your solution</summary>",
        "",
        "```rust",
        *test_code,
        "```",
        "",
        "</details>",
        "",
        "## Answer",
        "",
        "<details>",
        "<summary>Show the answer</summary>",
        "",
        *explanation,
        "",
        "```rust",
        *solution_code,
        "```",
        "",
        f"Full tested solution: [`src/solutions/{module}.rs`](../src/solutions/{module}.rs)",
        "",
        "</details>",
    ]
    return "\n".join(md) + "\n"


# ---------------------------------------------------------------- module list + README


def read_modules(level_dir: Path) -> list[str]:
    mod_rs = level_dir / "src" / "solutions" / "mod.rs"
    if not mod_rs.exists():
        raise FormatError(f"missing {mod_rs}")
    modules = []
    for line in mod_rs.read_text(encoding="utf-8").splitlines():
        m = MOD_LINE.match(line)
        if m:
            modules.append(m.group(1))
    return modules


def practice_mod_rs(modules: list[str]) -> str:
    lines = [
        "//! Practice starter code for this level. One file per problem: read the statement in `exercises/`,",
        "//! replace `todo!(...)` with your code, then run `cargo test practice::pNNNN`.",
        "",
    ]
    lines += [f"pub mod {m};" for m in modules]
    return "\n".join(lines) + "\n"


def rustfmt(files: list[Path]) -> None:
    for i in range(0, len(files), 100):
        r = subprocess.run(["rustfmt", "--edition", "2024", *[str(f) for f in files[i : i + 100]]], capture_output=True, text=True)
        if r.returncode != 0:
            raise FormatError("rustfmt failed:\n" + r.stderr)


def generate(level_dir: Path) -> tuple[dict[str, str], dict[str, str]]:
    """Return ({file name in src/practice: content}, {file name in exercises: markdown})."""
    modules = read_modules(level_dir)
    practice: dict[str, str] = {"mod.rs": practice_mod_rs(modules)}
    markdown: dict[str, str] = {}
    with tempfile.TemporaryDirectory() as tmp:
        tmp_dir = Path(tmp)
        files = []
        for module in modules:
            src = level_dir / "src" / "solutions" / f"{module}.rs"
            if not src.exists():
                raise FormatError(f"mod.rs declares {module} but {src} does not exist")
            dst = tmp_dir / f"{module}.rs"
            dst.write_text(to_practice(src.read_text(encoding="utf-8"), str(src), module), encoding="utf-8")
            files.append(dst)
        rustfmt(files)
        for module in modules:
            text = (tmp_dir / f"{module}.rs").read_text(encoding="utf-8")
            practice[f"{module}.rs"] = text
            solution = (level_dir / "src" / "solutions" / f"{module}.rs").read_text(encoding="utf-8")
            markdown[f"{module}.md"] = to_markdown(solution, text, module)
    return practice, markdown


def read_problem_info(level_dir: Path) -> list[ProblemInfo]:
    out = []
    for module in read_modules(level_dir):
        f = level_dir / "src" / "solutions" / f"{module}.rs"
        text = f.read_text(encoding="utf-8")
        t, d, tp, s = TITLE.search(text), DIFFICULTY.search(text), TOPICS.search(text), SOURCE.search(text)
        if not (t and d and tp and s):
            raise FormatError(f"{f}: missing a well-formed Problem title / Difficulty / Topics / Source line")
        out.append(ProblemInfo(int(t.group(1)), t.group(2), d.group(1), tp.group(1), s.group(1), module))
    return out


def problem_table(problems: list[ProblemInfo]) -> str:
    lines = [
        "| # | Problem | Difficulty | Topics | Source |",
        "| --- | ------- | ---------- | ------ | ------ |",
    ]
    for p in problems:
        m = LEETCODE_REF.search(p.source)
        if m:
            label = "Adapted from LC" if m.group("adapted") else "LC"
            source = f"[{label} {m.group('num')}](https://leetcode.com/problems/{m.group('slug')}/)"
        else:
            source = "Original"
        title = p.title.replace("|", "\\|")
        topics = p.topics.replace("|", "\\|")
        lines.append(f"| {p.number:04d} | [{title}](exercises/{p.module}.md) | {p.difficulty} | {topics} | {source} |")
    return "\n".join(lines)


def updated_readme(level_dir: Path) -> str:
    readme = level_dir / "README.md"
    text = readme.read_text(encoding="utf-8")
    if TABLE_START not in text or TABLE_END not in text:
        raise FormatError(f"{readme}: missing the {TABLE_START} / {TABLE_END} markers")
    head, rest = text.split(TABLE_START, 1)
    _, tail = rest.split(TABLE_END, 1)
    return f"{head}{TABLE_START}\n\n{problem_table(read_problem_info(level_dir))}\n\n{TABLE_END}{tail}"


# ---------------------------------------------------------------- practice manifest (protects learner work)


def sha256(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def read_manifest(practice_dir: Path) -> dict[str, str] | None:
    f = practice_dir / MANIFEST
    if not f.exists():
        return None
    out = {}
    for line in f.read_text(encoding="utf-8").splitlines():
        if line.strip():
            digest, name = line.split(None, 1)
            out[name.strip()] = digest
    return out


def is_learner_modified(practice_dir: Path, name: str, manifest: dict[str, str] | None) -> bool:
    """True if the learner edited this practice file after the tool generated it."""
    dst = practice_dir / name
    if manifest is None or not dst.exists():
        return False
    return manifest.get(name) != sha256(dst.read_text(encoding="utf-8"))


def write_practice(practice_dir: Path, files: dict[str, str], reset: bool) -> tuple[int, int, int]:
    """Write starter files, keeping every file the learner has edited (unless reset)."""
    practice_dir.mkdir(parents=True, exist_ok=True)
    manifest = read_manifest(practice_dir)
    new_manifest: dict[str, str] = {}
    written = kept = removed = 0
    for name, text in files.items():
        dst = practice_dir / name
        current = dst.read_text(encoding="utf-8") if dst.exists() else None
        if current == text:
            new_manifest[name] = sha256(text)
            continue
        if current is None or name == "mod.rs" or reset or not is_learner_modified(practice_dir, name, manifest):
            dst.write_text(text, encoding="utf-8")
            new_manifest[name] = sha256(text)
            written += 1
        else:
            kept += 1
            if manifest and name in manifest:
                new_manifest[name] = manifest[name]  # still marked as "edited by the learner"
    for f in practice_dir.glob("p[0-9]*.rs"):
        if f.name not in files and (reset or not is_learner_modified(practice_dir, f.name, manifest)):
            f.unlink()
            removed += 1
    (practice_dir / MANIFEST).write_text("".join(f"{h} {n}\n" for n, h in sorted(new_manifest.items())), encoding="utf-8")
    return written, kept, removed


# ---------------------------------------------------------------- main


def write_dir(directory: Path, files: dict[str, str], always: bool) -> tuple[int, int, int]:
    directory.mkdir(parents=True, exist_ok=True)
    written = kept = removed = 0
    for name, text in files.items():
        dst = directory / name
        if dst.exists() and not always and name != "mod.rs":
            kept += 1
            continue
        if not dst.exists() or dst.read_text(encoding="utf-8") != text:
            dst.write_text(text, encoding="utf-8")
            written += 1
    if always:
        for f in directory.glob("p[0-9]*.*"):
            if f.name not in files:
                f.unlink()
                removed += 1
    return written, kept, removed


def main(argv: list[str]) -> int:
    args = [a for a in argv if not a.startswith("--")]
    reset = "--reset-practice" in argv
    if len(args) != 1:
        print(__doc__)
        return 2
    level_dir = Path(args[0]).resolve()
    try:
        practice, markdown = generate(level_dir)
        readme = updated_readme(level_dir)
    except FormatError as e:
        print(f"ERROR: {e}", file=sys.stderr)
        return 1
    a = write_practice(level_dir / "src" / "practice", practice, reset)
    b = write_dir(level_dir / "exercises", markdown, True)
    (level_dir / "README.md").write_text(readme, encoding="utf-8")
    print(
        f"{level_dir.name}: exercises/*.md written {b[0]}, removed {b[2]}; "
        f"src/practice written {a[0]}, kept {a[1]} edited by you, removed {a[2]}; README table updated."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
