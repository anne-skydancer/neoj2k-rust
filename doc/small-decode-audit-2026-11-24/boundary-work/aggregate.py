"""Regenerate per-repetition current/control percentage summaries from preserved raw logs."""
from pathlib import Path
import json
import re
import statistics

root = Path(__file__).resolve().parent
prefixes = ["boundary-v3", "boundary-baseline", "const-boundary-v3", "const-boundary-baseline",
            "mq-transform-v3", "mq-transform-baseline", "mq-small-v3", "mq-small-baseline",
            "width-v3", "width-baseline"]
summary = {}
for prefix in prefixes:
    cases = {}
    for source in sorted(root.glob(prefix + "-*.txt")):
        rows = {}
        for line in source.read_text().splitlines():
            match = re.match(r"(\S+) reduce (\d) (\S+): ([0-9.]+) us change ([+\-0-9.]+)%", line)
            if match:
                rows.setdefault((match[1], match[2]), {})[match[3]] = float(match[4])
        for (fixture, reduce), values in rows.items():
            for label, value in values.items():
                case = cases.setdefault(f"{fixture} reduce{reduce} {label}", {"current": [], "control": []})
                for denominator in case:
                    case[denominator].append(100 * (value / values[denominator] - 1))
    summary[prefix] = {case: {kind: {"median": statistics.median(v), "min": min(v), "max": max(v)}
                             for kind, v in result.items()} for case, result in cases.items()}
(root / "summary.json").write_text(json.dumps(summary, indent=2))
