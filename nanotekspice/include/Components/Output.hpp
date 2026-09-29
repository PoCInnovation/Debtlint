/*
** EPITECH PROJECT, 2026
** output
** File description:
** 
*/

#ifndef OUTPUT_HPP
    #define OUTPUT_HPP
    #include <map>
    #include "AComponent.hpp"

class Output : public nts::AComponent
{
public:
    Output() : _value(nts::Undefined) {
        _pins = {{1, nts::Pin(*this, 1)}};
    }
    void simulate(std::size_t tick) final { (void)tick; }
    nts::Tristate compute(std::size_t pin) final {
        _value = _pins.at(pin).getLinkComp().compute(_pins.at(pin).getLinkIndex());
        return _value;
    }
    nts::Tristate getValue() { return _value; }
    void setValue(nts::Tristate value) { (void)value; }

private:
    nts::Tristate _value;
};

#endif
