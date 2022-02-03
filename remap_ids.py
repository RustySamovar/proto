#!/usr/bin/env python3

import csv
import sys
import re

def mint(x):
    if x:
        return int(x)
    else:
        return None

def read_map(filename):
    with open(filename, 'rt') as csvfile:
        r = csv.reader(csvfile)
        for row in r:
            yield mint(row[1]), int(row[2])

num = re.compile("^([\t ]+)([A-Za-z]+) = ([0-9]+),$")

_map = dict(read_map(sys.argv[1]))

for line in sys.stdin:
    m = num.match(line)
    if m:
        _ident = m.group(1)
        _id = int(m.group(3))
        _new_id = _map.get(_id)
        _text = m.group(2).rstrip()
        if _new_id:
            print ("\t\t{} = {:4d}, {}".format(_text, _new_id, " // TODO: automapped"))
        else:
            print ("\t\t#{} = {:4d},".format(_text, _id))
    else:
        print (line.rstrip())
