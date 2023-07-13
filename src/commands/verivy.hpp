#include <dpp/dpp.h>
#include "../dynmap.hpp"

extern std::string VERIFY_ROLE_ID;
extern std::string VERIFY_CHANNEL_ID;

extern PlayerData playerData;
extern std::mutex playerDataMutex;

void verify(dpp::cluster &bot, const dpp::slashcommand_t &event) {
    if(event.command.channel_id != dpp::snowflake(VERIFY_CHANNEL_ID)) {
        event.reply(dpp::message("Not in correct channnel!").set_flags(dpp::m_ephemeral));
        return;
    }

    for(auto it = event.command.member.roles.cbegin(); it < event.command.member.roles.cend(); it++) {
        if(*it == dpp::snowflake(VERIFY_ROLE_ID)) {
            event.reply(dpp::message("User already have the verified role!").set_flags(dpp::m_ephemeral));
            return;
        }
    }

    std::string playerSearched = std::get<std::string>(event.get_parameter("playername"));
    
    playerDataMutex.lock();
    auto players = playerData;
    playerDataMutex.unlock();
    for(auto& player : players) {
        if(player.first == playerSearched) {
            bot.guild_member_add_role(event.command.guild_id, event.command.member.user_id, dpp::snowflake(VERIFY_ROLE_ID));
            event.reply(dpp::message("Verified complete!").set_flags(dpp::m_ephemeral));
            return;
        }
    }
    
    
    event.reply(dpp::message("Player \"" + playerSearched + "\" not online!").set_flags(dpp::m_ephemeral));
};