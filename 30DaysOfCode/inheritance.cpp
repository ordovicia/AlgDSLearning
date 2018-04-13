class Student : public Person
{
private:
    vector<int> testScores;

public:
    /*
     * Class Constructor
     *
     * Parameters:
     * firstName - A string denoting the Person's first name.
     * lastName - A string denoting the Person's last name.
     * id - An integer denoting the Person's ID number.
     * scores - An array of integers denoting the Person's test scores.
     */
    Student(string firstName, string lastName, int id, vector<int> scores)
        : Person(firstName, lastName, id), testScores(scores) {}

    /*
     * Function Name: calculate
     * Return: A character denoting the grade.
     */
    char calculate() const
    {
        int ave = 0;
        for (int score : testScores)
            ave += score;
        ave /= testScores.size();
        if (ave >= 90)
            return 'O';
        else if (ave >= 80)
            return 'E';
        else if (ave >= 70)
            return 'A';
        else if (ave >= 55)
            return 'P';
        else if (ave >= 40)
            return 'D';
        else
            return 'T';
    }
};
