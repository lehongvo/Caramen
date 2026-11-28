#!/usr/bin/env python3
"""Measure your practice progress and update the progress table in the repository README.

A problem counts as SOLVED when every test of your file in `src/practice/` passes.
A problem counts as STARTED when you edited its practice file but not all tests pass yet.

Usage (from anywhere):

    python3 -I learn_11_leetcode_practice/tools/progress.py            # run tests, update ../README.md
    python3 -I learn_11_leetcode_practice/tools/progress.py --dry-run  # only print the table
"""

from __future__ import annotations

import datetime
import importlib.util
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent  # learn_11_leetcode_practice
REPO_README = ROOT.parent / "README.md"
START = "<!-- PRACTICE_PROGRESS:START -->"
END = "<!-- PRACTICE_PROGRESS:END -->"

_spec = importlib.util.spec_from_file_location("generate", Path(__file__).resolve().parent / "generate.py")
gen = importlib.util.module_from_spec(_spec)
sys.modules["generate"] = gen
_spec.loader.exec_module(gen)

_spec2 = importlib.util.spec_from_file_location("check", Path(__file__).resolve().parent / "check.py")
chk = importlib.util.module_from_spec(_spec2)
sys.modules["check"] = chk
_spec2.loader.exec_module(chk)

TITLES = {
    1: "Basics",
    2: "Ownership & Strings",
    3: "Structs, Enums & Pattern Matching",
    4: "Collections",
    5: "Errors, Closures & Iterators",
    6: "Generics, Traits & Lifetimes",
    7: "Smart Pointers, Linked Lists & Trees",
    8: "Algorithm Techniques",
    9: "DP, Graphs & Concurrency",
    10: "Advanced",
}


def bar(done: int, total: int, width: int = 10) -> str:
    if total == 0:
        return "░" * width + "   0%"
    filled = round(width * done / total)
    return "█" * filled + "░" * (width - filled) + f" {round(100 * done / total):3d}%"


def level_progress(level: int) -> dict:
    level_dir = ROOT / chk.LEVELS[level]
    info = {"level": level, "dir": chk.LEVELS[level], "total": 0, "solved": [], "started": [], "next": None, "error": None}
    try:
        modules = gen.read_modules(level_dir)
    except gen.FormatError:
        return info
    info["total"] = len(modules)
    if not modules:
        return info

    test_counts = {}
    for m in modules:
        text = (level_dir / "src" / "solutions" / f"{m}.rs").read_text(encoding="utf-8")
        test_counts[m] = len(re.findall(r"^\s*#\[test\]", text, re.M))

    practice_dir = level_dir / "src" / "practice"
    manifest = gen.read_manifest(practice_dir)
    edited = {m for m in modules if gen.is_learner_modified(practice_dir, f"{m}.rs", manifest)}

    r = subprocess.run(
        ["cargo", "test", "--lib", "practice::", "--", "--test-threads=8"],
        cwd=level_dir,
        capture_output=True,
        text=True,
    )
    if "could not compile" in r.stderr:
        broken = sorted(set(re.findall(r"src/practice/(p\d{4}_\w+)\.rs", r.stderr)))
        info["error"] = "does not compile" + (f" ({', '.join(b[:5] for b in broken[:3])})" if broken else "")
        info["started"] = sorted(edited)
    else:
        passed: dict[str, int] = {}
        for module, status in re.findall(r"^test practice::(p\d{4}_\w+)::tests::\S+ \.\.\. (ok|FAILED)$", r.stdout, re.M):
            if status == "ok":
                passed[module] = passed.get(module, 0) + 1
        info["solved"] = [m for m in modules if test_counts[m] > 0 and passed.get(m, 0) == test_counts[m]]
        info["started"] = [m for m in modules if m in edited and m not in info["solved"]]
    solved = set(info["solved"])
    info["next"] = next((m for m in modules if m not in solved), None)
    return info


def render(rows: list[dict]) -> str:
    now = datetime.datetime.now().strftime("%Y-%m-%d %H:%M")
    lines = [
        f"_Last updated: {now}. Run `python3 -I learn_11_leetcode_practice/tools/progress.py` to refresh._",
        "",
        "| Level | Solved | Progress | Started | Next problem |",
        "| ----- | ------ | -------- | ------- | ------------ |",
    ]
    total_solved = total_problems = 0
    for row in rows:
        name = f"[{row['level']}. {TITLES[row['level']]}](learn_11_leetcode_practice/{row['dir']}/README.md)"
        if (ROOT / row["dir"] / "PLAN.md").exists():
            name += " (being written)"
        if row["total"] == 0:
            lines.append(f"| {name} | - | not available yet | - | - |")
            continue
        solved, total = len(row["solved"]), row["total"]
        total_solved += solved
        total_problems += total
        started = str(len(row["started"])) if row["started"] else "-"
        if row["error"]:
            nxt = f"⚠️ {row['error']}"
        elif row["next"]:
            m = row["next"]
            nxt = f"[{m[1:5]}](learn_11_leetcode_practice/{row['dir']}/exercises/{m}.md)"
        else:
            nxt = "✅ level complete"
        lines.append(f"| {name} | {solved}/{total} | `{bar(solved, total)}` | {started} | {nxt} |")
    lines.append(f"| **Total** | **{total_solved}/1000** | `{bar(total_solved, 1000)}` | | |")
    return "\n".join(lines)


def main(argv: list[str]) -> int:
    rows = []
    for level in chk.LEVELS:
        print(f"checking {chk.LEVELS[level]} ...", file=sys.stderr)
        rows.append(level_progress(level))
    table = render(rows)
    print(table)
    if "--dry-run" in argv:
        return 0
    text = REPO_README.read_text(encoding="utf-8")
    if START not in text or END not in text:
        print(f"ERROR: {REPO_README} has no {START} / {END} markers", file=sys.stderr)
        return 1
    head, rest = text.split(START, 1)
    _, tail = rest.split(END, 1)
    REPO_README.write_text(f"{head}{START}\n\n{table}\n\n{END}{tail}", encoding="utf-8")
    print(f"\nUpdated {REPO_README}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
