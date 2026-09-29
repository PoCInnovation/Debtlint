/*
** EPITECH PROJECT, 2026
** 4069
** File description:
** 
*/

#ifndef COMPONENT4069_HPP
    #define COMPONENT4069_HPP
    #include "AComponent.hpp"
    #include <map>
    #include "Pin.hpp"
    #include "Gates.hpp"

class Component4069 : public nts::AComponent
{
public:
    Component4069() {
        _pins = {{1, nts::Pin(*this, 1)},
                {2, nts::Pin(*this, 2)},
                {3, nts::Pin(*this, 3)},
                {4, nts::Pin(*this, 4)},
                {5, nts::Pin(*this, 5)},
                {6, nts::Pin(*this, 6)},
                {7, nts::Pin(*this, 7)},
                {8, nts::Pin(*this, 8)},
                {9, nts::Pin(*this, 9)},
                {10, nts::Pin(*this, 10)},
                {11, nts::Pin(*this, 11)},
                {12, nts::Pin(*this, 12)},
                {13, nts::Pin(*this, 13)},
                {14, nts::Pin(*this, 14)}};
    }
    void simulate(std::size_t tick) final { (void)tick; }
    void setValue(nts::Tristate value) { (void)value; }
    nts::Tristate getValue() { return nts::Undefined; }
    nts::Tristate compute(std::size_t pin) final
    {
        if (pin == 2)
            return gate::notGate(
            _pins.at(1).getLinkComp().compute(_pins.at(1).getLinkIndex()));
        if (pin == 4)
            return gate::notGate(
                _pins.at(3).getLinkComp().compute(_pins.at(3).getLinkIndex()));
        if (pin == 6)
            return gate::notGate(
                _pins.at(5).getLinkComp().compute(_pins.at(5).getLinkIndex()));
        if (pin == 8)
            return gate::notGate(
                _pins.at(9).getLinkComp().compute(_pins.at(9).getLinkIndex()));
        if (pin == 10)
            return gate::notGate(
                _pins.at(11).getLinkComp().compute(_pins.at(11).getLinkIndex()));
        if (pin == 12)
            return gate::notGate(
                _pins.at(13).getLinkComp().compute(_pins.at(13).getLinkIndex()));
        return nts::Undefined;
    }
};

#endif
