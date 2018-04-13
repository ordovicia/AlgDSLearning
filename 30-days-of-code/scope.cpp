explicit Difference(vector<int> elements)
    : elements(std::move(elements)) {}

void computeDifference()
{
    maximumDifference = 0;
    for (int i = 0; i < elements.size() - 1; i++) {
        for (int j = i + 1; j < elements.size(); j++) {
            int diff = std::abs(elements[i] - elements[j]);
            if (diff > maximumDifference)
                maximumDifference = diff;
        }
    }
}
