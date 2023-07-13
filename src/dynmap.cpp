#include <thread>
#include <mutex>
#include "dynmap.hpp"

Dynmap cavernDynmap("https://thecavern.net/map/towny/");
TownyData townyData;
std::mutex townyDataMutex;
PlayerData playerData;
std::mutex playerDataMutex;

std::thread townyDataUpdater([]() {
    while(true) {
        auto data = cavernDynmap.getTownyData();
        townyDataMutex.lock();
        townyData = data;
        townyDataMutex.unlock();
    };
});
std::thread playerDataUpdater([]() {
    while(true) {
        auto data = cavernDynmap.getPlayerData();
        playerDataMutex.lock();
        playerData = data;
        playerDataMutex.unlock();
    };
});