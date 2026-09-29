/*
** EPITECH PROJECT, 2026
** Circuit
** File description:
** 
*/

#ifndef CIRCUIT_HPP
    #define CIRCUIT_HPP
    #include <vector>
    #include <map>
    #include <memory>
    #include "IComponent.hpp"
    #include "ParserHelper.hpp"
    #include "ComponentFactory.hpp"

namespace nts {
    class Circuit;
}

class nts::Circuit
{
public:
    class CircuitException : public std::exception {
        public:
            CircuitException(std::string message) : _message(message) {};
            std::string _message;
            const char *what() const noexcept override { return _message.c_str(); }
    };
    Circuit(const char **argv);
    nts::IComponent &findComponents(std::string name);
    void addComponent(const std::string &type, const std::string &name);

    void simulate();
    void display();
    void loop();
    void loop(std::size_t loopIterate);

private:
    size_t tick;
    std::map<std::string, std::unique_ptr<nts::IComponent>> _inputs;
    std::map<std::string, std::unique_ptr<nts::IComponent>> _components;
    std::map<std::string, std::unique_ptr<nts::IComponent>> _outputs;
    void init_circuit(std::vector<std::vector<std::string>> vec_file);
    void link_components(std::vector<std::string> compOne, std::vector<std::string> compTwo);
    ComponentFactory _factory;
};

#endif
