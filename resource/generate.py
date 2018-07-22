#!/usr/bin/env python3

import json
import os
import sys


if len(sys.argv) != 3:
	sys.exit('Usage: extract.py <colors.json> <BlockType.inc>')

with open(sys.argv[1]) as f:
	colors = json.load(f)

output = {}

with open(sys.argv[2], 'w') as f:
	for name, info in colors.items():
		print('{"%s", {%s, %s, %s, {%u, %u, %u}}},' % (
			name,
			['false', 'true'][info['opaque']],
			['false', 'true'][info['green']],
			['false', 'true'][info['blue']],
			info['color']['r'],
			info['color']['g'],
			info['color']['b'],
		), file=f)