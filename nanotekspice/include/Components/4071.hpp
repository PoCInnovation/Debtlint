/*
** EPITECH PROJECT, 2026
** 4071
** File description:
** 
*/

#ifndef COMPONENT4071_HPP
    #define COMPONENT4071_HPP
    #include "AComponent.hpp"
    #include <map>
    #include "Pin.hpp"
    #include "Gates.hpp"

class Component4071 : public nts::AComponent
{
public:
    Component4071() {
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
                {13, nts::Pin(*this, 13)}};
    }
    void simulate(std::size_t tick) final { (void)tick; }
    void setValue(nts::Tristate value) { (void)value; }
    nts::Tristate getValue() { return nts::Undefined; }
    nts::Tristate compute(std::size_t pin) final
    {
        if (pin == 3) {
            return gate::orGate(
            _pins.at(1).getLinkComp().compute(_pins.at(1).getLinkIndex()),
            _pins.at(2).getLinkComp().compute(_pins.at(2).getLinkIndex()));
        }
        if (pin == 4) {
            return gate::orGate(
                _pins.at(5).getLinkComp().compute(_pins.at(5).getLinkIndex()),
                _pins.at(6).getLinkComp().compute(_pins.at(6).getLinkIndex()));
        }
        if (pin == 10) {
            return gate::orGate(
                _pins.at(8).getLinkComp().compute(_pins.at(8).getLinkIndex()),
                _pins.at(9).getLinkComp().compute(_pins.at(9).getLinkIndex()));
        }
        if (pin == 11) {
            return gate::orGate(
                _pins.at(12).getLinkComp().compute(_pins.at(12).getLinkIndex()),
                _pins.at(13).getLinkComp().compute(_pins.at(13).getLinkIndex()));
        }
        return nts::Undefined;
    }
};

#endif
