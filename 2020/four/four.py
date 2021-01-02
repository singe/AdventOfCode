#!/user/bin/env python3

from sys import argv
import re

def parse(filename):
  db = []
  with open(filename, 'r') as infile:
    for passport in re.split('\n\n',infile.read().strip()):
      entry = {}
      for record in passport.replace('\n',' ').split(' '):
        key = record.split(':')[0]
        value = record.split(':')[1]
        entry[key] = value
      db.append(entry)
  return (db) 

def check(entry):
  fields = ['byr','iyr','eyr','hgt','hcl','ecl','pid']
  if len(entry) < 7:
    return 1 # not enough total fields
  for field in fields:
    if entry.get(field) == None:
      return 2 # missing required fields
  if entry.get('cid') == None:
    return 3 # valid but northpole
  return 0 # normal valid

if __name__ == '__main__':
  db = parse(argv[1])
  for entry in db:
    ret = check(entry)
    if ret == 0:
      print ('Valid')
    elif ret == 3:
      print ('Valid - North Pole')
    elif ret == 1:
      print ('Invalid - not enough fields')
    elif ret == 2:
      print ('Invalid - missing required fields')
