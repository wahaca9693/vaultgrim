#!/usr/bin/env python3
"""
Vaultgrim Python Wrapper
Use Vaultgrim encryption from Python!

Usage:
    from vaultgrim import encrypt, decrypt
    encrypt("file.txt", algorithm="aes-256-gcm")
    decrypt("file.txt.enc")
"""

import subprocess
import os
import sys
from pathlib import Path

class Vaultgrim:
    """Vaultgrim encryption tool wrapper"""
    
    def __init__(self, binary_path=None):
        if binary_path is None:
            # Try common locations
            locations = [
                "./target/release/vaultgrim",
                "~/.vaultgrim/bin/vaultgrim",
                "/usr/local/bin/vaultgrim",
            ]
            for loc in locations:
                path = os.path.expanduser(loc)
                if os.path.exists(path):
                    self.binary = path
                    break
            else:
                raise FileNotFoundError("Vaultgrim binary not found!")
        else:
            self.binary = binary_path
    
    def encrypt(self, input_file, output=None, key=None, password=False, 
                algorithm="aes-256-gcm", secure_delete=False):
        """Encrypt a file"""
        cmd = [self.binary, "encrypt", input_file]
        if output:
            cmd.extend(["--output", output])
        if key:
            cmd.extend(["--key", key])
        if password:
            cmd.append("--password")
        cmd.extend(["--algorithm", algorithm])
        if secure_delete:
            cmd.append("--secure-delete")
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result
    
    def decrypt(self, input_file, output=None, key=None, password=False):
        """Decrypt a file"""
        cmd = [self.binary, "decrypt", input_file]
        if output:
            cmd.extend(["--output", output])
        if key:
            cmd.extend(["--key", key])
        if password:
            cmd.append("--password")
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result
    
    def encrypt_dir(self, input_dir, output, key=None, password=False,
                    algorithm="aes-256-gcm"):
        """Encrypt a directory"""
        cmd = [self.binary, "encrypt-dir", input_dir, "--output", output]
        if key:
            cmd.extend(["--key", key])
        if password:
            cmd.append("--password")
        cmd.extend(["--algorithm", algorithm])
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result
    
    def keygen(self, key_type="x25519", output=None, password=False):
        """Generate keys"""
        cmd = [self.binary, "keygen", "--key-type", key_type]
        if output:
            cmd.extend(["--output", output])
        if password:
            cmd.append("--password")
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result
    
    def benchmark(self):
        """Run benchmark"""
        cmd = [self.binary, "benchmark"]
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result
    
    def algorithms(self):
        """List supported algorithms"""
        cmd = [self.binary, "algorithms", "--verbose"]
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result
    
    def menu(self):
        """Run interactive menu"""
        cmd = [self.binary, "menu"]
        subprocess.run(cmd)

# Convenience functions
def encrypt(input_file, **kwargs):
    """Encrypt a file"""
    vg = Vaultgrim()
    return vg.encrypt(input_file, **kwargs)

def decrypt(input_file, **kwargs):
    """Decrypt a file"""
    vg = Vaultgrim()
    return vg.decrypt(input_file, **kwargs)

def keygen(**kwargs):
    """Generate keys"""
    vg = Vaultgrim()
    return vg.keygen(**kwargs)

if __name__ == "__main__":
    print("Vaultgrim Python Wrapper v1.0.0")
    print("Import from Python: from vaultgrim import encrypt, decrypt")
