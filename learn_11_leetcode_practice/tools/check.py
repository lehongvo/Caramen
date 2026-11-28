#!/usr/bin/env python3
"""Automated checks for one level (or all levels) of learn_11_leetcode_practice.

Usage (run from the learn_11_leetcode_practice directory):

    python3 -I tools/check.py level_01_basics           # full check of one level
    python3 -I tools/check.py level_01_basics --quick   # skip clippy, rustfmt and practice tests (while authoring)
    python3 -I tools/check.py --all                     # every level + LeetCode problems reused across levels
    add --links to verify that every doc.rust-lang.org link resolves (needs network)

Exit code 0 means no ERRORs (WARNINGs do not fail the check).
"""

from __future__ import annotations

import csv
import importlib.util
import re
import subprocess
import sys
import urllib.request
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CACHE = ROOT / "tools" / ".cache" / "leetcode_index.csv"
INDEX_URL = "https://huggingface.co/datasets/kaysss/leetcode-problem-set/raw/main/data.csv"

LEVELS = {
    1: "level_01_basics",
    2: "level_02_ownership_strings",
    3: "level_03_structs_enums_match",
    4: "level_04_collections",
    5: "level_05_errors_closures_iterators",
    6: "level_06_generics_traits_lifetimes",
    7: "level_07_smart_pointers_lists_trees",
    8: "level_08_algorithms",
    9: "level_09_dp_graphs_concurrency",
    10: "level_10_advanced",
}

DIFFICULTIES = {"Easy", "Medium", "Hard"}
# LeetCode problems WITHOUT a Rust template (special Node types, node identity, concurrency...).
# They may only be used as "Adapted from".
NO_RUST = {
    116, 117, 133, 138, 141, 142, 160, 237, 426, 429, 430, 558, 559, 589, 590, 708, 1114, 1115, 1116,
    1117, 1195, 1226, 1242, 1279, 1474, 1485, 1490, 1506, 1516, 1522, 1650, 1660, 1586,
}
EXCLUDED_TAGS = {"Database", "Shell", "Concurrency"}
REQUIRED_SECTIONS = ("## Problem", "## Examples", "## Constraints", "## Hints", "## Solution")

# Load the generator from the same directory without relying on sys.path (works with python3 -I).
_spec = importlib.util.spec_from_file_location("generate", Path(__file__).resolve().parent / "generate.py")
gen = importlib.util.module_from_spec(_spec)
sys.modules["generate"] = gen  # dataclass needs the module in sys.modules
_spec.loader.exec_module(gen)


class Report:
    def __init__(self, name: str):
        self.name = name
        self.errors: list[str] = []
        self.warnings: list[str] = []

    def err(self, msg: str) -> None:
        self.errors.append(msg)

    def warn(self, msg: str) -> None:
        self.warnings.append(msg)

    def show(self) -> None:
        print(f"\n===== {self.name} =====")
        for m in self.errors:
            print(f"  ERROR: {m}")
        for m in self.warnings:
            print(f"  WARNING: {m}")
        print(f"  => {len(self.errors)} errors, {len(self.warnings)} warnings")


def load_index() -> dict[int, dict]:
    if not CACHE.exists():
        CACHE.parent.mkdir(parents=True, exist_ok=True)
        print(f"Downloading the LeetCode problem index from {INDEX_URL} ...")
        urllib.request.urlretrieve(INDEX_URL, CACHE)
    index = {}
    with CACHE.open(encoding="utf-8") as f:
        for row in csv.DictReader(f):
            try:
                index[int(row["frontendQuestionId"])] = row
            except ValueError:
                continue
    return index


def run(cmd: list[str], cwd: Path) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)


def check_file(path: Path, expected: int, index: dict[int, dict], rep: Report) -> tuple[int | None, int]:
    """Check the format of one solution file. Return (LeetCode number if referenced, number of tests)."""
    text = path.read_text(encoding="utf-8")
    name = path.name
    lc_num = None

    title = gen.TITLE.search(text)
    if not text.startswith("//! # Problem "):
        rep.err(f"{name}: the first line must be '//! # Problem {expected:04d}: <Title>'")
    if not title:
        rep.err(f"{name}: missing title '//! # Problem NNNN: Title'")
    elif int(title.group(1)) != expected:
        rep.err(f"{name}: title says problem {title.group(1)} but the file name says {expected:04d}")

    diff = gen.DIFFICULTY.search(text)
    if not diff or diff.group(1) not in DIFFICULTIES:
        rep.err(f"{name}: '- **Difficulty:**' must be one of {sorted(DIFFICULTIES)}")
    if not gen.TOPICS.search(text):
        rep.err(f"{name}: missing '- **Topics:**'")
    source = gen.SOURCE.search(text)
    if not source:
        rep.err(f"{name}: missing '- **Source:**'")
    else:
        m = gen.LEETCODE_REF.search(source.group(1))
        if "leetcode" in source.group(1).lower() and not m:
            rep.err(f"{name}: Source mentions LeetCode but is not '[LeetCode N. Title](https://leetcode.com/problems/slug/)'")
        if m:
            lc_num = int(m.group("num"))
            row = index.get(lc_num)
            adapted = bool(m.group("adapted"))
            if row is None:
                rep.err(f"{name}: there is no LeetCode problem {lc_num}")
            else:
                if row["titleSlug"] != m.group("slug"):
                    rep.err(f"{name}: LeetCode {lc_num} has slug '{row['titleSlug']}', file says '{m.group('slug')}'")
                if row["title"] != m.group("title"):
                    rep.err(f"{name}: LeetCode {lc_num} is titled '{row['title']}', file says '{m.group('title')}'")
                if row["paidOnly"] == "True":
                    rep.err(f"{name}: LeetCode {lc_num} is Premium (paid only); pick another problem")
                tags = set(re.findall(r"'([^']+)'", row.get("topicTags", "")))
                if tags & EXCLUDED_TAGS and not adapted:
                    rep.err(f"{name}: LeetCode {lc_num} is {tags & EXCLUDED_TAGS} (no Rust); use 'Adapted from'")
                if not tags:
                    rep.warn(f"{name}: LeetCode {lc_num} has no topic tags (maybe a JavaScript/SQL problem); double-check")
                if lc_num in NO_RUST and not adapted:
                    rep.err(f"{name}: LeetCode {lc_num} has no Rust template; write 'Adapted from [LeetCode ...]'")
                if not adapted and diff and row["difficulty"] != diff.group(1):
                    rep.err(f"{name}: LeetCode {lc_num} is '{row['difficulty']}', file says '{diff.group(1)}'")
                if not adapted and "impl Solution" not in text:
                    rep.warn(f"{name}: a direct LeetCode problem should use `impl Solution` with LeetCode's exact Rust signature")

    for section in REQUIRED_SECTIONS:
        if f"//! {section}" not in text:
            rep.err(f"{name}: missing section '{section}'")
    if "//! ## Solution" in text:
        after = text.split("//! ## Solution", 1)[1]
        if "**Idea:**" not in after:
            rep.err(f"{name}: the Solution section needs '**Idea:**'")
        if "**Rust concepts:**" not in after:
            rep.err(f"{name}: the Solution section needs '**Rust concepts:**'")
        if lc_num is not None and "**Complexity:**" not in after:
            rep.err(f"{name}: LeetCode-based problems need '**Complexity:**' in the Solution section")

    try:
        practice = gen.to_practice(text, name, path.stem)
        if "## Solution" in practice:
            rep.err(f"{name}: the '## Solution' section must be inside a '// @solution-start remove' region")
        code_lines = [d for d in practice.splitlines() if not d.lstrip().startswith("//")]
        if not any("todo!(" in d or "unimplemented!(" in d for d in code_lines):
            rep.warn(f"{name}: the starter code has no todo!(); are the solution regions marked?")
    except gen.FormatError as e:
        rep.err(str(e))

    # Code blocks inside doc comments must be ```text (avoid doctests)
    open_block = False
    for line_no, line in enumerate(text.splitlines(), start=1):
        s = line.strip()
        if s.startswith("//!") or s.startswith("///"):
            inner = s[3:].strip()
            if inner.startswith("```"):
                if not open_block and inner != "```text":
                    rep.err(f"{name}:{line_no}: code blocks in doc comments must open with ```text (found {inner!r})")
                open_block = not open_block
    if open_block:
        rep.err(f"{name}: unclosed ``` block in doc comments")

    # The statement must be English: flag Vietnamese diacritics anywhere in the file.
    vi = re.search(r"[ăâđêôơưĂÂĐÊÔƠƯàáảãạằắẳẵặầấẩẫậèéẻẽẹềếểễệìíỉĩịòóỏõọồốổỗộờớởỡợùúủũụừứửữựỳýỷỹỵ]", text)
    if vi:
        rep.err(f"{name}: contains Vietnamese text ({vi.group(0)!r}); problems must be written in English")

    tests = len(re.findall(r"^\s*#\[test\]", text, re.M))
    if "#[cfg(test)]" not in text:
        rep.err(f"{name}: missing the #[cfg(test)] test module")
    if tests < 3:
        rep.err(f"{name}: needs at least 3 tests, found {tests}")
    for m in re.finditer(r"#\[should_panic(.*)\]", text):
        if "expected" not in m.group(1):
            rep.err(f"{name}: #[should_panic] needs expected = \"...\" (otherwise todo!() passes the test)")
    if re.search(r"^\s*#\[ignore", text, re.M):
        rep.err(f"{name}: #[ignore] is not allowed")
    return lc_num, tests


def check_level(level: int, index: dict[int, dict], quick: bool, links: bool) -> tuple[Report, dict[int, list[str]]]:
    level_dir = ROOT / LEVELS[level]
    rep = Report(LEVELS[level])
    lc_used: dict[int, list[str]] = defaultdict(list)
    if not level_dir.exists():
        rep.err("level directory does not exist")
        return rep, lc_used

    first, last = (level - 1) * 100 + 1, level * 100
    try:
        modules = gen.read_modules(level_dir)
    except gen.FormatError as e:
        rep.err(str(e))
        return rep, lc_used

    numbers = [int(m[1:5]) for m in modules]
    if numbers != sorted(numbers):
        rep.err("src/solutions/mod.rs: problems must be listed in increasing order")
    if len(set(numbers)) != len(numbers):
        rep.err("src/solutions/mod.rs: duplicate problem numbers")
    outside = [n for n in numbers if not first <= n <= last]
    if outside:
        rep.err(f"problems outside {first:04d}-{last:04d}: {outside}")
    missing = sorted(set(range(first, last + 1)) - set(numbers))
    if missing:
        rep.err(f"{len(missing)} problems missing: {', '.join(f'{n:04d}' for n in missing[:20])}{' ...' if len(missing) > 20 else ''}")
    stray = {f.stem for f in (level_dir / "src" / "solutions").glob("p[0-9]*.rs")} - set(modules)
    if stray:
        rep.err(f"solution files not declared in mod.rs: {sorted(stray)}")

    total_tests = 0
    doc_links: set[str] = set()
    for module in modules:
        f = level_dir / "src" / "solutions" / f"{module}.rs"
        if not f.exists():
            rep.err(f"missing file {f.name}")
            continue
        lc_num, n_tests = check_file(f, int(module[1:5]), index, rep)
        total_tests += n_tests
        if lc_num is not None:
            lc_used[lc_num].append(module)
        doc_links |= set(re.findall(r"https://doc\.rust-lang\.org/[^\s)>\]`]+", f.read_text(encoding="utf-8")))
    for num, mods in lc_used.items():
        if len(mods) > 1:
            rep.err(f"LeetCode {num} is used more than once in this level: {mods}")

    readme = level_dir / "README.md"
    if not readme.exists():
        rep.err("missing README.md")
    else:
        rtext = readme.read_text(encoding="utf-8")
        if "WRITE_THIS_SECTION" in rtext:
            rep.err("README.md still contains the WRITE_THIS_SECTION placeholder")
        vi = re.search(r"[ăâđêôơưàáảãạằắẳẵặầấẩẫậèéẻẽẹềếểễệìíỉĩịòóỏõọồốổỗộờớởỡợùúủũụừứửữựỳýỷỹỵ]", rtext)
        if vi:
            rep.err("README.md contains Vietnamese text; write it in English")
        try:
            if gen.updated_readme(level_dir) != rtext:
                rep.err("the README problem table is stale; run tools/generate.py")
        except gen.FormatError as e:
            rep.err(str(e))

    practice_dir = level_dir / "src" / "practice"
    manifest = gen.read_manifest(practice_dir)
    learner_files: set[str] = set()
    try:
        practice, markdown = gen.generate(level_dir)
        for sub, expected_files, ext in ((practice_dir, practice, "rs"), (level_dir / "exercises", markdown, "md")):
            stale = [n for n, c in expected_files.items() if not (sub / n).exists() or (sub / n).read_text(encoding="utf-8") != c]
            if ext == "rs":
                learner_files = {n for n in stale if n != "mod.rs" and gen.is_learner_modified(sub, n, manifest)}
                stale = [n for n in stale if n not in learner_files]
                if learner_files:
                    rep.warn(f"{len(learner_files)} practice files contain the learner's own code and were not checked against the generator")
            if stale:
                rep.err(f"{len(stale)} files in {sub.relative_to(level_dir)} are not regenerated (e.g. {stale[:3]}); run tools/generate.py --force")
            extra = [f.name for f in sub.glob(f"p[0-9]*.{ext}") if f.name not in expected_files]
            if extra:
                rep.err(f"extra files in {sub.relative_to(level_dir)}: {extra[:5]}")
    except gen.FormatError as e:
        rep.err(str(e))

    r = run(["cargo", "test", "--lib", "--quiet", "solutions::"], level_dir)
    m = re.search(r"test result: (\w+)\. (\d+) passed; (\d+) failed", r.stdout)
    if r.returncode != 0 or not m:
        compile_errors = [d for d in r.stderr.splitlines() if d.startswith("error")][:10]
        failed = re.findall(r"^\s*(solutions::\S+)$", r.stdout, re.M)[:20]
        rep.err(f"cargo test solutions failed. Compile errors: {compile_errors} Failing tests: {failed}")
    else:
        passed = int(m.group(2))
        if passed != total_tests:
            rep.warn(f"tests run ({passed}) differ from #[test] count ({total_tests})")
        print(f"  {LEVELS[level]}: {len(modules)} problems, all {passed} solution tests pass")

    if not quick:
        # Practice starter code: NO test may pass (a pass means the solution leaked or the test calls nothing)
        r = run(["cargo", "test", "--lib", "practice::", "--", "--test-threads=8"], level_dir)
        if "could not compile" in r.stderr and learner_files:
            rep.warn("src/practice does not compile, probably because of the learner's unfinished code; practice tests skipped")
        elif "could not compile" in r.stderr:
            rep.err("starter code (src/practice) does not compile: " + " | ".join(d for d in r.stderr.splitlines() if d.startswith("error"))[:500])
        else:
            passing = re.findall(r"^test (practice::\S+) \.\.\. ok$", r.stdout, re.M)
            passing = [t for t in passing if f"{t.split('::')[1]}.rs" not in learner_files]
            if passing:
                rep.err(f"{len(passing)} starter-code tests PASS before any work (solution leaked or test calls nothing): {passing[:10]}")
        r = run(["cargo", "clippy", "--lib", "--tests", "--quiet", "--message-format=short"], level_dir)
        lints = [d for d in r.stderr.splitlines() if re.match(r"^src/.*: (warning|error)", d)]
        if lints:
            rep.err(f"clippy reports {len(lints)} warnings/errors, e.g. {lints[:5]}")
        r = run(["cargo", "fmt", "--check"], level_dir)
        if r.returncode != 0:
            unformatted = sorted({Path(p).name for p in re.findall(r"^Diff in (\S+?\.rs):\d+:", r.stdout, re.M)})
            ours = [f for f in unformatted if f not in learner_files]
            if ours or not unformatted:
                rep.err(f"cargo fmt --check is not clean: {ours[:5] or r.stdout[:300]}")

    if links:
        for url in sorted(doc_links):
            base = url.split("#", 1)[0]
            try:
                req = urllib.request.Request(base, method="HEAD", headers={"User-Agent": "learn-rust-checker"})
                with urllib.request.urlopen(req, timeout=20) as resp:
                    if resp.status != 200:
                        rep.err(f"link {url} returned {resp.status}")
            except Exception as e:  # noqa: BLE001
                rep.err(f"broken link {url}: {e}")
    return rep, lc_used


def main(argv: list[str]) -> int:
    quick, links, all_levels = "--quick" in argv, "--links" in argv, "--all" in argv
    args = [a for a in argv if not a.startswith("--")]
    index = load_index()
    if all_levels:
        targets = list(LEVELS)
    elif len(args) == 1:
        name = Path(args[0]).resolve().name
        targets = [k for k, v in LEVELS.items() if v == name]
        if not targets:
            print(f"Unknown level '{name}'. Levels: {list(LEVELS.values())}")
            return 2
    else:
        print(__doc__)
        return 2

    total_errors = 0
    lc_all: dict[int, list[str]] = defaultdict(list)
    for level in targets:
        rep, lc_used = check_level(level, index, quick, links)
        rep.show()
        total_errors += len(rep.errors)
        for num, mods in lc_used.items():
            lc_all[num] += [f"{LEVELS[level]}/{m}" for m in mods]

    if all_levels:
        shared = {n: ms for n, ms in lc_all.items() if len({m.split('/')[0] for m in ms}) > 1}
        print(f"\n===== LeetCode problems used in more than one level: {len(shared)} =====")
        for n, ms in sorted(shared.items()):
            print(f"  LeetCode {n}: {ms}")
        print(f"\nDistinct LeetCode problems: {len(lc_all)}")
    print(f"\nTOTAL: {total_errors} errors")
    return 1 if total_errors else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
