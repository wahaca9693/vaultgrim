# Vaultgrim Python Bindings
# This module provides Python access to Vaultgrim encryption

from .vaultgrim import encrypt, decrypt, keygen

__version__ = "1.0.0"
__all__ = ["encrypt", "decrypt", "keygen"]
