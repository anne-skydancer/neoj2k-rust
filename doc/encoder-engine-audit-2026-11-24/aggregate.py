#!/usr/bin/env python3
"""Recompute summaries from the preserved printed microsecond medians."""
import json, pathlib, re, statistics, sys
root = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else pathlib.Path(__file__).parent
for cpu in ["baseline", "v3"]:
    rows = {}
    for file in sorted(root.glob(f"tagtree-{cpu}-[123].txt")):
        for line in file.read_text().splitlines():
            name, numbers = line.split(" current=")
            values = [float(x) for x in re.findall(r"(?:^|[a-z]+=)([\d.]+)", numbers)]
            assert len(values) == 5
            rows.setdefault(name, []).append(values)
    summary = {}
    for name, repetitions in rows.items():
        result = {}
        for index, candidate in enumerate(["control", "bound", "single", "combined"], 1):
            original = [100 * (v[index] / v[0] - 1) for v in repetitions]
            control = [100 * (v[index] / v[1] - 1) for v in repetitions]
            result[candidate] = {
                "current": round(statistics.median(original), 3),
                "range": [round(min(original), 3), round(max(original), 3)],
                "control": round(statistics.median(control), 3),
                "control_range": [round(min(control), 3), round(max(control), 3)],
            }
        summary[name] = result
    (root / f"tagtree-{cpu}-summary.json").write_text(json.dumps(summary, indent=2))
