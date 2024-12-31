#!/usr/bin/env python3

import shutil
import sys

from PIL import Image

MATERIALS = [
	'acacia',
	'bamboo',
	'birch',
	'cherry',
	'crimson',
	'dark_oak',
	'jungle',
	'mangrove',
	'oak',
	'pale_oak',
	'spruce',
	'warped',
]

in_dir = sys.argv[1]
out_dir = sys.argv[2]

def sign_bg_image(material):
	in_path = f'{in_dir}/assets/minecraft/textures/entity/signs/{material}.png'
	out_path = f'{out_dir}/bg/{material}_sign.png'
	out_path_wall = f'{out_dir}/bg/{material}_wall_sign.png'

	in_image = Image.open(in_path)

	out_image = Image.new('RGBA', (24, 26))
	out_image.paste(in_image.crop((2, 2, 26, 14)), (0, 0))
	out_image.paste(in_image.crop((2, 16, 4, 30)), (11, 12))
	out_image.save(out_path)

	out_image = Image.new('RGBA', (24, 12))
	out_image.paste(in_image.crop((2, 2, 26, 14)), (0, 0))
	out_image.save(out_path_wall)

def hanging_sign_bg_image(material):
	in_path = f'{in_dir}/assets/minecraft/textures/gui/hanging_signs/{material}.png'
	out_path = f'{out_dir}/bg/{material}_hanging_sign.png'
	out_path_wall = f'{out_dir}/bg/{material}_hanging_wall_sign.png'

	in_image = Image.open(in_path)

	out_image = Image.new('RGBA', (16, 14))
	out_image.paste(in_image.crop((0, 2, 16, 16)), (0, 0))
	out_image.save(out_path)

	shutil.copyfile(in_path, out_path_wall)


def sign_icon_image(material):
	in_path = f'{in_dir}/assets/minecraft/textures/item/{material}_sign.png'
	out_path = f'{out_dir}/icon/{material}_sign.png'
	out_path_wall = f'{out_dir}/icon/{material}_wall_sign.png'

	in_image = Image.open(in_path)

	out_image = Image.new('RGBA', (13, 14))
	out_image.paste(in_image.crop((2, 2, 15, 16)), (0, 0))
	out_image.save(out_path)

	out_image = Image.new('RGBA', (13, 9))
	out_image.paste(in_image.crop((2, 2, 15, 11)), (0, 0))
	out_image.save(out_path_wall)


def hanging_sign_icon_image(material):
	in_path = f'{in_dir}/assets/minecraft/textures/item/{material}_hanging_sign.png'
	out_path = f'{out_dir}/icon/{material}_hanging_sign.png'
	out_path_wall = f'{out_dir}/icon/{material}_hanging_wall_sign.png'

	in_image = Image.open(in_path)

	out_image = Image.new('RGBA', (14, 12))
	out_image.paste(in_image.crop((1, 3, 15, 15)), (0, 0))
	out_image.save(out_path)

	out_image = Image.new('RGBA', (14, 14))
	out_image.paste(in_image.crop((1, 1, 15, 15)), (0, 0))
	out_image.save(out_path_wall)

for material in MATERIALS:
	sign_bg_image(material)
	hanging_sign_bg_image(material)
	sign_icon_image(material)
	hanging_sign_icon_image(material)
