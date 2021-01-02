./fivev3 input|cut -d\  -f12|sort -n > input-seatids
seq 0 1024 > all-seatids
diff all-seatids input-seatids|grep -A3 "^---$"|tail -n1
