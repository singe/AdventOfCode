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

def inrange(string,minimum,maximum):
  try:
    num = int(string)
  except ValueError:
    return False
  if num >= minimum and num <= maximum:
    return True
  else:
    return False
  
def check(entry):
  fields = ['byr','iyr','eyr','hgt','hcl','ecl','pid']
  if len(entry) < 7:
    return 100 # not enough total fields
  for field in fields:
    if entry.get(field) == None:
      return 200 # missing required fields

  if not inrange(entry['byr'],1920,2002):
    return 300 # birth year out of range
  if not inrange(entry['iyr'],2010,2020):
    return 400 # birth year out of range
  if not inrange(entry['eyr'],2020,2030):
    return 500 # birth year out of range
  if entry['hgt'][-2:] != 'cm' and entry['hgt'][-2:] != 'in':
    return 600 # height not in right format
  if entry['hgt'][-2:] == 'cm':
    if not inrange(entry['hgt'][0:-2],150,193):
      return 650 # height in cm out of range
  elif entry['hgt'][-2:] == 'in':
    if not inrange(entry['hgt'][0:-2],59,76):
      return 700 # height in cm out of range
  if not re.match('^#[0-9a-f]{6}$',entry['hcl']):
    return 800 # hair colour wrong
  cols = ['amb','blu','brn','gry','grn','hzl','oth']
  if entry['ecl'] not in cols:
    return 900 # eye colour wrong
  if len(entry['pid']) != 9:
    return 1000 # passport length wrong
  if not inrange(entry['pid'],0,999999999):
    return 1100 # passport not a number

  if entry.get('cid') == None:
    return 1 # valid but northpole
  return 0 # normal valid

if __name__ == '__main__':
  db = parse(argv[1])
  for entry in db:
    ret = check(entry)
    if ret < 100:
      print ('Valid')
    if ret >= 100:
      print ('Invalid')

    if ret == 0:
      print ('  - Perfect')
    elif ret == 1:
      print ('  - North Pole')
    elif ret == 100:
      print ('  - not enough fields')
    elif ret == 200:
      print ('  - missing required fields')
    elif ret == 300:
      print ('  - birth year out of range')
    elif ret == 400:
      print ('  - issue year out of range')
    elif ret == 500:
      print ('  - expiration year out of range')
    elif ret == 600:
      print ('  - height not cm or in')
    elif ret == 650:
      print ('  - height in cm out of range')
    elif ret == 700:
      print ('  - height in in out of range')
    elif ret == 800:
      print ('  - not a hair colour')
    elif ret == 900:
      print ('  - not a eye colour')
    elif ret == 1000:
      print ('  - passport number not nine')
    elif ret == 1100:
      print ('  - passport number not a number')
