#define _CRT_SECURE_NO_WARNINGS 1
#include <stdio.h>
#include <Windows.h>
#include <clocale >
#include <direct.h> // _getcwd

#pragma comment(lib, "user32.lib")
#pragma comment(lib, "advapi32.lib")
#pragma comment(linker, "/subsystem:windows /entry:wmainCRTStartup")

void log(const TCHAR *message)
{
    MessageBoxW(NULL, message, L"log", 0);
}

// https://stackoverflow.com/questions/40664890/copy-unicode-string-to-clipboard-isnt-working/68077157#68077157
void toClipboardWStr(const wchar_t *strData)
{
    if (OpenClipboard(0))
    {
        EmptyClipboard();
        int size_m = sizeof(WCHAR) * (wcslen(strData) + 1);
        HGLOBAL hClipboardData = GlobalAlloc(GMEM_DDESHARE, size_m);
        WCHAR *pchData;
        pchData = (WCHAR *)GlobalLock(hClipboardData);
        //wcscpy(pchData, strData);
        wcscpy_s(pchData, size_m / sizeof(wchar_t), strData);
        GlobalUnlock(hClipboardData);
        SetClipboardData(CF_UNICODETEXT, hClipboardData);
        CloseClipboard();
        // if you need to call this function multiple times, I test no need to GlobalFree, or will occur error
        //GlobalFree(hClipboardData);
    }
}

wchar_t *escape_path(wchar_t *origin)
{
    int origin_len = wcslen(origin);
    wchar_t *ret = (wchar_t *)malloc(origin_len * 4);
    memset(ret, 0, origin_len * 4);
    //wcscpy()
    int j = 0;
    for (int i = 0; i < origin_len; i++)
    {
        if (origin[i] == L'\\')
        {
            ret[j++] = L'\\';
            ret[j++] = L'\\';
        }
        else
        {
            ret[j++] = origin[i];
        }
    }
    return ret;
}

// https://thegeekpage.com/add-any-program-to-right-click-context-menu/

int wmain(int argc, wchar_t *argv[])
{
    if (argc != 2)
    {
        log(L"wrong args");
        exit(0);
    }

    const wchar_t *name = _wcsdup(argv[1]);
    setlocale(LC_ALL, "zh_CN.UTF-8");

    wchar_t abs_path[MAX_PATH] = {0};
    _wfullpath(abs_path, name, MAX_PATH);
    wprintf(L"abs_path: %s \n", abs_path);
    // log(abs_path);
    FILE *file = _wfopen(abs_path, L"r");

    // no good, maybe privileges error
    if (file == NULL)
    {
        wprintf(L"file not exit\n");
        //log(L"file not exit, give up\n");
        exit(0);
    }
    wchar_t *new_path = escape_path(abs_path);
    //log(new_path);
    toClipboardWStr(new_path);
    log(L"file exit, copied path to clipboard\n");
    return 0;
}
