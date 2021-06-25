#!/usr/bin/env python3

import argparse
import os
import sys

def blocklist(path):
	blockpath = os.path.join(path, 'assets', 'minecraft', 'blockstates')
	try:
		return map(
			lambda filename: filename[:-5],
			filter(
				lambda filename: filename.endswith('.json'),
				os.listdir(blockpath),
			),
		)
	except FileNotFoundError:
		sys.exit('''
Path '{}' not found.
Please pass a directory containing the unpacked contents of a JAR of a recent Minecraft version.
		'''.strip().format(blockpath))

def print_blocklist(path):
	for block in sorted(blocklist(path)):
		print(block)

def diff_blocklist(old, new, cmd):
	blocks_old = set(blocklist(old))
	blocks_new = set(blocklist(new))
	diff = sorted(blocks_old.symmetric_difference(blocks_new))

	for block in diff:
		if block in blocks_old:
			if cmd == 'removed':
				print(block)
			elif cmd == 'diff':
				print('-', block)
		else:
			if cmd == 'added':
				print(block)
			elif cmd == 'diff':
				print('+', block)


parser = argparse.ArgumentParser(description='List block IDs of an unpacked Minecraft JAR, or diff two versions')

subparsers = parser.add_subparsers(dest='cmd', metavar='COMMAND', required=True)

parse_list = subparsers.add_parser('list', help='list supported blocks')
parse_list.add_argument('DIR')

parse_added = subparsers.add_parser('added', help='list added blocks')
parse_added.add_argument('OLD')
parse_added.add_argument('NEW')

parse_removed = subparsers.add_parser('removed', help='list removed blocks')
parse_removed.add_argument('OLD')
parse_removed.add_argument('NEW')

parse_removed = subparsers.add_parser('diff', help='diff lists of supported blocks')
parse_removed.add_argument('OLD')
parse_removed.add_argument('NEW')

args = parser.parse_args()

if args.cmd == 'list':
	print_blocklist(args.DIR)
else:
	diff_blocklist(args.OLD, args.NEW, args.cmd)
