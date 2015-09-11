from ctypes import cdll

lib = cdll.LoadLibrary("target/release/embeber.dll")

lib.procesar()

print("completado!")
