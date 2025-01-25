section .bss
   tape resb 30000
section .text
    global _start
_start:
    mov rbx, 0
    cmp rbx, 30000
    jne skip0
    mov rbx, 0
skip0: 
    inc rbx
    cmp byte tape[rbx], 255
    jne skip1
    mov byte tape[rbx], 0
skip1: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip2
    mov byte tape[rbx], 0
skip2: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip3
    mov byte tape[rbx], 0
skip3: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip4
    mov byte tape[rbx], 0
skip4: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip5
    mov byte tape[rbx], 0
skip5: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip6
    mov byte tape[rbx], 0
skip6: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip7
    mov byte tape[rbx], 0
skip7: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip8
    mov byte tape[rbx], 0
skip8: 
    inc byte tape[rbx]
again9: 
    cmp byte tape[rbx], 0
    je skip10
    cmp rbx, 0
    jne skip11
    mov rbx, 29999
skip11: 
    dec rbx
    cmp byte tape[rbx], 255
    jne skip12
    mov byte tape[rbx], 0
skip12: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip13
    mov byte tape[rbx], 0
skip13: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip14
    mov byte tape[rbx], 0
skip14: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip15
    mov byte tape[rbx], 0
skip15: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip16
    mov byte tape[rbx], 0
skip16: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip17
    mov byte tape[rbx], 0
skip17: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip18
    mov byte tape[rbx], 0
skip18: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip19
    mov byte tape[rbx], 0
skip19: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip20
    mov byte tape[rbx], 0
skip20: 
    inc byte tape[rbx]
    cmp rbx, 30000
    jne skip21
    mov rbx, 0
skip21: 
    inc rbx
    cmp byte tape[rbx], 0
   jne skip22
    mov byte tape[rbx], 255
skip22: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
    jne again9
skip10: 
    cmp rbx, 0
    jne skip23
    mov rbx, 29999
skip23: 
    dec rbx
    call _print
    cmp rbx, 30000
    jne skip24
    mov rbx, 0
skip24: 
    inc rbx
    cmp byte tape[rbx], 255
    jne skip25
    mov byte tape[rbx], 0
skip25: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip26
    mov byte tape[rbx], 0
skip26: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip27
    mov byte tape[rbx], 0
skip27: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip28
    mov byte tape[rbx], 0
skip28: 
    inc byte tape[rbx]
again29: 
    cmp byte tape[rbx], 0
    je skip30
    cmp rbx, 0
    jne skip31
    mov rbx, 29999
skip31: 
    dec rbx
    cmp byte tape[rbx], 255
    jne skip32
    mov byte tape[rbx], 0
skip32: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip33
    mov byte tape[rbx], 0
skip33: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip34
    mov byte tape[rbx], 0
skip34: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip35
    mov byte tape[rbx], 0
skip35: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip36
    mov byte tape[rbx], 0
skip36: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip37
    mov byte tape[rbx], 0
skip37: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip38
    mov byte tape[rbx], 0
skip38: 
    inc byte tape[rbx]
    cmp rbx, 30000
    jne skip39
    mov rbx, 0
skip39: 
    inc rbx
    cmp byte tape[rbx], 0
   jne skip40
    mov byte tape[rbx], 255
skip40: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
    jne again29
skip30: 
    cmp rbx, 0
    jne skip41
    mov rbx, 29999
skip41: 
    dec rbx
    cmp byte tape[rbx], 255
    jne skip42
    mov byte tape[rbx], 0
skip42: 
    inc byte tape[rbx]
    call _print
    cmp byte tape[rbx], 255
    jne skip43
    mov byte tape[rbx], 0
skip43: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip44
    mov byte tape[rbx], 0
skip44: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip45
    mov byte tape[rbx], 0
skip45: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip46
    mov byte tape[rbx], 0
skip46: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip47
    mov byte tape[rbx], 0
skip47: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip48
    mov byte tape[rbx], 0
skip48: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip49
    mov byte tape[rbx], 0
skip49: 
    inc byte tape[rbx]
    call _print
    call _print
    cmp byte tape[rbx], 255
    jne skip50
    mov byte tape[rbx], 0
skip50: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip51
    mov byte tape[rbx], 0
skip51: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip52
    mov byte tape[rbx], 0
skip52: 
    inc byte tape[rbx]
    call _print
    cmp rbx, 30000
    jne skip53
    mov rbx, 0
skip53: 
    inc rbx
    cmp rbx, 30000
    jne skip54
    mov rbx, 0
skip54: 
    inc rbx
    cmp byte tape[rbx], 255
    jne skip55
    mov byte tape[rbx], 0
skip55: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip56
    mov byte tape[rbx], 0
skip56: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip57
    mov byte tape[rbx], 0
skip57: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip58
    mov byte tape[rbx], 0
skip58: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip59
    mov byte tape[rbx], 0
skip59: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip60
    mov byte tape[rbx], 0
skip60: 
    inc byte tape[rbx]
again61: 
    cmp byte tape[rbx], 0
    je skip62
    cmp rbx, 0
    jne skip63
    mov rbx, 29999
skip63: 
    dec rbx
    cmp byte tape[rbx], 255
    jne skip64
    mov byte tape[rbx], 0
skip64: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip65
    mov byte tape[rbx], 0
skip65: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip66
    mov byte tape[rbx], 0
skip66: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip67
    mov byte tape[rbx], 0
skip67: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip68
    mov byte tape[rbx], 0
skip68: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip69
    mov byte tape[rbx], 0
skip69: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip70
    mov byte tape[rbx], 0
skip70: 
    inc byte tape[rbx]
    cmp rbx, 30000
    jne skip71
    mov rbx, 0
skip71: 
    inc rbx
    cmp byte tape[rbx], 0
   jne skip72
    mov byte tape[rbx], 255
skip72: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
    jne again61
skip62: 
    cmp rbx, 0
    jne skip73
    mov rbx, 29999
skip73: 
    dec rbx
    cmp byte tape[rbx], 255
    jne skip74
    mov byte tape[rbx], 0
skip74: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip75
    mov byte tape[rbx], 0
skip75: 
    inc byte tape[rbx]
    call _print
    cmp byte tape[rbx], 0
   jne skip76
    mov byte tape[rbx], 255
skip76: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip77
    mov byte tape[rbx], 255
skip77: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip78
    mov byte tape[rbx], 255
skip78: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip79
    mov byte tape[rbx], 255
skip79: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip80
    mov byte tape[rbx], 255
skip80: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip81
    mov byte tape[rbx], 255
skip81: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip82
    mov byte tape[rbx], 255
skip82: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip83
    mov byte tape[rbx], 255
skip83: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip84
    mov byte tape[rbx], 255
skip84: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip85
    mov byte tape[rbx], 255
skip85: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip86
    mov byte tape[rbx], 255
skip86: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip87
    mov byte tape[rbx], 255
skip87: 
    dec byte tape[rbx]
    call _print
    cmp rbx, 30000
    jne skip88
    mov rbx, 0
skip88: 
    inc rbx
    cmp byte tape[rbx], 255
    jne skip89
    mov byte tape[rbx], 0
skip89: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip90
    mov byte tape[rbx], 0
skip90: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip91
    mov byte tape[rbx], 0
skip91: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip92
    mov byte tape[rbx], 0
skip92: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip93
    mov byte tape[rbx], 0
skip93: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip94
    mov byte tape[rbx], 0
skip94: 
    inc byte tape[rbx]
again95: 
    cmp byte tape[rbx], 0
    je skip96
    cmp rbx, 0
    jne skip97
    mov rbx, 29999
skip97: 
    dec rbx
    cmp byte tape[rbx], 255
    jne skip98
    mov byte tape[rbx], 0
skip98: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip99
    mov byte tape[rbx], 0
skip99: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip100
    mov byte tape[rbx], 0
skip100: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip101
    mov byte tape[rbx], 0
skip101: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip102
    mov byte tape[rbx], 0
skip102: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip103
    mov byte tape[rbx], 0
skip103: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip104
    mov byte tape[rbx], 0
skip104: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip105
    mov byte tape[rbx], 0
skip105: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip106
    mov byte tape[rbx], 0
skip106: 
    inc byte tape[rbx]
    cmp rbx, 30000
    jne skip107
    mov rbx, 0
skip107: 
    inc rbx
    cmp byte tape[rbx], 0
   jne skip108
    mov byte tape[rbx], 255
skip108: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
    jne again95
skip96: 
    cmp rbx, 0
    jne skip109
    mov rbx, 29999
skip109: 
    dec rbx
    cmp byte tape[rbx], 255
    jne skip110
    mov byte tape[rbx], 0
skip110: 
    inc byte tape[rbx]
    call _print
    cmp rbx, 0
    jne skip111
    mov rbx, 29999
skip111: 
    dec rbx
    call _print
    cmp byte tape[rbx], 255
    jne skip112
    mov byte tape[rbx], 0
skip112: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip113
    mov byte tape[rbx], 0
skip113: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip114
    mov byte tape[rbx], 0
skip114: 
    inc byte tape[rbx]
    call _print
    cmp byte tape[rbx], 0
   jne skip115
    mov byte tape[rbx], 255
skip115: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip116
    mov byte tape[rbx], 255
skip116: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip117
    mov byte tape[rbx], 255
skip117: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip118
    mov byte tape[rbx], 255
skip118: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip119
    mov byte tape[rbx], 255
skip119: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip120
    mov byte tape[rbx], 255
skip120: 
    dec byte tape[rbx]
    call _print
    cmp byte tape[rbx], 0
   jne skip121
    mov byte tape[rbx], 255
skip121: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip122
    mov byte tape[rbx], 255
skip122: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip123
    mov byte tape[rbx], 255
skip123: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip124
    mov byte tape[rbx], 255
skip124: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip125
    mov byte tape[rbx], 255
skip125: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip126
    mov byte tape[rbx], 255
skip126: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip127
    mov byte tape[rbx], 255
skip127: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
   jne skip128
    mov byte tape[rbx], 255
skip128: 
    dec byte tape[rbx]
    call _print
    cmp rbx, 30000
    jne skip129
    mov rbx, 0
skip129: 
    inc rbx
    cmp rbx, 30000
    jne skip130
    mov rbx, 0
skip130: 
    inc rbx
    cmp rbx, 30000
    jne skip131
    mov rbx, 0
skip131: 
    inc rbx
    cmp byte tape[rbx], 255
    jne skip132
    mov byte tape[rbx], 0
skip132: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip133
    mov byte tape[rbx], 0
skip133: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip134
    mov byte tape[rbx], 0
skip134: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip135
    mov byte tape[rbx], 0
skip135: 
    inc byte tape[rbx]
again136: 
    cmp byte tape[rbx], 0
    je skip137
    cmp rbx, 0
    jne skip138
    mov rbx, 29999
skip138: 
    dec rbx
    cmp byte tape[rbx], 255
    jne skip139
    mov byte tape[rbx], 0
skip139: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip140
    mov byte tape[rbx], 0
skip140: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip141
    mov byte tape[rbx], 0
skip141: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip142
    mov byte tape[rbx], 0
skip142: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip143
    mov byte tape[rbx], 0
skip143: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip144
    mov byte tape[rbx], 0
skip144: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip145
    mov byte tape[rbx], 0
skip145: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip146
    mov byte tape[rbx], 0
skip146: 
    inc byte tape[rbx]
    cmp rbx, 30000
    jne skip147
    mov rbx, 0
skip147: 
    inc rbx
    cmp byte tape[rbx], 0
   jne skip148
    mov byte tape[rbx], 255
skip148: 
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
    jne again136
skip137: 
    cmp rbx, 0
    jne skip149
    mov rbx, 29999
skip149: 
    dec rbx
    cmp byte tape[rbx], 255
    jne skip150
    mov byte tape[rbx], 0
skip150: 
    inc byte tape[rbx]
    call _print
    cmp rbx, 30000
    jne skip151
    mov rbx, 0
skip151: 
    inc rbx
    cmp byte tape[rbx], 255
    jne skip152
    mov byte tape[rbx], 0
skip152: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip153
    mov byte tape[rbx], 0
skip153: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip154
    mov byte tape[rbx], 0
skip154: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip155
    mov byte tape[rbx], 0
skip155: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip156
    mov byte tape[rbx], 0
skip156: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip157
    mov byte tape[rbx], 0
skip157: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip158
    mov byte tape[rbx], 0
skip158: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip159
    mov byte tape[rbx], 0
skip159: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip160
    mov byte tape[rbx], 0
skip160: 
    inc byte tape[rbx]
    cmp byte tape[rbx], 255
    jne skip161
    mov byte tape[rbx], 0
skip161: 
    inc byte tape[rbx]
    call _print

_end:
    mov rax, 60
    mov rdi, 0
    syscall
_print:
    mov rax, 1
    mov rdi, 1
    lea rsi, [tape+rbx]
    mov rdx, 1
    syscall
    ret
