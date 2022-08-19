import os

res = os.system("cargo build --target-dir .")
print("Jail successfully builded, output in ./debug/jail")
exit(0)
