import os

print("Entering into ./src/jail and building Jail")
res = os.system("cd ./src/jail/ && cargo build --target-dir ../../")
print("Jail successfully builded, output in ./debug/jail")
exit(0)
