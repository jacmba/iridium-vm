ld $0 #2
ld $1 #3
ld $2 #100
ld $3 #20
ld $4 #36
loop: add $0 $1 $1
gt $1 $2
jeq $4
jmp $3
end: hlt