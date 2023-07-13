#include <dpp/dpp.h>
#include <dpp/presence.h>
#include <mutex>
#include <thread>

#include "commands.hpp"
#include "dynmap.hpp"

extern std::string BOT_TOKEN;
extern std::vector<Command> commandList;
extern Dynmap cavernDynmap;
extern TownyData townyData;
extern std::mutex townyDataMutex;
extern PlayerData playerData;
extern std::mutex playerDataMutex;

int main(int argc, const char* argv[]) {
    dpp::cluster bot(BOT_TOKEN);
    bot.on_log(dpp::utility::cout_logger());

    bot.on_slashcommand([&bot](const dpp::slashcommand_t &event) {
        for(Command& command : commandList) {
            if(event.command.get_command_name() == command.name) {
                std::cout << "Launching command " << command.name << " from /" << event.command.get_command_name() << std::endl;
                command.func(bot, event);
                return;
            }
        }
        event.reply("There is no command named \"" + event.command.get_command_name() + "\" dumbass");
    });
    bot.on_ready([&bot](const dpp::ready_t & event) {
        std::vector<dpp::slashcommand> commands;
        for(Command& command : commandList) {
            dpp::slashcommand cmd(command.name, command.description, bot.me.id);
            for(CommandOption commandOption : command.options) {
                dpp::command_option cmdOpt(dpp::command_option_type(commandOption.optionType), commandOption.name, commandOption.description, commandOption.isRequired);
                for(CommandOptionChoice commandOptionChoice : commandOption.choices)
                    cmdOpt.add_choice(dpp::command_option_choice(commandOptionChoice.n, commandOptionChoice.v));
                cmd.add_option(cmdOpt);
            }
            commands.push_back(cmd);
        }
        bot.global_bulk_command_create(commands);
        bot.set_presence(dpp::presence(dpp::ps_idle, dpp::at_watching, "Cavern Dynmap"));
    });
    bot.start(dpp::st_wait);
    return 0;
}