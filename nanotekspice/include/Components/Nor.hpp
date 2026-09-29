/*
** EPITECH PROJECT, 2026
** nor
** File description:
** 
*/

#ifndef NOR_HPP
    #define NOR_HPP
    #include "AComponent.hpp"
    #include <map>
    #include "Pin.hpp"
    #include "Gates.hpp"

class Nor : public nts::AComponent
{
public:
    Nor() {
        _pins = {{1, nts::Pin(*this, 1)},
                {2, nts::Pin(*this, 2)},
                {3, nts::Pin(*this, 3)}};
    }
    void simulate(std::size_t tick) final { (void)tick; }
    void setValue(nts::Tristate value) { (void)value; }
    nts::Tristate getValue() { return nts::Undefined; }
    nts::Tristate compute(std::size_t pin) final
    {
        if (pin == 3) {
            return gate::norGate(
            _pins.at(1).getLinkComp().compute(_pins.at(1).getLinkIndex()),
            _pins.at(2).getLinkComp().compute(_pins.at(2).getLinkIndex()));
        }
        return nts::Undefined;
    }
};

#endif
