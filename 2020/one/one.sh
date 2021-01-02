#!/bin/sh
awk '{ expenses[NR] = $0 }
END { 
  for (i = 1;i <= length(expenses); i++) {
    for (j = 1;j <= length(expenses); j++) { 
      if (expenses[i] + expenses[j] == 2020) { 
        print expenses[i]*expenses[j];
        exit;
      }
    }
  }
}' input
