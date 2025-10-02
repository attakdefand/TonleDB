#!/usr/bin/env python3
import sys, json, re
pii = re.compile(r'(?i)(password|secret|api[_-]?key|token|mnemonic|private[_-]?key)')
data = json.load(sys.stdin)
def scrub(x):
    if isinstance(x, dict):
        return {k: ("***REDACTED***" if pii.search(k) else scrub(v)) for k,v in x.items()}
    if isinstance(x, list):
        return [scrub(i) for i in x]
    return x
json.dump(scrub(data), sys.stdout, ensure_ascii=False, indent=2)
