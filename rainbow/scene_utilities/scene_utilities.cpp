#include "scene_utilities.hpp"

#include <string>

using namespace std::literals;

namespace scene_utils {

Asset::Asset(std::string name) : name_{name} {
    int num_cameras = std::size(name) / 5;
    cameras_.resize(num_cameras);
    int index = 0;
    for (auto& camera : cameras_) {
        camera = detail::make_camera(index);
        index++;
    }
}

auto Asset::name() const -> std::string {
    return name_;
}

auto Asset::cameras() const -> std::vector<Camera> {
    return cameras_;
}

Image::Image(std::string name) : name_{name} {}

auto Image::name() const -> std::string {
    return name_;
}

auto Image::bytes() -> std::vector<char> {
    std::vector<char> result;
    for (auto c : name_) {
        result.push_back(c);
    }
    return result;
}

auto load(const std::string& path) -> Asset {
    return Asset(path);
}

auto render(const Asset& asset, const Camera& camera) -> Image {
    std::string builder;
    builder += asset.name();
    builder += " ";
    for (auto coord : camera.position) {
        builder += std::to_string(coord);
        builder += ", ";
    }
    for (int i = 0; i < 2; i++) {
        builder.pop_back();
    }
    return Image(builder);
}

auto detail::make_camera(int i) -> Camera {
    return Camera{
        "Camera"s + std::to_string(i),
        {i, i, i},
    };
}

}