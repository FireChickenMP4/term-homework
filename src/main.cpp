#include <drogon/drogon.h>
#include "init/init.h"

int main() {
    auto& app = drogon::app();
    init(app);
    app.addListener("0.0.0.0",8080).run();
}
