#include "commands.hpp"
#include "commands/verivy.hpp"
#include "commands/whereis.hpp"

std::vector<Command> commandList {
    { verify, "verify", "Verify yourself!", {
        {dpp::co_string, "playername", "Enter your name in the server", true, {}}
    }},
    { whereis, "whereis", "Find a player location in the server", {
        {dpp::co_string, "playername", "Enter the searched player name", true, {}}
    }}
};