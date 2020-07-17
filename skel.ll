declare i8* @calloc(i32, i32)
declare void @free(i8*)
declare i32 @putchar(i32)
declare i32 @getchar()

define i32 @main() {{
	; Allocate memory for the tape
	%stack = call i8* @calloc(i32 30000, i32 1)

	; Create data pointer
	%sp = alloca i8*
	store i8* %stack , i8** %sp

	;; Code goes here

	call void @free(i8* %stack)
	ret i32 0
}}