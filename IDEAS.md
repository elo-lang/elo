## There are 2 (actually 3) options on how we are going to compile Oxide:

- 1st Option: Compile directly to Assembly for each platform;
- 2nd Option: Compile to a compiler backend;
  * Inside this option, I have 3 main ideas:
    * 1st Idea: Classical LLVM interface
    * 2nd Idea: Use QBE compiler backend
    * 3rd Idea: Create our own compiler backend

- 3rd Option: We can also try to compile it to a VM like JVM or Erlang's VM or even create our own VM in rust
