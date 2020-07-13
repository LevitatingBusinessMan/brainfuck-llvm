declare i8* @calloc(i32, i32)
declare void @free(i8*)
declare i32 @putchar(i32)
declare i32 @getchar()

define i32 @main() {
	; Setup cell stack in heap
	%stack = call i8* @calloc(i32 1000, i32 1)

	; Setup stack pointer
	%sp = alloca i8*
	store i8* %stack , i8** %sp

	; Code here

	call void @free(i8* %stack)
	ret i32 0
}
