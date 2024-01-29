void workflow();

extern "C" void run() {
    workflow();
}

int main() {
    run();
}