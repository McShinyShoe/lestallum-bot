#include <vector>
#include <dpp/dpp.h>
#include <cstdint>

struct CommandOptionChoice {
    std::string n;
    dpp::command_value v;
};

struct CommandOption {
    uint8_t optionType;
    std::string name;
    std::string description;
    bool isRequired;
    std::vector<CommandOptionChoice> choices;
};

struct Command {
    void (*func)(dpp::cluster &, const dpp::slashcommand_t &);
    std::string name;
    std::string description;
    std::vector<CommandOption> options;
};