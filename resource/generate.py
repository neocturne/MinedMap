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
		flags = []
		if info['opaque']:
			flags.append('BLOCK_OPAQUE')
		if info['grass']:
			flags.append('BLOCK_GRASS')
		if info['foliage']:
			flags.append('BLOCK_FOLIAGE')
		if info['water']:
			flags.append('BLOCK_WATER')
		if flags:
			flags = '|'.join(flags)
		else:
			flags = '0'

		print('{"%s", {%s, {%u, %u, %u}}},' % (
			name,
			flags,
			info['color']['r'],
			info['color']['g'],
			info['color']['b'],
		), file=f)
