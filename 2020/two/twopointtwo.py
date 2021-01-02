#!/user/bin/env python3

from sys import argv

def parse(filename):
  db = []
  with open(filename, 'r') as infile:
    for line in infile:
      pol = {}
      policy,password = line.split(":")
      pol["min"] = policy.split(" ")[0].split("-")[0]
      pol["max"] = policy.split(" ")[0].split("-")[1]
      pol["chr"] = policy.split(" ")[1]
      pol["pwd"] = password.strip()
      db.append(pol)
  return (db)

def check(policy):
  first = int(policy["min"])-1 #not zero indexed
  second = int(policy["max"])-1
  char = policy["chr"]
  password = policy["pwd"]
  if password[first] != char and \
     password[second] != char:
    return (1) #neither position matches
  if password[first] == char and \
     password[second] == char:
    return (2) #both position matches
  if password[first] == char or \
     password[second] == char:
    return (0) #only one position matches
  exit(1) #Something went wrong


if __name__ == '__main__':
  db = parse(argv[1])
  for policy in db:
    result = check(policy)
    if result != 0:
      print(f'[*] Password "{policy["pwd"]}" violated policy "{policy["min"]}-{policy["max"]} {policy["chr"]}"')
      if result == 1:
        print('  - Neither character matches')
      if result == 2:
        print('  - Both characters match')
    else:
      print(f'[+] Password "{policy["pwd"]}" meets policy "{policy["min"]}-{policy["max"]} {policy["chr"]}"')

