#ifndef CALCULATOR__DATATYPES_H
#define CALCULATOR__DATATYPES_H

struct SimpleFraction
{
    SimpleFraction(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen); // normalises
    
    SimpleFraction(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen); // normalises
    
    SimpleFraction()
    {
        numerator = 0;
        denominator = 1;
    }
    
    SafeInt<int64_t> numerator;
    SafeInt<int64_t> denominator;
    
    explicit operator SafeInt<int64_t>() const
    { return numerator / denominator; }
    
    explicit operator double() const
    { return double(numerator) / double(denominator); }
    
    bool operator==(const SimpleFraction& Other) const
    { return numerator == Other.numerator && denominator == Other.denominator; }
    
    SimpleFraction operator+(SimpleFraction) const;
    
    SimpleFraction operator-(SimpleFraction) const;
    
    SimpleFraction operator*(SimpleFraction) const;
    
    void normalise();
};

struct Fraction
{
    Fraction(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen); // normalises
    
    Fraction(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen); // does not normalise
    
    Fraction()
    {
        integer = 0;
        numerator = 0;
        denominator = 1;
    }
    
    SafeInt<int64_t> integer;
    SafeInt<int64_t> numerator;
    SafeInt<int64_t> denominator;
    
    explicit operator SafeInt<int64_t>() const
    { return integer; }
    
    explicit operator double() const
    { return double(integer) + double(numerator) / double(denominator); }
    
    explicit operator SimpleFraction() const
    { return  SimpleFraction(numerator + integer * denominator, denominator); }
    
    Fraction operator+(Fraction) const;
    
    Fraction operator-(Fraction) const;
    
    Fraction operator*(Fraction) const;
    
    Fraction operator*(SafeInt<int64_t>) const;
    
    void normalise();
};

struct Power
{
    Fraction multiplicand;
    SimpleFraction base;
    SimpleFraction exponent;
    Power(Fraction GivenMultiplicand, SimpleFraction GivenBase, SimpleFraction GivenExponent);
    Power(Fraction GivenMultiplicand, SafeInt<int64_t> GivenBase, SimpleFraction GivenExponent);
    Power();
    
    explicit operator double() const;
};


#endif //CALCULATOR__DATATYPES_H
