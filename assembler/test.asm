START 300
BEGIN: READ NUM
LOOP: MOVEM AREG NUM
PRINT NUM
MUL AREG NUM
COMP AREG X
BC LT LOOP
STOP
NUM: DS 2
HUNDRED: DC 100
END 