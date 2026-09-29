/*
** EPITECH PROJECT, 2026
** gate
** File description:
** teckspice
*/

#ifndef GATES_HPP
    #define GATES_HPP
    #include "IComponent.hpp"

namespace gate {
    nts::Tristate andGate(nts::Tristate i1, nts::Tristate i2);
    nts::Tristate orGate(nts::Tristate i1, nts::Tristate i2);
    nts::Tristate xorGate(nts::Tristate i1, nts::Tristate i2);
    nts::Tristate notGate(nts::Tristate i1);
    nts::Tristate nandGate(nts::Tristate i1, nts::Tristate i2);
    nts::Tristate norGate(nts::Tristate i1, nts::Tristate i2);
    nts::Tristate xorGateThreeInputs(nts::Tristate i1, nts::Tristate i2, nts::Tristate i3);
    nts::Tristate majorityGate(nts::Tristate i1, nts::Tristate i2, nts::Tristate i3);
}

#endif