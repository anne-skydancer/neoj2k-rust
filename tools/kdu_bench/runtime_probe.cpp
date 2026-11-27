// SPDX-License-Identifier: BSD-2-Clause
// Copyright (c) 2026, Anne Skydancer (neo-j2k)
// Benchmark-only child. No viewer startup function is invoked.
#include <windows.h>
#include <cstdio>
#include <stdexcept>
#include <string>
#include <fstream>
#include <cstdint>

template<class T>T symbol(HMODULE module,const char* name) {
    auto p=GetProcAddress(module,name);
    if(!p)throw std::runtime_error(std::string("Missing export: ")+name);
    return reinterpret_cast<T>(p);
}
int wmain(int argc,wchar_t** argv) {
    SetErrorMode(SEM_FAILCRITICALERRORS|SEM_NOGPFAULTERRORBOX);
    if(argc!=2)return 2;
    // Refuse an ordinary EXE or any image with an entry point. The Python driver
    // creates a private DLL-flagged, entry-point-disabled copy before this call.
    std::ifstream image(argv[1], std::ios::binary);
    IMAGE_DOS_HEADER dos{};
    image.read(reinterpret_cast<char*>(&dos), sizeof(dos));
    if (!image || dos.e_magic != IMAGE_DOS_SIGNATURE || dos.e_lfanew < 0) return 5;
    image.seekg(dos.e_lfanew);
    IMAGE_NT_HEADERS64 nt{};
    image.read(reinterpret_cast<char*>(&nt), sizeof(nt));
    if (!image || nt.Signature != IMAGE_NT_SIGNATURE ||
        nt.FileHeader.Machine != IMAGE_FILE_MACHINE_AMD64 ||
        nt.OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR64_MAGIC ||
        !(nt.FileHeader.Characteristics & IMAGE_FILE_DLL) ||
        nt.OptionalHeader.AddressOfEntryPoint != 0) {
        std::fprintf(stderr, "Refusing image without the private no-entry DLL preparation\n");
        return 5;
    }
    HMODULE module=LoadLibraryExW(argv[1],nullptr,LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR|LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);
    if(!module){std::fprintf(stderr,"LoadLibraryEx failed:%lu\n",GetLastError());return 3;}
    try {
        auto version=symbol<const char*(*)()>(module,"?kdu_get_core_version@kdu_core@@YAPEBDXZ");
        auto processors=symbol<int(*)()>(module,"?kdu_get_num_processors@kdu_core@@YAHXZ");
        std::printf("{\"version\":\"%s\",\"processors\":%d,\"viewer_entry_point\":false}\n",version(),processors());
        std::fflush(stdout);
        // Imported runtimes/TLS can have shutdown assumptions; exit this isolated probe process.
        ExitProcess(0);
    }catch(const std::exception& e){std::fprintf(stderr,"%s\n",e.what());return 4;}
}
