"""Minimal rustfmt-style formatter for the CI `Format` step.

Not a full rustfmt: only the line-length / wrapping rules rustfmt flagged on the
new modules (import ordering, call-argument wrapping, long boolean chains,
trailing comments). Run when rustfmt is unavailable; CI runs real rustfmt.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WIDTH = 100


def use_block(body: str, indent: str) -> list[str]:
    """Split a use-group body into rustfmt's one-per-line-ish ordering."""
    parts = [p.strip() for p in body.split(",") if p.strip()]
    parts.sort(key=lambda s: (s.lower(), s))
    return [f"{indent}{parts[0]},"] + [f"{indent}{p}," for p in parts[1:-1]] + (
        [f"{indent}{parts[-1]};"] if len(parts) > 1 else [f"{indent}{parts[-1]};"]
    )


def format_source(src: str) -> str:
    # 1) use statements with many imports: rustfmt wraps them vertically.
    def fix_use(m):
        prefix, body = m.group(1), m.group(2)
        if len(prefix) + len(body) + 6 <= WIDTH:
            return m.group(0)
        items = [p.strip() for p in body.split(",") if p.strip()]
        items.sort(key=lambda s: (s.lower(), s))
        if len(items) <= 2:
            return m.group(0)
        lines = [f"use {prefix}{{"] + [f"    {it}," for it in items[:-1]]
        lines.append(f"    {items[-1]}")
        lines.append("};")
        return "\n".join(lines)

    src = re.sub(r"use ([\w:]+)::\{([^}]*)\};", fix_use, src)

    # 2) reorder simple use ...; statements (rustfmt sorts them)
    def sort_uses(m):
        block = m.group(0)
        stmts = re.findall(r"^use [^;]+;$", block, re.M)
        if len(stmts) < 2:
            return block
        ordered = sorted(stmts, key=lambda s: s.lower())
        return block.replace(stmts[0], ordered[0]).replace(stmts[1], ordered[1])

    # 3) wrap long function signatures: `pub fn name(` followed by args on one line
    def wrap_sig(m):
        head, args = m.group(1), m.group(2)
        if len(head) + len(args) + 3 <= WIDTH:
            return m.group(0)
        pieces = [p.strip() for p in args.split(",")]
        lines = [head + "("]
        for p in pieces[:-1]:
            lines.append(f"    {p},")
        lines.append(f"    {pieces[-1]}")
        lines.append(") {")
        return "\n".join(lines)

    src = re.sub(r"^(\s*(?:pub )?fn \w+) \((.+)\) \{", wrap_sig, src, flags=re.M)

    # 4) wrap long calls: `ident.method(a, b, c, ...)` over WIDTH
    def wrap_call(m):
        head, args = m.group(1), m.group(2)
        if len(head) + len(args) + 3 <= WIDTH:
            return m.group(0)
        depth = 0
        pieces, cur = [], ""
        for ch in args:
            if ch in "([{":
                depth += 1
            elif ch in ")]}":
                depth -= 1
            if ch == "," and depth == 0:
                pieces.append(cur.strip())
                cur = ""
            else:
                cur += ch
        pieces.append(cur.strip())
        if len(pieces) < 3:
            return m.group(0)
        lines = [head + "("] + [f"    {p}," for p in pieces[:-1]] + [f"    {pieces[-1]}", ")"]
        return "\n".join(lines)

    src = re.sub(
        r"^(\s*(?:[a-z_][\w:]*\.?)+\()([^()]*(?:\([^()]*\))?[^()]*)\);$",
        lambda m: wrap_call(m) + ";",
        src,
        flags=re.M,
    )
    return src


def main() -> None:
    targets = [
        ROOT / "crates/app/src/alphared_welcome.rs",
        ROOT / "crates/app/src/analytics.rs",
        ROOT / "crates/app/src/app.rs",
        ROOT / "crates/app/src/bandwidth_profile.rs",
        ROOT / "crates/app/src/command_palette.rs",
        ROOT / "crates/app/src/hud_progress.rs",
        ROOT / "crates/app/src/hud_widget.rs",
        ROOT / "crates/app/src/main.rs",
        ROOT / "crates/app/src/radial_menu.rs",
        ROOT / "crates/app/src/sound_synth.rs",
        ROOT / "crates/app/src/wallpaper.rs",
        ROOT / "crates/app/src/progress.rs",
        ROOT / "crates/engine/src/browserlike.rs",
        ROOT / "crates/engine/src/crawler.rs",
        ROOT / "crates/engine/src/health_probe.rs",
        ROOT / "crates/tools/src/post_process.rs",
        ROOT / "crates/tools/src/social.rs",
    ]
    changed = []
    for path in targets:
        src = path.read_text(encoding="utf-8")
        new = format_source(src)
        if new != src:
            path.write_text(new, encoding="utf-8", newline="\n")
            changed.append(str(path.relative_to(ROOT)))
    print("formatted:", len(changed))
    for c in changed:
        print(" -", c)


if __name__ == "__main__":
    main()
