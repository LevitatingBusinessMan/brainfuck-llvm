; Setup stack
%stack = call i8* @calloc(i32 1000, i32 1)

; Create integer which is an offset of the stack pointer
%sp = alloca i32
store i32 0, i32* %sp

; In this case we increase the pointer like this:
%tmp = load i32, i32* %sp
%tmp2 = add i32 1, %tmp
store i32 %tmp2, i32* %sp
