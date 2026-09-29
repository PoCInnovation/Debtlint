/*
** EPITECH PROJECT, 2026
** 4008
** File description:
** 
*/

#ifndef COMPONENT4008_HPP
    #define COMPONENT4008_HPP
    #include "AComponent.hpp"
    #include <map>
    #include "Pin.hpp"
    #include "Gates.hpp"

class Component4008 : public nts::AComponent
{
public:
    Component4008() {
        _pins = {{1, nts::Pin(*this, 1)},
                {2, nts::Pin(*this, 2)},
                {3, nts::Pin(*this, 3)}, // Input 
                {4, nts::Pin(*this, 4)}, // Input B2
                {5, nts::Pin(*this, 5)}, // Input A2
                {6, nts::Pin(*this, 6)}, // Input B1
                {7, nts::Pin(*this, 7)}, // Input A1
                {8, nts::Pin(*this, 8)}, // VSS
                {9, nts::Pin(*this, 9)}, // Carry in
                {10, nts::Pin(*this, 10)}, // Output C1
                {11, nts::Pin(*this, 11)}, // Output C2
                {12, nts::Pin(*this, 12)}, // Output C3
                {13, nts::Pin(*this, 13)}, // Output C4
                {14, nts::Pin(*this, 14)}, // Carry out
                {15, nts::Pin(*this, 15)}, // Inout B4
                {16, nts::Pin(*this, 16)}}; // VDD
    }
    void simulate(std::size_t tick) final { (void)tick; }
    void setValue(nts::Tristate value) { (void)value; }
    nts::Tristate getValue() { return nts::Undefined; }

    nts::Tristate sumValue(size_t pin_a, size_t pin_b, nts::Tristate &carry)
    {
        nts::Tristate valueA = _pins.at(pin_a).getLinkComp().compute(_pins.at(pin_a).getLinkIndex());
        nts::Tristate valueB =_pins.at(pin_b).getLinkComp().compute(_pins.at(pin_b).getLinkIndex());
        nts::Tristate sumValue;

        sumValue = gate::xorGateThreeInputs(valueA, valueB, carry);
        carry = gate::majorityGate(valueA, valueB, carry);
        return sumValue;
    }

    void setSum(nts::Tristate &carry)
    {
        sum1 = sumValue(7, 6, carry);
        sum2 = sumValue(5, 4, carry);
        sum3 = sumValue(3, 2, carry);
        sum4 = sumValue(1, 15, carry);
    }

    nts::Tristate compute(std::size_t pin) final
    {
        nts::Tristate carry = _pins.at(9).getLinkComp().compute(_pins.at(9).getLinkIndex());
        setSum(carry);
    
        if (pin == 10)
            return sum1;
        if (pin == 11)
            return sum2;
        if (pin == 12)
            return sum3;
        if (pin == 13)
            return sum4;
        if (pin == 14)
            return carry;
        return nts::Undefined;
    }
private:
    nts::Tristate sum1;
    nts::Tristate sum2;
    nts::Tristate sum3;
    nts::Tristate sum4;
};

#endif
