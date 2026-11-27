// SPDX-License-Identifier: BSD-2-Clause
#define wmain standalone_probe_entry
#include "decode_probe.cpp"
#undef wmain
#include <neoj2k/neoj2k.h>
#include <iostream>
#include <memory>

int wmain(int argc, wchar_t** argv) {
    SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);
    if (argc != 9) return 2;
    try {
        bool kakadu = std::wstring(argv[1]) == L"kdu";
        if (!kakadu && std::wstring(argv[1]) != L"cpp") throw std::runtime_error("Unknown actor");
        int width=std::stoi(argv[6]), height=std::stoi(argv[7]), channels=std::stoi(argv[8]);
        auto input=read_file(argv[3]); auto expected=read_file(argv[4]);
        Bytes storage(expected.size()+128, 0xcd);
        if (std::size_t(width)*height*channels != expected.size()) throw std::runtime_error("Invalid geometry");
        std::unique_ptr<Codec> kdu;
        if (kakadu) {
            if (sha256(read_file(argv[2])) != "dcd2a3726a3ad382681d55c14eec0120fbc5ba4c93279440cc7089f3ec4927da") throw std::runtime_error("Wrong private binary");
            auto module=LoadLibraryExW(argv[2],nullptr,LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR|LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);
            if (!module) throw std::runtime_error("Cannot load private image");
            kdu=std::make_unique<Codec>(module);
        }
        std::vector<nj2k_plane_t> planes;
        for (int c=0;c<channels;++c) planes.push_back({storage.data()+64+c,std::int64_t(width)*channels,static_cast<NJ2K_UINT32>(channels),static_cast<NJ2K_UINT32>(c)});
        nj2k_output_t output{sizeof(output),NJ2K_SAMPLE_U8,static_cast<NJ2K_UINT32>(channels),planes.data()};
        auto decode=[&] {
            if (kakadu) {
                Dims geometry{}; int count{};
                kdu->decode(input,storage,geometry,count);
                if (geometry.w!=width || geometry.h!=height || count!=channels) throw std::runtime_error("KDU geometry changed");
            } else {
                auto codec=nj2k_create_decompress(NJ2K_CODEC_J2K);
                if (!codec) throw std::runtime_error("C++ create failed");
                struct Guard {nj2k_codec_t* c; ~Guard(){nj2k_destroy_codec(c);}} guard{codec};
                nj2k_dparameters_t params{}; nj2k_set_default_decoder_parameters(&params);
                if (!nj2k_setup_decoder(codec,&params)) throw std::runtime_error("C++ setup failed");
                nj2k_image_t image{};
                if (!nj2k_read_header_memory(codec,input.data(),input.size(),&image) || nj2k_get_status(codec)!=NJ2K_OK) throw std::runtime_error("C++ header failed");
                if (image.numcomps!=channels) throw std::runtime_error("C++ components changed");
                for (int c=0;c<channels;++c) {
                    const auto& comp=image.comps[c];
                    if (comp.w!=width || comp.h!=height || comp.prec!=8 || comp.sgnd) throw std::runtime_error("C++ geometry changed");
                }
                if (!nj2k_decode_memory(codec,input.data(),input.size(),&output) || nj2k_get_status(codec)!=NJ2K_OK) throw std::runtime_error("C++ decode failed");
            }
        };
        auto verify=[&] {
            std::fill(storage.begin(),storage.end(),0xcd); decode();
            if (!std::equal(expected.begin(),expected.end(),storage.begin()+64)) throw std::runtime_error("Pixel mismatch");
            if (!std::all_of(storage.begin(),storage.begin()+64,[](auto b){return b==0xcd;}) || !std::all_of(storage.end()-64,storage.end(),[](auto b){return b==0xcd;})) throw std::runtime_error("Guard overwritten");
        };
        verify();
        std::ofstream pixels(std::filesystem::path(argv[5]),std::ios::binary);
        pixels.write(reinterpret_cast<const char*>(storage.data()+64),expected.size());pixels.close();
        if (!pixels) throw std::runtime_error("Cannot write pixels");
        std::cout << "{\"ready\":true}" << std::endl;
        int iterations,warm;
        while (std::cin >> iterations >> warm) {
            if(iterations<1 || iterations>100000 || warm<0 || warm>1000) throw std::runtime_error("Bad batch");
            verify();
            for(int i=0;i<warm;++i) decode();
            auto start=std::chrono::steady_clock::now();
            for(int i=0;i<iterations;++i) decode();
            double us=std::chrono::duration<double,std::micro>(std::chrono::steady_clock::now()-start).count()/iterations;
            verify();
            std::printf("{\"us\":%.9f,\"validated\":true}\n",us); std::fflush(stdout);
        }
        ExitProcess(0);
    } catch(const std::exception& e) {std::fprintf(stderr,"%s\n",e.what());return 4;}
    catch(...) {std::fprintf(stderr,"Native exception\n");return 6;}
}
