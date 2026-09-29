/*
** EPITECH PROJECT, 2026
** shell
** File description:
** teckspice
*/

#include "Shell.hpp"

nts::Shell::Shell(nts::Circuit &circuit) : _circuit(circuit)
{
    _commands["exit"] = std::bind(&Shell::exit, this);
    _commands["display"] = std::bind(&Shell::display, this);
    _commands["simulate"] = std::bind(&Shell::simulate, this);
    _commands["loop"] = std::bind(&Shell::loop, this);
};

bool nts::Shell::readInput()
{
    std::cout << "> ";
    return (std::getline(std::cin, _currLine)) ? true : false;
}

void nts::Shell::exit()
{
    _runLoop = false;
}
void nts::Shell::loop()
{
    std::vector tokenLine = ParserHelper::tokenizeLine(_currLine);

    if (tokenLine.size() == 2){
        try {
            std::size_t it = stoi(tokenLine[1]);
            _circuit.loop(it);
        } catch (const std::exception &e) {
            std::cerr << "loop doesnt handle non integer iterator" << std::endl;
            return;
        }
    } else {
        _circuit.loop();
    }
}

void nts::Shell::exitLoop()
{
    _runLoop = false;
}

void nts::Shell::display()
{
    _circuit.display();
}

void nts::Shell::simulate()
{
    _circuit.simulate();
}

void nts::Shell::setInput(std::string &name, std::string &valueStr)
{
    try {
        if (valueStr == "U"){
            _circuit.findComponents(name).setValue(nts::Undefined);
            return;
        }
        int value = std::stoi(valueStr);
        if (value == 1 || value == 0){
            _circuit.findComponents(name).setValue(static_cast<nts::Tristate>(value));
        }
    } catch (const std::invalid_argument &e) {
        std::cout << "not a number" << std::endl;
        return;
    }
}

void nts::Shell::shellLoop()
{
    while (_runLoop && readInput()) {
        std::vector tokenLine = ParserHelper::tokenizeLine(_currLine);
        if (_commands.count(tokenLine[0])){
            _commands[tokenLine[0]]();
            continue;
        }
        tokenLine = ParserHelper::tokenizeByChar(_currLine, '=');
        if (tokenLine.size() == 2){
            setInput(tokenLine[0], tokenLine[1]);
            continue;
        }
        std::cout << "Unknow command" << std::endl;
    }
    
}