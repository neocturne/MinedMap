#!/usr/bin/env python3

import json
import os
import sys


if len(sys.argv) != 3:
	sys.exit('Usage: biomes.py <data directory> <biomes.rs>')

biomes = {}

for file in os.scandir(os.path.join(sys.argv[1], 'data/minecraft/worldgen/biome')):
	(name, ext) = os.path.splitext(file.name)
	if ext != '.json':
		continue
	with open(file) as f:
		data = json.load(f)
	biomes[name] = {
		'downfall': data['downfall'],
		'temperature': data['temperature'],
		'foliage_color': data['effects'].get('foliage_color'),
		'grass_color': data['effects'].get('grass_color'),
		'grass_color_modifier': data['effects'].get('grass_color_modifier'),
		'water_color': data['effects'].get('water_color'),
	}

def color(v):
	return f'[{v>>16}, {(v>>8)&0xff}, {v&0xff}]'

# Converts the snake_case grass color modifier to CamelCase
def modify(v):
	return ''.join([s.capitalize() for s in v.split('_')])

def gen_biome(name, info, f):
	temp = round(100*info['temperature'])
	downfall = round(100*info['downfall'])
	foliage_color = info['foliage_color']
	grass_color = info['grass_color']
	grass_color_modifier = info['grass_color_modifier']
	water_color = info['water_color']

	print(f'\t("{name}", Biome::new({temp}, {downfall})', file=f)

	if foliage_color is not None:
		print(f'\t\t.foliage({color(foliage_color)})', file=f)
	if grass_color is not None:
		print(f'\t\t.grass({color(grass_color)})', file=f)
	if grass_color_modifier is not None:
		print(f'\t\t.modify({modify(grass_color_modifier)})', file=f)
	if water_color is not None and water_color != 0x3f76e4:
		print(f'\t\t.water({color(water_color)})', file=f)

	print('\t),', file=f)

with open(sys.argv[2], 'w') as f:
	print('//! Biome data', file=f);
	print('//!', file=f);
	print('//! This file is generated using resource/biomes.py, do not edit', file=f);
	print('', file=f)
	print('use super::*;', file=f)
	print('use BiomeGrassColorModifier::*;', file=f)
	print('', file=f)
	print('/// List if known biomes and their properties', file=f);
	print('pub const BIOMES: &[(&str, Biome)] = &[', file=f)

	for name in sorted(biomes):
		gen_biome(name, biomes[name], f)

	print('];', file=f)
