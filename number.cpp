#include "number.h"
#include "myMath.h"

fraction::fraction(int64_t GivenInt, int64_t GivenNum, int64_t GivenDen)    // Not unused
{
    integer = GivenInt;
    numerator = GivenNum;
    denominator = GivenDen;
}

fraction::fraction(int64_t GivenNum, int64_t GivenDen)  // Not unused
{
    integer = 0;
    numerator = GivenNum;
    denominator = GivenDen;
    normalise();
}

void fraction::normalise()
{
    
    if (denominator < 0)
    {
        numerator *= -1;
        denominator *= -1;
    }
    
    if (denominator == 1)
    {
        integer += numerator;
        numerator = 0;
        return;
    }
    
    if (numerator >= denominator)
    {
        integer += numerator / denominator;
        numerator -= (numerator / denominator) * denominator;
    }
    
    if (numerator == 0)
    {
        denominator = 1;
        return;
    }
    
    const int64_t gcd = std::gcd((int64_t) numerator, (int64_t) denominator);
    
    if (gcd != 1)
    {
        numerator = numerator / gcd;
        denominator = denominator / gcd;
    }
    
}

std::ostream &operator<<(std::ostream &Strm, const number &N1)
{
    if (N1.type == number::INTEGER_TYPE)
    {
        Strm << N1.integer;
    }
    else if (N1.type == number::FRACTION_TYPE && N1.fraction.denominator == 1)
    {
        Strm << N1.fraction.integer;
    }
    else if (N1.type == number::FRACTION_TYPE)
    {
        Strm << N1.fraction.integer << "+" << N1.fraction.numerator << "/" << N1.fraction.denominator;
    }
    return Strm;
}

number::number(int64_t GivenInt, int64_t GivenNum, int64_t GivenDen)
{
    type = FRACTION_TYPE;
    fraction = {GivenInt, GivenNum, GivenDen};
}

number::number(int64_t GivenNum, int64_t GivenDen)
{
    type = FRACTION_TYPE;
    fraction = {GivenNum, GivenDen};
    if (fraction.numerator == 0)
    {
        integer = fraction.integer;
        type = INTEGER_TYPE;
    }
    
}

number::number(int64_t GivenInt)
{
    type = INTEGER_TYPE;
    integer = GivenInt;
}

number::number(const std::string_view &Number)  // Assumes there is only one decimal point
{
    const std::size_t offset = Number.find('.');    // TODO: Test for ','
    if (offset != std::string::npos)
    {
        type = FRACTION_TYPE;
        fraction.integer = std::stoll(Number.substr(0, offset).data());
        fraction.numerator = std::stoll(Number.substr(offset + 1).data());
        if (fraction.integer < 0)
        {
            fraction.numerator = -fraction.numerator;
        }
        
        fraction.denominator = powll(10LL, Number.length() - (offset + 1));
        
        fraction.normalise();
    }
    else
    {
        type = INTEGER_TYPE;
        integer = std::stoll(Number.data());
    }
}

number number::operator+(const number N1) const
{
    number result = number(0, 0, 1);
    
    if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
    {
        result = number(integer + N1.integer);
    }
    else if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
    {
        result = number(fraction.integer + N1.integer, fraction.numerator, fraction.denominator);
    }
    else if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
    {
        result = number(integer + N1.fraction.integer, N1.fraction.numerator, N1.fraction.denominator);
    }
    else if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
    {
        result.fraction.integer = fraction.integer + N1.fraction.integer;
        result.fraction.numerator =
            N1.fraction.denominator * fraction.numerator + N1.fraction.numerator * fraction.denominator;
        result.fraction.denominator = N1.fraction.denominator * fraction.denominator;
        
        result.fraction.normalise();
        if (result.fraction.numerator == 0)
        {
            result.integer = result.fraction.integer;
            result.type = INTEGER_TYPE;
        }
    }
    
    return result;
}

number number::operator-(const number N1) const
{
    number result = number(0);
    
    if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
    {
        result = number(integer - N1.integer);
    }
    else if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
    {
        result = number(fraction.integer - N1.integer, fraction.numerator, fraction.denominator);
    }
    else if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
    {
        result = number(integer - N1.fraction.integer, -N1.fraction.numerator, N1.fraction.denominator);
    }
    else if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
    {
        result = number(0, 0, 1);
        result.fraction.integer = fraction.integer - N1.fraction.integer;
        result.fraction.numerator =
            N1.fraction.denominator * fraction.numerator - N1.fraction.numerator * fraction.denominator;
        result.fraction.denominator = N1.fraction.denominator * fraction.denominator;
        
        result.fraction.normalise();
        if (result.fraction.numerator == 0)
        {
            result.integer = result.fraction.integer;
            result.type = INTEGER_TYPE;
        }
    }
    
    return result;
}

number number::operator*(const number N1) const
{
    number result = number(0, 0, 1);
    
    if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
    {
        result = number(integer * N1.integer);
    }
    else if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
    {
        result.fraction.integer = fraction.integer * N1.integer;
        result.fraction.numerator = N1.integer * fraction.numerator;
        result.fraction.denominator = fraction.denominator;
        
        result.fraction.normalise();
        if (result.fraction.numerator == 0)
        {
            result.integer = result.fraction.integer;
            result.type = INTEGER_TYPE;
        }
    }
    else if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
    {
        result.fraction.integer = N1.fraction.integer * integer;
        result.fraction.numerator = integer * N1.fraction.numerator;
        result.fraction.denominator = N1.fraction.denominator;
        
        result.fraction.normalise();
        if (result.fraction.numerator == 0)
        {
            result.integer = result.fraction.integer;
            result.type = INTEGER_TYPE;
        }
    }
    else if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
    {
        result.fraction.integer = N1.fraction.integer * fraction.integer;
        result.fraction.numerator = N1.fraction.numerator * fraction.numerator
            + N1.fraction.denominator * N1.fraction.integer * fraction.numerator +
            fraction.denominator * fraction.integer * N1.fraction.numerator;
        result.fraction.denominator = N1.fraction.denominator * fraction.denominator;
        
        result.fraction.normalise();
        if (result.fraction.numerator == 0)
        {
            result.integer = result.fraction.integer;
            result.type = INTEGER_TYPE;
        }
    }
    
    return result;
}

number number::operator/(const number N1) const
{
    number result = number(0, 0, 1);
    
    if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
    {
        result = number(integer, N1.integer);
        if (result.fraction.numerator == 0)
        {
            result.integer = result.fraction.integer;
            result.type = INTEGER_TYPE;
        }
    }
    else if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
    {
        result.fraction.numerator = fraction.numerator + N1.integer * fraction.denominator;
        result.fraction.denominator = N1.integer * fraction.denominator;
        
        result.fraction.normalise();
        if (result.fraction.numerator == 0)
        {
            result.integer = result.fraction.integer;
            result.type = INTEGER_TYPE;
        }
    }
    else if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
    {
        result.fraction.numerator = integer * N1.fraction.denominator;
        result.fraction.denominator = N1.fraction.numerator + integer * N1.fraction.denominator;
        
        result.fraction.normalise();
        if (result.fraction.numerator == 0)
        {
            result.integer = result.fraction.integer;
            result.type = INTEGER_TYPE;
        }
    }
    else if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
    {
        result.fraction.numerator = fraction.integer * fraction.denominator * N1.fraction.denominator
            + N1.fraction.denominator * fraction.numerator;
        result.fraction.denominator = N1.fraction.denominator * fraction.denominator * N1.fraction.integer
            + fraction.denominator * N1.fraction.numerator;
        
        result.fraction.normalise();
        if (result.fraction.numerator == 0)
        {
            result.integer = result.fraction.integer;
            result.type = INTEGER_TYPE;
        }
    }
    
    return result;
}

number::number()
{
    type = INTEGER_TYPE;
    integer = 0;
}
