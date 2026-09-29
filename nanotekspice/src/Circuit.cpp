/*
** EPITECH PROJECT, 2026
** circuit
** File description:
** 
*/

#include "Circuit.hpp"
#include <fstream>
#include <string>
#include <iostream>
#include <vector>
#include <algorithm>
#include <signal.h>
#include "Components/Input.hpp"
#include "Components/Output.hpp"
#include <termios.h>

nts::Circuit::Circuit(const char **argv)
{
    std::vector<std::vector<std::string>> vec_file = ParserHelper::tokenizeFile(argv[1], '#');

    if (vec_file.empty())
            throw CircuitException("File doesn't exist.");
    init_circuit(vec_file);
}

void nts::Circuit::init_circuit(std::vector<std::vector<std::string>> vec_file)
{
    enum Mode { Chipsets, Links, Default };
    Mode mode = Mode::Default;

    for (const auto &elem : vec_file) {
        if (elem[0] == ".chipsets:") {
            mode = Chipsets;
            continue;
        }
        if (elem[0] == ".links:") {
            mode = Links;
            continue;
        }
        if (mode == Chipsets && (elem.size() == 2))
            addComponent(elem[0], elem[1]);
        else if (mode == Links && (elem.size() == 2)) {
            std::vector<std::string> compOne = ParserHelper::tokenizeByChar(elem[0], ':');
            std::vector<std::string> compTwo = ParserHelper::tokenizeByChar(elem[1], ':');
            link_components(compOne, compTwo);
        }
        else
            throw CircuitException("Invalid File");
    }
}

bool isNumber(const std::string &str)
{
    const char *begin = str.data();
    char *end = nullptr;
    errno = 0;
    std::strtoul(begin, &end, 10);

    if (begin == end)
        return false;
    if (errno == ERANGE)
        return false;
    return *end == '\0';
}

void nts::Circuit::link_components(std::vector<std::string> compOne, std::vector<std::string> compTwo)
{
    if (compOne.size() != 2 || compTwo.size() != 2)
        throw CircuitException("Invalid Format");
    if (!isNumber(compOne[1]) || !isNumber(compTwo[1]))
        throw CircuitException("Invalid Pin");

    size_t pin_1 = std::strtoul(compOne[1].c_str(), nullptr, 10);
    size_t pin_2 = std::strtoul(compTwo[1].c_str(), nullptr, 10);
    nts::IComponent &component_one = findComponents(compOne[0]);
    nts::IComponent &component_two = findComponents(compTwo[0]);
    component_one.setLink(pin_1, component_two, pin_2);
    component_two.setLink(pin_2, component_one, pin_1);
}

volatile sig_atomic_t GSIGNALSTATUS;

void signalHandler(int signal)
{
    GSIGNALSTATUS = signal;
}

static void disableEchoctl(termios &oldTerm)
{
    termios newTerm;
    tcgetattr(STDIN_FILENO, &oldTerm);
    newTerm = oldTerm;

    newTerm.c_lflag &= ~ECHOCTL;
    tcsetattr(STDIN_FILENO, TCSANOW, &newTerm);
}

void nts::Circuit::loop()
{
    termios terminal;

    disableEchoctl(terminal);
    signal(SIGINT, signalHandler);
    while (GSIGNALSTATUS != SIGINT) {
        simulate();
    }
    tcsetattr(STDIN_FILENO, TCSANOW, &terminal);
    display();
    GSIGNALSTATUS = 0;
    signal(SIGINT, SIG_DFL);
    return;
}

void nts::Circuit::loop(std::size_t loopIterate)
{
    termios terminal;
    std::size_t iterator = 0;

    disableEchoctl(terminal);
    signal(SIGINT, signalHandler);
    while (iterator < loopIterate) {
        simulate();
        iterator++;
    }
    tcsetattr(STDIN_FILENO, TCSANOW, &terminal);
    display();
    GSIGNALSTATUS = 0;
    signal(SIGINT, SIG_DFL);
    return;
}

nts::IComponent &nts::Circuit::findComponents(std::string name)
{
    auto it = _inputs.find(name);

    if (it != _inputs.end())
        return *it->second;
    it = _outputs.find(name);
    if (it != _outputs.end())
        return *it->second;
    it = _components.find(name);
    if (it != _components.end())
        return *it->second;
    throw CircuitException("Component name is unknow");
}

void nts::Circuit::addComponent(const std::string &type, const std::string &name)
{
    if (_inputs.contains(name) || _outputs.contains(name) || _components.contains(name))
        throw CircuitException("Several components share the same name.");
    auto comp = _factory.createComponent(type);
    if (!comp)
        throw CircuitException("Component type is unknown.");
    if (type == "input" || type == "clock")
        _inputs[name] = std::move(comp);
    if (type == "output")
        _outputs[name] = std::move(comp);
    else
        _components[name] = std::move(comp);
}

void nts::Circuit::simulate()
{
    tick++;
    for (auto &[name, comp]: _inputs){
        comp.get()->simulate(tick);
    }
    for (auto &[name, comp]: _outputs){
        comp.get()->simulate(tick);
    }
    for (auto &[name, comp]: _components){
        if (comp.get())
            comp.get()->simulate(tick);
    }
}

void nts::Circuit::display()
{
    std::cout << "tick: " << tick << std::endl;
    for (auto const &output : _outputs) {
        output.second.get()->compute(1);
    }
    std::cout << "input(s):" << std::endl;
    for (auto const &[name, comp]: _inputs) {
        std::cout << "  " << name << ": " << comp->getValue() << std::endl;
    }
    std::cout << "output(s):" << std::endl;
    for (auto const &[name, comp]: _outputs) {
        std::cout << "  " << name << ": " << comp->getValue() << std::endl;
    }
}

std::ostream &operator<<(std::ostream &stream, const nts::Tristate value)
{
    if (value == nts::Undefined){
        std::cout << 'U';
    } else {
        std::cout << static_cast<bool>(value);
    }
    return stream;
}
