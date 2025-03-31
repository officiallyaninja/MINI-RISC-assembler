MOV R0, R1
LOADBR Inner_Loop
Loop:
	ADD R3, R1, R4
	SET R5.3
  MOV R7, #0
  Inner_Loop:
    INC R0
    CMP R0, R7
    CPL EQ
    JF EQ 
  LOADBR Loop
  JF P
HALT
    
  

