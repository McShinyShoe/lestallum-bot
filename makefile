# Compiler Used
CC= g++
# Flags used when compiling
LIB= -ldpp -lpthread
# Output
OUT= lestallum-bot
# Using Standard
STD= c++2a

all: lestallum-bot

lestallum-bot: mkdir envivars.o dynmap.o commands.o lestallum-bot.o 
	$(CC) -std=$(STD) build/*.o $(LIB) -o $(OUT)

envivars.o: mkdir
	$(CC) -std=$(STD) -c -o build/envivars.o src/envivars.cpp

dynmap.o: mkdir
	$(CC) -std=$(STD) -c -o build/dynmap.o src/dynmap.cpp

commands.o: mkdir
	$(CC) -std=$(STD) -c -o build/commands.o src/commands.cpp

lestallum-bot.o: mkdir
	$(CC) -std=$(STD) -c -o build/lestallum-bot.o src/lestallum-bot.cpp

mkdir:
	mkdir -p build

clean:
	rm -rf build

container:
	docker build ./ -t shinyshoe/lestallum-bot