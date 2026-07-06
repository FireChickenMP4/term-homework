#include "init/init.h"

int main() {
    auto& app = drogon::app();
    init(app);
    app.run();
}
