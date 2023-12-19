; ModuleID = 'probe4.6cf85f6b5da5e399-cgu.0'
source_filename = "probe4.6cf85f6b5da5e399-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@alloc_83fbc6600f0fb3504e593ff89b06a2e6 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/3f28fe133475ec5faf3413b556bf3cfb0d51336c/library/core/src/num/mod.rs" }>, align 1
@alloc_19cf114d4092d1673ba828c0f43bfbab = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_83fbc6600f0fb3504e593ff89b06a2e6, [16 x i8] c"K\00\00\00\00\00\00\00y\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal unnamed_addr constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17h8b0e77b5d017669cE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7836b7f895dafb0eE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hd00c90271e931644E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_19cf114d4092d1673ba828c0f43bfbab) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7836b7f895dafb0eE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hd00c90271e931644E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.76.0-nightly (3f28fe133 2023-12-18)"}
