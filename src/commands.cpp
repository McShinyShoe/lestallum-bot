#include "commands.hpp"
#include "commands/verivy.hpp"


std::vector<Command> commandList {
    { verify, "verify", "Verify yourself!", {
        {dpp::co_string, "playername", "Enter your name in the server", true, {}}
    }}
};