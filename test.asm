ld $0 #2
ld $1 #3
ld $2 #100
loop: add $0 $1 $1
gt $1 $2
jne #12
