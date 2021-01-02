#!/bin/sh
awk '{ expenses[NR] = $0 }
END { 
  for (i = 1;i <= length(expenses); i++) {
    for (j = 1;j <= length(expenses); j++) { 
      for (k = 1;k <= length(expenses); k++) { 
        print expenses[i] " : " expenses[j] " : " expenses[k]
        if ((expenses[i] + expenses[j] + expenses[k]) == 2020) { 
          print expenses[i]*expenses[j]*expenses[k];
          exit;
        }
      }
    }
  }
}' $1
