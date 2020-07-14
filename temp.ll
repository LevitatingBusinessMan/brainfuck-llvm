declare i8* @calloc(i32, i32)
declare void @free(i8*)
declare i32 @putchar(i32)
declare i32 @getchar()

define i32 @main() {
    %stack = call i8* @calloc(i32 1000, i32 1)

    %sp = alloca i8*
    store i8* %stack , i8** %sp

    br label %l1
    
l1:
    %1 = load i8*, i8** %sp
    %2 = load i8, i8* %1
    %3 = icmp eq i8 %2, 0
    br i1 %3, label %l3, label %l2
    
l2:
    %4 = load i8*, i8** %sp
    %5 = load i8, i8* %4
    %6 = add i8 %5, 1
    store i8 %6, i8* %4
    %7 = load i8*, i8** %sp
    %8 = load i8, i8* %7
    %9 = add i8 %8, 1
    store i8 %9, i8* %7
    %10 = load i8*, i8** %sp
    %11 = load i8, i8* %10
    %12 = add i8 %11, 1
    store i8 %12, i8* %10
    %13 = load i8*, i8** %sp
    %14 = load i8, i8* %13
    %15 = add i8 %14, 1
    store i8 %15, i8* %13
    %16 = load i8*, i8** %sp
    %17 = load i8, i8* %16
    %18 = add i8 %17, 1
    store i8 %18, i8* %16
    %19 = load i8*, i8** %sp
    %20 = load i8, i8* %19
    %21 = add i8 %20, 1
    store i8 %21, i8* %19
    %22 = load i8*, i8** %sp
    %23 = load i8, i8* %22
    %24 = add i8 %23, 1
    store i8 %24, i8* %22
    %25 = load i8*, i8** %sp
    %26 = load i8, i8* %25
    %27 = add i8 %26, 1
    store i8 %27, i8* %25
    %28 = load i8*, i8** %sp
    %29 = load i8, i8* %28
    %30 = add i8 %29, 1
    store i8 %30, i8* %28
    %31 = load i8*, i8** %sp
    %32 = load i8, i8* %31
    %33 = add i8 %32, 1
    store i8 %33, i8* %31
    %34 = load i8*, i8** %sp
    %35 = load i8, i8* %34
    %36 = add i8 %35, 1
    store i8 %36, i8* %34
    %37 = load i8*, i8** %sp
    %38 = load i8, i8* %37
    %39 = add i8 %38, 1
    store i8 %39, i8* %37
    %40 = load i8*, i8** %sp
    %41 = load i8, i8* %40
    %42 = add i8 %41, 1
    store i8 %42, i8* %40
    %43 = load i8*, i8** %sp
    %44 = load i8, i8* %43
    %45 = add i8 %44, 1
    store i8 %45, i8* %43
    %46 = load i8*, i8** %sp
    %47 = load i8, i8* %46
    %48 = add i8 %47, 1
    store i8 %48, i8* %46
    %49 = load i8*, i8** %sp
    %50 = load i8, i8* %49
    %51 = add i8 %50, 1
    store i8 %51, i8* %49
    %52 = load i8*, i8** %sp
    %53 = load i8, i8* %52
    %54 = add i8 %53, 1
    store i8 %54, i8* %52
    %55 = load i8*, i8** %sp
    %56 = load i8, i8* %55
    %57 = add i8 %56, 1
    store i8 %57, i8* %55
    %58 = load i8*, i8** %sp
    %59 = load i8, i8* %58
    %60 = add i8 %59, 1
    store i8 %60, i8* %58
    %61 = load i8*, i8** %sp
    %62 = load i8, i8* %61
    %63 = add i8 %62, 1
    store i8 %63, i8* %61
    %64 = load i8*, i8** %sp
    %65 = load i8, i8* %64
    %66 = add i8 %65, 1
    store i8 %66, i8* %64
    %67 = load i8*, i8** %sp
    %68 = load i8, i8* %67
    %69 = add i8 %68, 1
    store i8 %69, i8* %67
    %70 = load i8*, i8** %sp
    %71 = load i8, i8* %70
    %72 = add i8 %71, 1
    store i8 %72, i8* %70
    %73 = load i8*, i8** %sp
    %74 = load i8, i8* %73
    %75 = add i8 %74, 1
    store i8 %75, i8* %73
    %76 = load i8*, i8** %sp
    %77 = load i8, i8* %76
    %78 = add i8 %77, 1
    store i8 %78, i8* %76
    %79 = load i8*, i8** %sp
    %80 = load i8, i8* %79
    %81 = add i8 %80, 1
    store i8 %81, i8* %79
    %82 = load i8*, i8** %sp
    %83 = load i8, i8* %82
    %84 = add i8 %83, 1
    store i8 %84, i8* %82
    %85 = load i8*, i8** %sp
    %86 = load i8, i8* %85
    %87 = add i8 %86, 1
    store i8 %87, i8* %85
    br label %l1
    
l3:
    

    call void @free(i8* %stack)
    ret i32 0
}


