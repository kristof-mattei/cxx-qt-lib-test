#include <QtPlugin>

void _init_qt_resources() {
    std::fprintf(stderr, "Initializing resources...\n");
    Q_INIT_RESOURCE(qml_module_resources_qrc);
    std::fprintf(stderr, "Done initializing resources...\n");
}

extern "C" {
    void init_qt_resources() {
        _init_qt_resources();
    }
}
