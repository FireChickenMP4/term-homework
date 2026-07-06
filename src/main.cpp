#include <drogon/drogon.h>

int main() {
    drogon::app()
        .setUploadPath("/tmp/drogon_uploads")
        .registerHandler("/", [](const drogon::HttpRequestPtr& req,
                                 std::function<void(const drogon::HttpResponsePtr&)>&& callback) {
            auto resp = drogon::HttpResponse::newHttpResponse();
            resp->setBody("Hello from Drogon!");
            callback(resp);
        })
        .addListener("0.0.0.0", 8080)
        .run();
}
