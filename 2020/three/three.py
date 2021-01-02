#!/user/bin/env python3

from sys import argv

def parse_map(filename):
  mapp = {}
  x = 0
  y = 0
  width = 0
  with open(filename, 'r') as infile:
    for line in infile:
      for char in line:
        if char != '\n':
          mapp[(x,y)] = char
          width = x
        x = x + 1
      y = y + 1
      x = 0
  return (mapp, width)

def parse_instructions(instructions):
  pos = 0
  x = 0
  y = 0
  while pos < len(instructions):
    char = instructions[pos]
    if char == 'r':
      pos = pos + 1
      x = x + int(instructions[pos])
    elif char == 'l':
      pos = pos + 1
      x = x - int(instructions[pos])
    elif char == 'd':
      pos = pos + 1
      y = y + int(instructions[pos])
    elif char == 'u':
      pos = pos + 1
      y = y - int(instructions[pos])
    pos = pos + 1
  return (x, y)

def route(mapp, width, movement):
  x = 0
  y = 0
  trees = 0
  try:
    while 1:
      x = x + movement[0]
      x = x % (width+1)
      y = y + movement[1]
      print (x, y, mapp[(x,y)])
      if mapp[(x,y)] == '#':
        trees = trees + 1
  except Exception:
    return (trees)

if __name__ == '__main__':
  mapp,width = parse_map(argv[1])
  print(width)
  result = parse_instructions(argv[2])
  trees = route(mapp, width, result)
  print(trees)
