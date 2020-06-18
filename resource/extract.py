#!/usr/bin/env python3

import json
import os
import sys

from PIL import Image


if len(sys.argv) != 4:
	sys.exit('Usage: extract.py <blocks.json> <asset directory> <colors.json>')

def mean_color(texture):
	path = os.path.join(sys.argv[2], texture + '.png')
	im = Image.open(path)

	data = im.convert('RGBA').getdata()
	a = sum([a for (r, g, b, a) in data])
	if a == 0:
		return None

	r = sum([r * a for (r, g, b, a) in data])
	g = sum([g * a for (r, g, b, a) in data])
	b = sum([b * a for (r, g, b, a) in data])
	return {
		'r': r / a,
		'g': g / a,
		'b': b / a,
	}


with open(sys.argv[1]) as f:
	blocks = json.load(f)

output = {}

for name, info in blocks.items():
	id = 'minecraft:' + name

	output[id] = {
		'color': {'r': 0, 'g': 0, 'b': 0},
		'opaque': False,
		'grass': False,
		'foliage': False,
		'water': False,
	}

	if info is None:
		continue

	color = mean_color(info.get('texture', name))
	if color:
		output[id]['color'] = color
		output[id]['opaque'] = True
		output[id]['grass'] = info.get('grass', False)
		output[id]['foliage'] = info.get('foliage', False)
		output[id]['water'] = info.get('water', False)

with open(sys.argv[3], 'w') as f:
	json.dump(output, f)
