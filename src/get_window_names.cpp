// https://gist.github.com/Recognition101/1e28655eece7f1169951
#include <iostream>
#include <Carbon/Carbon.h>

// compile with: clang++ -framework carbon -framework foundation appNames.cpp -o test

int main(int argc, char *argv[]) {
    char *b1 = (char *)malloc(400);
    char *b2 = (char *)malloc(400);
    int layer;

    CFArrayRef windowList = CGWindowListCopyWindowInfo(kCGWindowListOptionOnScreenOnly, kCGNullWindowID);

    CFIndex numWindows = CFArrayGetCount( windowList );

    for( int i = 0; i < (int)numWindows; i++ ) {
        CFDictionaryRef info = (CFDictionaryRef)CFArrayGetValueAtIndex(windowList, i);

        CFStringRef appName = (CFStringRef)CFDictionaryGetValue(info, kCGWindowOwnerName);
        CFStringRef winName = (CFStringRef)CFDictionaryGetValue(info, kCGWindowName);
        CFNumberGetValue((CFNumberRef)CFDictionaryGetValue(info, kCGWindowLayer), kCFNumberIntType, &layer);

//        std::cout << "appName " << appName << "\n";
//        std::cout << "winName " << winName << "\n";
//        std::cout << "layer " << layer << "\n";

        if (appName != 0 && winName != 0 && layer == 0) {
            CFStringGetCString(appName, b1, 400, kCFStringEncodingUTF8);
            CFStringGetCString(winName, b2, 400, kCFStringEncodingUTF8);
            std::cout << b1 << "," << b2 << "\n";
        }
    }

    CFRelease(windowList);
}