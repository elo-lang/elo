; ModuleID = 'test'
source_filename = "test"

declare void @exit(i32)

define i32 @main() {
entry:
  call void @exit(i32 10)
  ret i32 0
}