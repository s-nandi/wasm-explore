#include <http_utilities.hpp>
#include <scene_utilities.hpp>
#include <string>

void workflow() {
    auto path = "hello_world.glb";
    auto asset = scene_utils::load(path);
    int i = 0;
    for (auto camera : asset.cameras()) {
        auto image = scene_utils::render(asset, camera);
        // do layering here
        http::upload(image.bytes(), image.name() + std::to_string(i));
        i++;
    }
}