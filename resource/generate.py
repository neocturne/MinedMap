#!/usr/bin/env python3

import json
import os
import sys


if len(sys.argv) != 3:
	sys.exit('Usage: extract.py <colors.json> <block_types.rs>')

with open(sys.argv[1]) as f:
	colors = json.load(f)

output = {}

with open(sys.argv[2], 'w') as f:
	print('use enumflags2::make_bitflags;', file=f);
	print('', file=f)
	print('use super::*;', file=f)
	print('', file=f)
	print('pub const BLOCK_TYPES: &[(&str, BlockType)] = &[', file=f)

	for name, info in colors.items():
		flags = []
		if info['opaque']:
			flags.append('Opaque')
		if info['grass']:
			flags.append('Grass')
		if info['foliage']:
			flags.append('Foliage')
		if info['birch']:
			flags.append('Birch')
		if info['spruce']:
			flags.append('Spruce')
		if info['water']:
			flags.append('Water')
		flags = 'make_bitflags!(BlockFlag::{' + '|'.join(flags) + '})'

		print('\t("%s", BlockType { flags: %s, color: Color([%u, %u, %u]) }),' % (
			name,
			flags,
			info['color']['r'],
			info['color']['g'],
			info['color']['b'],
		), file=f)
	print('];', file=f)
