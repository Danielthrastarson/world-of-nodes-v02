#!/usr/bin/env python3
"""Strict immutable P0–P4 historical panel comparison, including all hashes.

No semantic normalization is permitted. Gzip container metadata is discarded;
its uncompressed bytes are exact evidence. Wall-time fields are absent from
these registered F11C and F13B panels, so CSV values have no exceptions.
"""
import csv
import gzip
import hashlib
import io
import json
from pathlib import Path
import sys


def content(path):
    raw = path.read_bytes()
    return gzip.decompress(raw) if path.suffix == '.gz' else raw


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def table(raw):
    reader = csv.DictReader(io.StringIO(raw.decode()))
    return reader.fieldnames, list(reader)


def main():
    old_root, new_root, output = map(Path, sys.argv[1:])
    if output.exists():
        raise FileExistsError(output)
    old_dirs = {p.name for p in old_root.iterdir() if p.is_dir()}
    new_dirs = {p.name for p in new_root.iterdir() if p.is_dir()}
    result = {'old_root': str(old_root), 'new_root': str(new_root),
              'comparison': 'Every registered arm artifact exact; no excluded fields or hashes; gzip decoded',
              'checks': [], 'differences': [], 'runs': 0}
    diffs = result['differences']
    if old_dirs != new_dirs:
        diffs.append({'scope': 'directories', 'old_only': sorted(old_dirs-new_dirs), 'new_only': sorted(new_dirs-old_dirs)})
    for folder in sorted(old_dirs & new_dirs):
        old, new = old_root/folder, new_root/folder
        before_files = {p.name: p for p in old.iterdir() if p.is_file()}
        after_files = {p.name: p for p in new.iterdir() if p.is_file()}
        if before_files.keys() != after_files.keys():
            diffs.append({'scope': folder, 'old_only': sorted(before_files.keys()-after_files.keys()), 'new_only': sorted(after_files.keys()-before_files.keys())})
        for filename in sorted(before_files.keys() & after_files.keys()):
            a, b = content(before_files[filename]), content(after_files[filename])
            scope = f'{folder}/{filename}'
            result['checks'].append({'scope': scope, 'old_sha256': sha(a), 'new_sha256': sha(b),
                                     'old_bytes': len(a), 'new_bytes': len(b), 'equal': a == b})
            if filename in ('runs.csv', 'results.csv'):
                result['runs'] += len(table(b)[1])
            if a == b:
                continue
            difference = {'scope': scope, 'old_bytes': len(a), 'new_bytes': len(b)}
            if filename.endswith('.csv'):
                af, ar = table(a)
                bf, br = table(b)
                difference.update(old_fields=af, new_fields=bf, old_rows=len(ar), new_rows=len(br))
                changes = []
                for index, (previous, current) in enumerate(zip(ar, br)):
                    for field in set(af) | set(bf):
                        if previous.get(field) != current.get(field):
                            changes.append({'row': index, 'seed': current.get('seed'), 'field': field,
                                            'old': previous.get(field), 'new': current.get(field)})
                difference['changed_values'] = sorted(changes, key=lambda c: (c['row'], c['field']))
            else:
                al, bl = a.splitlines(), b.splitlines()
                first = next((i for i, (x, y) in enumerate(zip(al, bl)) if x != y), min(len(al), len(bl)))
                difference.update(old_lines=len(al), new_lines=len(bl), first_changed_line=first+1,
                                  old_context=[x.decode(errors='replace') for x in al[max(0, first-2):first+4]],
                                  new_context=[x.decode(errors='replace') for x in bl[max(0, first-2):first+4]])
            diffs.append(difference)
    result['status'] = 'FAIL' if diffs else 'PASS'
    output.write_text(json.dumps(result, indent=2, sort_keys=True)+'\n')
    print(result['status'], result['runs'], 'runs;', len(result['checks']), 'file checks;', len(diffs), 'differences')
    raise SystemExit(bool(diffs))


if __name__ == '__main__':
    main()
