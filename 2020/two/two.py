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
  occurances = policy["pwd"].count(policy["chr"]) 
  if occurances > int(policy["max"]):
    return (1)
  if occurances < int(policy["min"]):
    return (2)
  return (0)


if __name__ == '__main__':
  db = parse(argv[1])
  for policy in db:
    result = check(policy)
    if result != 0:
      print(f'[*] Password "{policy["pwd"]}" violated policy "{policy["min"]}-{policy["max"]} {policy["chr"]}"')
      if result == 1:
        print('  - Password exceeds maximum number of characters')
      if result == 2:
        print('  - Password doesn\'t meet minimum number of characters')
    else:
      print(f'[+] Password "{policy["pwd"]}" meets policy "{policy["min"]}-{policy["max"]} {policy["chr"]}"')

