#!/usr/bin/env python3
# SPDX-License-Identifier: BSD-2-Clause
# Copyright (c) 2026, Anne Skydancer (neo-j2k)
"""Inspect exported Kakadu in a user-supplied viewer; isolated, no viewer entry point."""
import argparse,ctypes,hashlib,json,os,struct,subprocess,sys
from pathlib import Path
VERSION_SYMBOL='?kdu_get_core_version@kdu_core@@YAPEBDXZ'
class PE:
    def __init__(self,path):
        self.path=Path(path).resolve();self.data=self.path.read_bytes();d=self.data
        if d[:2]!=b'MZ':raise ValueError('Not a PE file')
        self.pe=self.u32(0x3c)
        if d[self.pe:self.pe+4]!=b'PE\0\0':raise ValueError('Invalid PE signature')
        self.opt=self.pe+24;self.machine=self.u16(self.pe+4)
        if self.machine!=0x8664 or self.u16(self.opt)!=0x20b:raise ValueError('Requires an x64 PE32+ image')
        self.base=struct.unpack_from('<Q',d,self.opt+24)[0];self.size=self.u32(self.opt+56);self.dirs=self.opt+112
        self.sections=[]
        for n in range(self.u16(self.pe+6)):
            off=self.opt+self.u16(self.pe+20)+40*n
            self.sections.append((self.u32(off+12),max(self.u32(off+8),self.u32(off+16)),self.u32(off+20)))
        self.exports={};er=self.u32(self.dirs)
        if er:
            e=self.rva(er);names=self.rva(self.u32(e+32));ords=self.rva(self.u32(e+36));functions=self.rva(self.u32(e+28))
            for n in range(self.u32(e+24)):
                name=self.string(self.rva(self.u32(names+n*4)))
                self.exports[name]=self.u32(functions+self.u16(ords+n*2)*4)
    def u16(self,p):return struct.unpack_from('<H',self.data,p)[0]
    def u32(self,p):return struct.unpack_from('<I',self.data,p)[0]
    def rva(self,r):
        for va,size,raw in self.sections:
            if va<=r<va+size:
                offset=raw+r-va
                if offset>=len(self.data):raise ValueError('RVA outside file')
                return offset
        raise ValueError('RVA outside sections')
    def string(self,p):
        end=self.data.find(b'\0',p)
        if end<0:raise ValueError('Unterminated PE string')
        return self.data[p:end].decode('ascii')
    def inspect(self):
        r=self.exports[VERSION_SYMBOL];code=self.data[self.rva(r):self.rva(r)+8]
        # Calling is allowed only for a verified import-free LEA/RET version accessor.
        if code[:3]!=bytes.fromhex('488d05')or code[7]!=0xc3:
            raise ValueError('Version getter differs from the verified import-free LEA/RET pattern')
        target=r+7+int.from_bytes(code[3:7],'little',signed=True)
        return {'viewer':str(self.path),'sha256':hashlib.sha256(self.data).hexdigest(),'machine':'x64',
                'exports':len(self.exports),'kdu_exports':sum('kdu_'in n.lower()for n in self.exports),
                'version':self.string(self.rva(target)),'version_rva':r,'version_string_rva':target,'entry_point_invoked':False}

def child(path):
    if os.name!='nt' or ctypes.sizeof(ctypes.c_void_p)!=8:raise RuntimeError('Native probe requires64-bit Windows Python')
    pe=PE(path);result=pe.inspect()
    k=ctypes.WinDLL('kernel32',use_last_error=True)
    k.LoadLibraryExW.argtypes=[ctypes.c_wchar_p,ctypes.c_void_p,ctypes.c_uint32];k.LoadLibraryExW.restype=ctypes.c_void_p
    k.GetProcAddress.argtypes=[ctypes.c_void_p,ctypes.c_char_p];k.GetProcAddress.restype=ctypes.c_void_p
    k.FreeLibrary.argtypes=[ctypes.c_void_p];k.FreeLibrary.restype=ctypes.c_int
    # DONT_RESOLVE_DLL_REFERENCES: do not run entry point, TLS callbacks or import initialization.
    module=k.LoadLibraryExW(str(pe.path),None,1)
    if not module:raise ctypes.WinError(ctypes.get_last_error())
    try:
        address=k.GetProcAddress(module,VERSION_SYMBOL.encode('ascii'))
        if address!=module+result['version_rva']:raise RuntimeError('Mapped export differs from inspected RVA')
        code=ctypes.string_at(address,8)
        if code!=pe.data[pe.rva(result['version_rva']):pe.rva(result['version_rva'])+8]:raise RuntimeError('Mapped code changed')
        pointer=ctypes.WINFUNCTYPE(ctypes.c_void_p)(address)()
        if pointer!=module+result['version_string_rva']:raise RuntimeError('Unexpected version pointer')
        actual=ctypes.string_at(pointer,len(result['version'])+1).rstrip(b'\0').decode('ascii')
        if actual!=result['version']:raise RuntimeError('Version return mismatch')
        result.update(native_call_passed=True,returned_version=actual,mode='verified import-free version call only')
        print(json.dumps(result),flush=True)
    finally:k.FreeLibrary(module)

if __name__=='__main__':
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--viewer',type=Path,required=True)
    ap.add_argument('--child',action='store_true');ap.add_argument('--inspect-only',action='store_true')
    ap.add_argument('--json',type=Path);args=ap.parse_args()
    if args.child:child(args.viewer)
    else:
        if args.inspect_only:result=PE(args.viewer).inspect()
        else:
            r=subprocess.run([sys.executable,str(Path(__file__).resolve()),'--viewer',str(args.viewer.resolve()),'--child'],capture_output=True,text=True,timeout=20,creationflags=getattr(subprocess,'CREATE_NO_WINDOW',0))
            if r.returncode:raise RuntimeError(f'Isolated probe failed ({r.returncode}): {r.stderr}')
            result=json.loads(r.stdout)
        if args.json:args.json.write_text(json.dumps(result,indent=2)+'\n')
        print(json.dumps(result,indent=2))
