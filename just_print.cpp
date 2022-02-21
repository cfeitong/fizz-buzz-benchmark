#include <unistd.h>

const int N = 65536;
char buf[N];

int main() {
    for (int i = 0; i < N; i++) buf[i] = 'A';
    while (1) {
        char *out = buf;
        do {
            int w = write(1, out, &out[65536] - out);
            if (w > 0) out += w;
        } while (out < &buf[N]);
    }
    return 0;
}