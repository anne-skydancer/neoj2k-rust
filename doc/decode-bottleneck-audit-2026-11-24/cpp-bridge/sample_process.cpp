// Samples only the primary thread of a benchmark process created here.
#define NOMINMAX
#include <windows.h>
#include <dbghelp.h>
#include <mmsystem.h>
#include <algorithm>
#include <chrono>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>
std::wstring quote(const std::wstring& s){std::wstring o=L"\"";size_t n=0;for(wchar_t c:s){if(c==L'\\'){++n;continue;}if(c==L'\"'){o.append(n*2+1,L'\\');o+=c;n=0;continue;}o.append(n,L'\\');n=0;o+=c;}o.append(n*2,L'\\');o+=L'\"';return o;}
std::string csv(const char* s){std::string o="\"";for(;*s;++s){if(*s=='\"')o+='\"';o+=*s;}return o+'\"';}
int wmain(int argc,wchar_t** argv){
 if(argc<4){std::cerr<<"usage: sampler output.csv target.exe [target args...]\n";return 2;}
 std::wstring command;for(int i=2;i<argc;++i){if(i>2)command+=L' ';command+=quote(argv[i]);}
 std::vector<wchar_t> mutable_command(command.begin(),command.end());mutable_command.push_back(0);
 STARTUPINFOW si{};si.cb=sizeof(si);PROCESS_INFORMATION pi{};
 if(!CreateProcessW(argv[2],mutable_command.data(),nullptr,nullptr,FALSE,CREATE_NO_WINDOW,nullptr,nullptr,&si,&pi)){std::cerr<<"CreateProcess error "<<GetLastError()<<'\n';return 3;}
 SetProcessAffinityMask(pi.hProcess,128);SetProcessAffinityMask(GetCurrentProcess(),1);
 timeBeginPeriod(1);Sleep(250);
 SymSetOptions(SYMOPT_LOAD_LINES|SYMOPT_UNDNAME|SYMOPT_DEFERRED_LOADS);
 const BOOL symbols=SymInitialize(pi.hProcess,nullptr,TRUE);
 std::unordered_map<DWORD64,size_t> counts;size_t failures=0;
 const auto start=std::chrono::steady_clock::now();
 while(WaitForSingleObject(pi.hProcess,0)==WAIT_TIMEOUT){
  if(std::chrono::steady_clock::now()-start>std::chrono::seconds(60)){TerminateProcess(pi.hProcess,99);break;}
  CONTEXT c{};c.ContextFlags=CONTEXT_CONTROL;
  if(SuspendThread(pi.hThread)!=static_cast<DWORD>(-1)){
   const BOOL ok=GetThreadContext(pi.hThread,&c);ResumeThread(pi.hThread);
   if(ok)++counts[c.Rip];else ++failures;
  }else ++failures;
  Sleep(2);
 }
 timeEndPeriod(1);DWORD exitcode=0;GetExitCodeProcess(pi.hProcess,&exitcode);
 std::vector<std::pair<DWORD64,size_t>> sorted(counts.begin(),counts.end());std::sort(sorted.begin(),sorted.end(),[](auto a,auto b){return a.second>b.second;});
 std::ofstream out{std::wstring(argv[1])};out<<"samples,address,module,function,file,line\n";
 size_t total=0;for(auto [addr,count]:sorted){total+=count;char storage[sizeof(SYMBOL_INFO)+4096]{};auto* symbol=reinterpret_cast<SYMBOL_INFO*>(storage);symbol->SizeOfStruct=sizeof(SYMBOL_INFO);symbol->MaxNameLen=4095;DWORD64 displacement=0;const BOOL named=symbols&&SymFromAddr(pi.hProcess,addr,&displacement,symbol);
 IMAGEHLP_LINE64 line{};line.SizeOfStruct=sizeof(line);DWORD linedisp=0;const BOOL located=symbols&&SymGetLineFromAddr64(pi.hProcess,addr,&linedisp,&line);
 IMAGEHLP_MODULE64 module{};module.SizeOfStruct=sizeof(module);const BOOL mod=symbols&&SymGetModuleInfo64(pi.hProcess,addr,&module);
 out<<count<<",0x"<<std::hex<<addr<<std::dec<<','<<csv(mod?module.ModuleName:"")<<','<<csv(named?symbol->Name:"")<<','<<csv(located?line.FileName:"")<<','<<(located?line.LineNumber:0)<<'\n';
 }
 std::cout<<"PID "<<pi.dwProcessId<<" samples "<<total<<" unique_pc "<<counts.size()<<" context_failures "<<failures<<" symbol_init "<<symbols<<" target_exit "<<exitcode<<'\n';
 if(symbols)SymCleanup(pi.hProcess);CloseHandle(pi.hThread);CloseHandle(pi.hProcess);return exitcode==0?0:4;
}
