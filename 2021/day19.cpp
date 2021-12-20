#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <unordered_set>

#include <glm/glm.hpp>
#include <glm/gtx/string_cast.hpp>

using vec3 = glm::vec<3, int>;
using mat3 = glm::mat<3, 3, int>;

template<>
struct std::hash<vec3> {
    std::size_t operator() (vec3 const& vec) const noexcept {
        std::hash<std::string> shash;

        return shash(glm::to_string(vec));
    }
};

class Scanner {
private: 
    std::vector<vec3> beacons;
    bool complete = false;
    vec3 disp;
    mat3 rot;
    int id;
public:
    Scanner (std::vector<vec3>&& _beacons, int _id) : beacons(_beacons), id(_id) {}

    const std::vector<vec3>& getBeacons () {
        return beacons;
    }

    bool isComplete() {
        return complete;
    }

    vec3 getDisp () {
        return disp;
    }

    mat3 getRot () {
        return rot;
    }

    void setStart () {
        disp = {0, 0, 0};
        rot = {{1, 0, 0}, {0, 1, 0}, {0, 0, 1}};
        complete = true;
    }

    std::vector<vec3> getRelBeacons () {
        std::vector<vec3> vec;
        for (auto& i : beacons) {
            vec.emplace_back(i * rot + disp);
        }
        return vec;
    }

    std::vector<std::vector<int>> getRelSizes ();

    std::vector<std::pair<vec3, vec3>> getCommonBeacons (Scanner& other);

    bool completeLocation (Scanner& other);
};

std::string stringStrip (std::string& s);
std::vector<std::string> stringSplit (const std::string& s, std::string delim);

Scanner getScanner (std::fstream& file, int id);
std::vector<Scanner> getScanners (std::fstream& file);

template <typename T>
int getCommonElements (std::vector<T>& l1, std::vector<T>& l2);

int sqSize (vec3 a);
std::vector<mat3> getRotationMatrices ();
int getManhattenDistance (vec3 a, vec3 b);

int main (int argc, char* argv[]) {
    auto file = std::fstream("data/day19.txt");

    std::vector<Scanner> scanVec = getScanners(file);

    auto vec = scanVec[0].getCommonBeacons(scanVec[1]);

    std::vector<Scanner> complete;
    complete.push_back(scanVec[0]);
    scanVec.erase(scanVec.begin());
    complete[0].setStart();

    vec = complete[0].getCommonBeacons(scanVec[0]);

    std::size_t considerIndex = 0;
    while (!scanVec.empty() && considerIndex < complete.size()) {
        auto consider = complete[considerIndex];
        scanVec.erase(std::remove_if(scanVec.begin(), scanVec.end(), [&consider, &complete] (Scanner& element) {
            if (element.completeLocation(consider)) {
                complete.push_back(element);
                return true;   
            }
            return false;
        }));

        considerIndex++;
    }

    std::unordered_set<vec3> beaconSet{};

    for (auto& i : complete) {
        auto vec = i.getRelBeacons();

        beaconSet.insert(vec.begin(), vec.end());
    }

    std::cout << "Number of beacons: " << beaconSet.size() << "\n";

    auto max = 0;

    for (auto& i : complete) {
        for (auto& j : complete) {
            auto n = getManhattenDistance(i.getDisp(), j.getDisp());
            if (n > max) {
                max = n;
            }
        }
    }

    std::cout << "Max manhatten distance: " << max << "\n";

    return 0;

}

std::vector<Scanner> getScanners (std::fstream& file) {
    std::vector<Scanner> scanVec;
    int curr = 0;
    while (file) {
        std::string s;
        std::getline(file, s);
        if (stringStrip(s) == "") {
            break;
        }
        scanVec.push_back(getScanner(file, curr++));
    }

    return scanVec;
}

Scanner getScanner (std::fstream& file, int id) {
    std::string s;
    std::vector<vec3> vecs;
    while (file) {
        std::getline(file, s);
        if (stringStrip(s).empty()) {
            break;
        }
        //std::cout<< "a\n";
        auto v = stringSplit(stringStrip(s), ",");
        vecs.emplace_back(vec3{std::stoi(v[0]), std::stoi(v[1]), std::stoi((v[2]))});
    }
    return Scanner(std::move(vecs), id);
}

template <typename T>
int getCommonElements (std::vector<T>& l1, std::vector<T>& l2) {
    std::size_t i = 0;
    std::size_t j = 0;
    int common = 0;
    while (i < l1.size() && j < l2.size()) {
        if (l1[i] == l2[j]) {
            common++;
            i++;
            j++;
        } else if (l1[i] < l2[j]) {
            i++;
        } else {
            j++;
        }
    }
    return common;
}

int sqSize (vec3 a) {
    return a.x * a.x + a.y * a.y + a.z * a.z;
}

std::vector<mat3> getRotationMatrices () {
    std::vector<mat3> directions = {
        mat3{{1, 0, 0}, {0, 1, 0}, {0, 0, 1}},
        mat3{{0, 0, 1}, {0, 1, 0}, {-1, 0, 0}},
        mat3{{-1, 0, 0}, {0, 1, 0}, {0, 0, -1}},
        mat3{{0, 0, -1}, {0, 1, 0}, {1, 0, 0}},
        mat3{{1, 0, 0}, {0, 0, 1}, {0, -1, 0}},
        mat3{{1, 0, 0}, {0, 0, -1}, {0, 1, 0}}
    };

    std::vector<mat3> rotations = {
        mat3{{1, 0, 0}, {0, 1, 0}, {0, 0, 1}},
        mat3{{0, -1, 0}, {1, 0, 0}, {0, 0, 1}},
        mat3{{-1, 0, 0}, {0, -1, 0}, {0, 0, 1}},
        mat3{{0, 1, 0}, {-1, 0, 0}, {0, 0, 1}}
    };

    std::vector<mat3> matrices;

    for (auto& i : directions) {
        for (auto& j : rotations) {
            matrices.push_back(i * j);
        }
    }
    return matrices;
}

int getManhattenDistance (vec3 a, vec3 b) {
    auto diff = a - b;
    return abs(diff.x) + abs(diff.y) + abs(diff.z);
}

std::string stringStrip (std::string& s) {
    std::string s2 = s;
    s2.erase(s.begin(), std::find_if(s.begin(), s.end(), std::not1(std::ptr_fun<int, int>(std::isspace))));
    return s2;
}

std::vector<std::string> stringSplit (const std::string& s, std::string delim) {
    std::size_t pos = 0;
    std::vector<std::string> vec;
    std::size_t oldPos = 0;
    while ((pos = s.find(delim, oldPos)) != std::string::npos) {
        vec.push_back(s.substr(oldPos, pos));
        oldPos = pos + 1;
    }
    vec.push_back(s.substr(oldPos));
    return vec;
}

std::vector<std::vector<int>> Scanner::getRelSizes () {
    std::vector<std::vector<int>> selfSizes;

    for (auto& i : beacons) {
        std::vector<int> vec;
        for (auto& j : beacons) {
            vec.push_back(sqSize(j - i));
        }
        std::sort(vec.begin(), vec.end());

        selfSizes.push_back(std::move(vec));
    }
    return selfSizes;
}

std::vector<std::pair<vec3, vec3>> Scanner::getCommonBeacons (Scanner& other) {
    auto selfSizes = getRelSizes();
    auto otherSizes = other.getRelSizes();

    std::vector<std::pair<vec3, vec3>> common;

    for (std::size_t i = 0; i < selfSizes.size(); i++) {
        for (std::size_t j = 0; j < otherSizes.size(); j++) {
            if (getCommonElements(selfSizes[i], otherSizes[j]) >= 12) {
                common.push_back({beacons[i], other.getBeacons()[j]});
            }
        }
    }
    return common;
}

bool Scanner::completeLocation (Scanner& other) {
    if (!other.isComplete()) {
        return false;
    }

    auto common = getCommonBeacons(other);

    if (common.size() < 12) {
        return false;
    }

    for (auto& i : getRotationMatrices()) {
        vec3 currDisp{};
        bool found = false;
        for (auto& p : common) {
            if (found) {
                if (p.second != p.first * i + currDisp) {
                    found = false;
                    break;
                }
            } else {
                currDisp = p.second - p.first * i;
                found = true;
            }
        }
        if (found) {
            glm::mat3 m = other.getRot();
            disp = other.getDisp() + (vec3)(glm::inverse(m) * currDisp);
            rot = i * other.getRot();
            complete = true;
            return true;
        }
    }
    return false;
}