/*
** EPITECH PROJECT, 2026
** shell
** File description:
** teckspice
*/

#ifndef SHELL_HPP
    #define SHELL_HPP
    #include "Circuit.hpp"
    #include <functional>

namespace nts {
    class Shell
    {
    public:
        Shell(nts::Circuit &circuit);
        ~Shell() = default;
        bool readInput();
        void exit();
        void loop();
        void exitLoop();
        void display();
        void simulate();
        void setInput(std::string &name, std::string &valueStr);
        void shellLoop();

    private:
        nts::Circuit &_circuit;
        std::string _currLine;
        bool _runLoop = true;
        std::map<std::string, std::function<void()>> _commands;

    };
}

#endif
