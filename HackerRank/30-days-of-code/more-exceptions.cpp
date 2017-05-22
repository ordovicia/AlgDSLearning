struct Calculator {
    static int power(int n, int p)
    {
        if (n < 0 || p < 0)
            throw std::out_of_range{"n and p should be non-negative"};

        return p == 0 ? 1 : n * power(n, p - 1);
    }
};
