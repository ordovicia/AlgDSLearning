struct Calculator : public AdvancedArithmetic {
    int divisorSum(int n)
    {
        int sum = 0;
        for (int i = 1, end = static_cast<int>(floor(sqrt(n))); i <= end; i++) {
            if (n % i == 0)
                sum += i + (i * i == n ? 0 : n / i);
        }

        return sum;
    }
};
