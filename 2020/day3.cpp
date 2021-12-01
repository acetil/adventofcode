#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <utility>


int getTrees (std::vector<std::string>& map, int x, int y) {
    int xPos = 0;
    int width = map[0].length();
    int numTrees = 0;
    for (size_t i = 0 ; i < map.size(); i += y) {
        if (map[i][xPos] == '#') {
            numTrees++;
        }
        xPos = (xPos + x) % width;
    }
    return numTrees;
}

int main (int argc, char* argv[]) {
    std::vector<std::string> map;

    std::ifstream file("day3.txt");

    while (file) {
        std::string s;
        std::getline(file, s);
        map.emplace_back(s);
        //std::cout << "Reading line!" << std::endl;
    }

    std::vector<std::pair<int, int>> tests ({{1,1}, {3, 1}, {5, 1}, {7, 1}, {1, 2}});
    int total = 1;
    for (auto p : tests) {
        int trees = getTrees(map, p.first, p.second);
        std::cout << "Num of trees for right " << p.first << " and down " << p.second << " is " << trees << std::endl;
        total *= trees;
    }

    std::cout << "Total: " << total << std::endl;

    return 0;
}