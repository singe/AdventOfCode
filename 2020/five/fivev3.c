#include <stdio.h>

int main() {
  char* foo;
  gets(foo);

  int seatid = 0;
  for (int i=0; foo[i] != 0; i++) {
    int y = foo[i] & 4; //only keep 3rd lsb
    y >>= 2;            //shift to 1st lsb
    y ^= 1;             //flip it
    y <<= 9-i;          //put in position
    seatid = seatid | y;//add to final
  }
  //row is first 7 bits, col is last 3 bits
  printf("%d %d %d\n",seatid >> 3, seatid & 7, seatid);

  return 1;
}
