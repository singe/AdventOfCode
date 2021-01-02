#!/user/bin/env python3

from sys import argv

def parse(filename):
  struct = []
  with open(filename, 'r') as infile:
    for line in infile:
      print('hello world')
  return (struct) 

def check():
  print('hello world')

if __name__ == '__main__':
  db = parse(argv[1])

