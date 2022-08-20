import os

print("Entering into ./compiler/jail and building Jail")
res = os.system("cd ./compiler/jail/ && cargo build --target-dir ../../")
print("Jail successfully builded, output in ./debug/jail")
exit(0)
