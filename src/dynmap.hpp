#pragma once
#include "../lib/json/json.hpp"
#include <unordered_map>
#include <fstream>
#include <mutex>

#ifndef CURL_SILENT
#define CURL_SILENT 1
#endif

#define DYNMAP_LOCATION "standalone/dynmap_world.json"
#define MARKER_LOCATION "tiles/_markers_/marker_world.json"

struct Location {
    int64_t x, y, z;
    std::string world;
};

struct Player {
    std::string name;
    std::string nick;
    size_t armor;
    size_t health;
    Location location;
};

struct Town {
    std::string name;
    std::string mayor;
    std::string nation;
    bool ruined;
    struct {
        bool pvp;
        bool mobs;
        bool explosion;
        bool fire;
    } permission;
    std::vector<std::string> residents;
    Location spawn;
};

struct Nation {
    std::string mapColor;
    std::string capital;
    std::vector<std::string> towns;
};

typedef std::pair<std::unordered_map<std::string, Town>, std::unordered_map<std::string, Nation>> TownyData;
typedef std::unordered_map<std::string, Player> PlayerData;

class Dynmap {
public:
    std::string location;
    TownyData getTownyData() {
        TownyData r;
        auto &[towns, nations] = r;
        system(std::string((CURL_SILENT ? "curl -s " : "curl ") + location + MARKER_LOCATION + " -o marker.json").c_str());
        nlohmann::json dynmap_json = nlohmann::json::parse(std::ifstream("marker.json"));

        for (auto &town : dynmap_json["sets"]["towny.markerset"]["markers"]) {
            Town &currentTown = towns[town["label"]];
            currentTown.name = town["label"];
            currentTown.spawn.x = town["x"];
            currentTown.spawn.y = town["y"];
            currentTown.spawn.z = town["z"];
            currentTown.spawn.world = "world";
            std::string desc = town["desc"];
            currentTown.nation = desc.substr(desc.find('(') + 1, desc.find(')', desc.find('(') + 1) - (desc.find('(') + 1));
            currentTown.mayor = desc.substr(desc.find("Mayor") + 37, desc.find('<', desc.find("Mayor") + 37) - (desc.find("Mayor") + 37));
            currentTown.permission.pvp = desc.substr(desc.find("pvp") + 5, desc.find('<', desc.find("pvp") + 5) - (desc.find("pvp") + 5)) == "false" ? 0 : 1;
            currentTown.permission.mobs = desc.substr(desc.find("mobs") + 6, desc.find('<', desc.find("mobs") + 6) - (desc.find("mobs") + 6)) == "false" ? 0 : 1;
            currentTown.permission.explosion = desc.substr(desc.find("explosion") + 11, desc.find('<', desc.find("explosion") + 11) - (desc.find("explosion") + 11)) == "false" ? 0 : 1;
            currentTown.permission.fire = desc.substr(desc.find("fire") + 6, desc.find('<', desc.find("fire") + 6) - (desc.find("fire") + 6)) == "false" ? 0 : 1;
            currentTown.ruined = desc.substr(desc.find("ruined") + 8, desc.find('<', desc.find("ruined") + 8) - (desc.find("ruined") + 8)) == "false" ? 0 : 1;
        }
        return r;
    };
    PlayerData getPlayerData() {
        PlayerData r;
        system(std::string((CURL_SILENT ? "curl -s " : "curl ") + location + DYNMAP_LOCATION + " -o dynmap.json").c_str());
        nlohmann::json dynmap_json = nlohmann::json::parse(std::ifstream("dynmap.json"));

        for (auto &player : dynmap_json["players"]) {
            Player &currentPlayer = r[player["account"]];
            currentPlayer.name = player["account"];
            currentPlayer.nick = player["name"];
            currentPlayer.armor = player["armor"];
            currentPlayer.health = player["health"];
            currentPlayer.location.world = player["account"];
            currentPlayer.location.x = player["x"];
            currentPlayer.location.y = player["y"];
            currentPlayer.location.z = player["z"];
        }
        return r;
    };
    
    Dynmap(){};
    Dynmap(std::string location) : location(location){};
};