/*
** EPITECH PROJECT, 2026
** gate
** File description:
** teckspice
*/

#include "Gates.hpp"

nts::Tristate gate::andGate(nts::Tristate i1, nts::Tristate i2)
{
    if (i1 == nts::Tristate::True && i2 == nts::Tristate::True)
        return nts::Tristate::True;
    if (i1 == nts::Tristate::False || i2 == nts::Tristate::False)
        return nts::Tristate::False;
    return nts::Tristate::Undefined;
}

nts::Tristate gate::orGate(nts::Tristate i1, nts::Tristate i2)
{
    if (i1 == nts::Tristate::True || i2 == nts::Tristate::True)
    	return nts::Tristate::True;
    if (i1 == nts::Tristate::Undefined || i2 == nts::Tristate::Undefined)
    	return nts::Tristate::Undefined;
    return nts::Tristate::False;
}

nts::Tristate gate::xorGate(nts::Tristate i1, nts::Tristate i2)
{
    if (i1 == nts::Tristate::Undefined || i2 == nts::Tristate::Undefined)
        return nts::Tristate::Undefined;
    if (i1 != i2)
        return nts::Tristate::True;
    return nts::Tristate::False;
}

nts::Tristate gate::notGate(nts::Tristate i1)
{
    if (i1 == nts::Tristate::True)
        return nts::Tristate::False;
    if (i1 == nts::Tristate::False)
        return nts::Tristate::True;
    return nts::Tristate::Undefined;
}

nts::Tristate gate::nandGate(nts::Tristate i1, nts::Tristate i2)
{
    return notGate(andGate(i1, i2));
}

nts::Tristate gate::norGate(nts::Tristate i1, nts::Tristate i2)
{
    return notGate(orGate(i1, i2));  
}

nts::Tristate gate::xorGateThreeInputs(nts::Tristate i1, nts::Tristate i2, nts::Tristate i3)
{
    return xorGate(gate::xorGate(i1, i2), i3);
}

nts::Tristate gate::majorityGate(nts::Tristate i1, nts::Tristate i2, nts::Tristate i3)
{
    return orGate(orGate(andGate(i1, i2), andGate(i1, i3)), andGate(i2, i3));
}
