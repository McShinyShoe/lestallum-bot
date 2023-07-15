#include <dpp/dpp.h>
#include "../dynmap.hpp"

extern Dynmap cavernDynmap;
extern PlayerData playerData;
extern std::mutex playerDataMutex;

void whereis(dpp::cluster &bot, const dpp::slashcommand_t &event) {
    std::string playerSearched = std::get<std::string>(event.get_parameter("playername"));
    
    playerDataMutex.lock();
    auto players = playerData;
    playerDataMutex.unlock();
    
    dpp::message msg("Player not found!");
    msg.set_flags(dpp::m_ephemeral);
    for(auto& player : players) {
        if(player.first == playerSearched) {
            dpp::embed emb;
            emb.set_color(0x0099ff);
            emb.set_author(player.first, "https://thecavern.net/map/#world;flat;" + std::to_string(player.second.location.x) + "," + std::to_string(player.second.location.y) + "," + std::to_string(player.second.location.z) + ";0", "https://thecavern.net/map/towny/tiles/faces/16x16/" + player.first + ".png");
            emb.set_thumbnail("https://mc-heads.net/body/" + player.first + "/right");
            emb.add_field("Health", std::to_string(player.second.health), true);
            emb.add_field("Armor", std::to_string(player.second.armor), true);
            emb.add_field("", "", true);
            emb.add_field("World", player.second.location.world, true);
            emb.add_field("Location", "(" + std::to_string(player.second.location.x) + ", " + std::to_string(player.second.location.y) + ", " + std::to_string(player.second.location.z) + ")", true);
            emb.set_image(cavernDynmap.getMapImage(player.second.location.x, player.second.location.z));
            emb.set_footer("mc.thecavern.net", "https://cdn.craftingstore.net/rPPmDHlLQ1/ee136bb3e7c876f01863bb782b7a5377/jfbfb33i46btkmnp9yon.png");
            emb.set_timestamp(time(0));
            msg.set_content("");
            msg.add_embed(emb);
        }
    }
    event.reply(msg);
    
};