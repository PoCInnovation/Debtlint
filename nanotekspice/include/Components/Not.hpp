/*
** EPITECH PROJECT, 2026
** not
** File description:
** 
*/

#ifndef NOT_HPP
    #define NOT_HPP
    #include "AComponent.hpp"
    #include <map>
    #include "Pin.hpp"
    #include "Gates.hpp"

class Not : public nts::AComponent
{
public:
    Not() {
        _pins = {{1, nts::Pin(*this, 1)},
                {2, nts::Pin(*this, 2)}};
    }
    void simulate(std::size_t tick) final { (void)tick; }
    void setValue(nts::Tristate value) { (void)value; }
    nts::Tristate getValue() { return nts::Undefined; }
    nts::Tristate compute(std::size_t pin) final
    {
        if (pin == 2) {
            return gate::notGate(
            _pins.at(1).getLinkComp().compute(_pins.at(1).getLinkIndex()));
        }
        return nts::Undefined;
    }
};

#endif
