import os

print("Entering into ./compiler/jail and building Jail")
res = os.system("cd ./compiler/jail/ && cargo build --target-dir ../../ && cp ../../debug/jail ../../ && rm -rf ../../debug")
print("Jail successfully builded, output in ./jail")
exit(0)
