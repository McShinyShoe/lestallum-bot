#include <cstdlib>
#include <string>

#define ENVIVAR(x) std::string x(std::getenv(#x) != NULL ? std::getenv(#x) : "")

ENVIVAR(BOT_TOKEN);
ENVIVAR(VERIFY_ROLE_ID);
ENVIVAR(VERIFY_CHANNEL_ID);